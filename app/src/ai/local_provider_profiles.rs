use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp_core::user_preferences::GetUserPreferences;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};
use warpui_extras::secure_storage::AppContextExt;

use crate::ai::llms::{
    AvailableLLMs, DisableReason, LLMContextWindow, LLMInfo, LLMModelHost, LLMProvider,
    LLMUsageMetadata, ModelsByFeature, RoutingHostConfig,
};

const LOCAL_PROVIDER_PROFILES_KEY: &str = "LocalAIProviderProfiles";
const LOCAL_PROVIDER_SECRET_PREFIX: &str = "LocalAIProviderProfileSecret";
const DEFAULT_PROFILE_ID: &str = "default-openai-compatible";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalAIProviderProfilesEvent {
    ProfilesChanged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAIProviderCompatibility {
    OpenAIChatCompletions,
    OpenAIResponses,
}

impl LocalAIProviderCompatibility {
    pub fn display_name(&self) -> &'static str {
        match self {
            LocalAIProviderCompatibility::OpenAIChatCompletions => "OpenAI chat completions",
            LocalAIProviderCompatibility::OpenAIResponses => "OpenAI responses",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalAIProviderHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalAIProviderProfile {
    pub id: String,
    pub display_name: String,
    pub base_url: String,
    pub model_id: String,
    pub compatibility: LocalAIProviderCompatibility,
    pub headers: Vec<LocalAIProviderHeader>,
    pub api_key_storage_key: String,
}

impl LocalAIProviderProfile {
    fn default_openai_compatible() -> Self {
        Self {
            id: DEFAULT_PROFILE_ID.to_string(),
            display_name: "Local OpenAI-compatible".to_string(),
            base_url: String::new(),
            model_id: String::new(),
            compatibility: LocalAIProviderCompatibility::OpenAIChatCompletions,
            headers: Vec::new(),
            api_key_storage_key: format!("{LOCAL_PROVIDER_SECRET_PREFIX}:{DEFAULT_PROFILE_ID}"),
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.base_url.trim().is_empty() && !self.model_id.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct LocalAIProviderProfilesSnapshot {
    profiles: Vec<LocalAIProviderProfile>,
    active_profile_id: Option<String>,
}

pub struct LocalAIProviderProfiles {
    profiles: Vec<LocalAIProviderProfile>,
    active_profile_id: Option<String>,
}

impl LocalAIProviderProfiles {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        Self::load(ctx).unwrap_or_default()
    }

    pub fn load_from_model_context<M: Entity>(ctx: &mut ModelContext<M>) -> Self {
        Self::load(ctx).unwrap_or_default()
    }

    pub fn default_profile(&self) -> &LocalAIProviderProfile {
        self.profiles
            .first()
            .expect("LocalAIProviderProfiles always has a default profile")
    }

    pub fn active_profile(&self) -> &LocalAIProviderProfile {
        self.active_profile_id
            .as_ref()
            .and_then(|id| self.profiles.iter().find(|profile| &profile.id == id))
            .unwrap_or_else(|| self.default_profile())
    }

    pub fn set_default_profile(
        &mut self,
        display_name: String,
        base_url: String,
        model_id: String,
        compatibility: LocalAIProviderCompatibility,
        ctx: &mut ModelContext<Self>,
    ) {
        let profile = self
            .profiles
            .first_mut()
            .expect("LocalAIProviderProfiles always has a default profile");
        profile.display_name = display_name.trim().to_string();
        profile.base_url = base_url.trim().trim_end_matches('/').to_string();
        profile.model_id = model_id.trim().to_string();
        profile.compatibility = compatibility;
        self.active_profile_id = Some(profile.id.clone());
        self.save(ctx);
    }

    pub fn clear_default_profile(&mut self, ctx: &mut ModelContext<Self>) {
        let old_secret_key = self.default_profile().api_key_storage_key.clone();
        self.profiles = vec![LocalAIProviderProfile::default_openai_compatible()];
        self.active_profile_id = Some(DEFAULT_PROFILE_ID.to_string());
        if let Err(err) = ctx.secure_storage().remove_value(&old_secret_key) {
            log::warn!("Failed to remove local AI provider secret: {err:#}");
        }
        self.save(ctx);
    }

    pub fn set_default_api_key(&mut self, api_key: Option<String>, ctx: &mut ModelContext<Self>) {
        let storage_key = self.default_profile().api_key_storage_key.clone();
        match api_key {
            Some(value) if !value.trim().is_empty() => {
                if let Err(err) = ctx.secure_storage().write_value(&storage_key, value.trim()) {
                    log::warn!("Failed to write local AI provider secret: {err:#}");
                }
            }
            _ => {
                if let Err(err) = ctx.secure_storage().remove_value(&storage_key) {
                    log::warn!("Failed to remove local AI provider secret: {err:#}");
                }
            }
        }
        ctx.emit(LocalAIProviderProfilesEvent::ProfilesChanged);
    }

    pub fn has_api_key(&self, profile: &LocalAIProviderProfile, ctx: &AppContext) -> bool {
        ctx.secure_storage()
            .read_value(&profile.api_key_storage_key)
            .is_ok_and(|value| !value.trim().is_empty())
    }

    pub fn default_api_key_for_display(&self, ctx: &AppContext) -> Option<String> {
        let profile = self.default_profile();
        ctx.secure_storage()
            .read_value(&profile.api_key_storage_key)
            .ok()
    }

    pub fn models_by_feature(&self, ctx: &AppContext) -> ModelsByFeature {
        let choices: Vec<LLMInfo> = self
            .profiles
            .iter()
            .map(|profile| self.llm_info_for_profile(profile, ctx))
            .collect();
        let default_id = self
            .active_profile()
            .model_id
            .trim()
            .is_empty()
            .then_some("local-provider-unconfigured")
            .map(str::to_string)
            .unwrap_or_else(|| self.active_profile().model_id.clone())
            .into();
        let available = AvailableLLMs::new(default_id, choices, None)
            .unwrap_or_else(|_| unavailable_local_models());

        ModelsByFeature {
            agent_mode: available.clone(),
            coding: available.clone(),
            cli_agent: Some(available.clone()),
            computer_use: Some(available),
        }
    }

    fn llm_info_for_profile(&self, profile: &LocalAIProviderProfile, ctx: &AppContext) -> LLMInfo {
        let is_configured = profile.is_configured() && self.has_api_key(profile, ctx);
        let model_id = if profile.model_id.trim().is_empty() {
            "local-provider-unconfigured"
        } else {
            profile.model_id.trim()
        };
        let display_name = if profile.display_name.trim().is_empty() {
            model_id.to_string()
        } else {
            profile.display_name.clone()
        };

        LLMInfo {
            display_name,
            base_model_name: model_id.to_string(),
            id: model_id.to_string().into(),
            reasoning_level: None,
            usage_metadata: LLMUsageMetadata {
                request_multiplier: 1,
                credit_multiplier: None,
            },
            description: Some(profile.compatibility.display_name().to_string()),
            disable_reason: (!is_configured).then_some(DisableReason::Unavailable),
            vision_supported: true,
            spec: None,
            provider: LLMProvider::OpenAI,
            host_configs: HashMap::from([(
                LLMModelHost::DirectApi,
                RoutingHostConfig {
                    enabled: true,
                    model_routing_host: LLMModelHost::DirectApi,
                },
            )]),
            discount_percentage: None,
            context_window: LLMContextWindow::default(),
        }
    }

    fn load<M: Entity>(ctx: &mut ModelContext<M>) -> Option<Self> {
        let raw = ctx
            .private_user_preferences()
            .read_value(LOCAL_PROVIDER_PROFILES_KEY)
            .ok()
            .flatten()?;
        let snapshot = serde_json::from_str::<LocalAIProviderProfilesSnapshot>(&raw)
            .map_err(|err| {
                log::warn!("Failed to load local AI provider profiles: {err:#}");
                err
            })
            .ok()?;
        let profiles = if snapshot.profiles.is_empty() {
            vec![LocalAIProviderProfile::default_openai_compatible()]
        } else {
            snapshot.profiles
        };
        let active_profile_id = snapshot
            .active_profile_id
            .or_else(|| profiles.first().map(|profile| profile.id.clone()));
        Some(Self {
            profiles,
            active_profile_id,
        })
    }

    fn save(&self, ctx: &mut ModelContext<Self>) {
        let snapshot = LocalAIProviderProfilesSnapshot {
            profiles: self.profiles.clone(),
            active_profile_id: self.active_profile_id.clone(),
        };
        match serde_json::to_string(&snapshot) {
            Ok(serialized) => {
                if let Err(err) = ctx
                    .private_user_preferences()
                    .write_value(LOCAL_PROVIDER_PROFILES_KEY, serialized)
                {
                    log::warn!("Failed to save local AI provider profiles: {err:#}");
                }
            }
            Err(err) => log::warn!("Failed to serialize local AI provider profiles: {err:#}"),
        }
        ctx.emit(LocalAIProviderProfilesEvent::ProfilesChanged);
    }
}

impl Default for LocalAIProviderProfiles {
    fn default() -> Self {
        Self {
            profiles: vec![LocalAIProviderProfile::default_openai_compatible()],
            active_profile_id: Some(DEFAULT_PROFILE_ID.to_string()),
        }
    }
}

impl Entity for LocalAIProviderProfiles {
    type Event = LocalAIProviderProfilesEvent;
}

impl SingletonEntity for LocalAIProviderProfiles {}

fn unavailable_local_models() -> AvailableLLMs {
    AvailableLLMs::new(
        "local-provider-unconfigured".to_string().into(),
        [LLMInfo {
            display_name: "Configure a local provider".to_string(),
            base_model_name: "Configure a local provider".to_string(),
            id: "local-provider-unconfigured".to_string().into(),
            reasoning_level: None,
            usage_metadata: LLMUsageMetadata {
                request_multiplier: 1,
                credit_multiplier: None,
            },
            description: Some("OpenAI-compatible local profile required".to_string()),
            disable_reason: Some(DisableReason::Unavailable),
            vision_supported: true,
            spec: None,
            provider: LLMProvider::OpenAI,
            host_configs: HashMap::new(),
            discount_percentage: None,
            context_window: LLMContextWindow::default(),
        }],
        None,
    )
    .expect("unavailable local provider fallback includes one choice")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::settings::initialize_settings_for_tests;
    use warpui::App;

    #[test]
    fn local_provider_profile_metadata_does_not_store_api_key() {
        App::test((), |mut app| async move {
            initialize_settings_for_tests(&mut app);
            let profiles = app.add_singleton_model(LocalAIProviderProfiles::new);

            profiles.update(&mut app, |profiles, ctx| {
                profiles.set_default_profile(
                    "Local gateway".to_string(),
                    "http://localhost:11434/v1".to_string(),
                    "gpt-local".to_string(),
                    LocalAIProviderCompatibility::OpenAIChatCompletions,
                    ctx,
                );
                profiles.set_default_api_key(Some("sk-local-secret".to_string()), ctx);
            });

            let serialized = app.update(|ctx| {
                ctx.private_user_preferences()
                    .read_value(LOCAL_PROVIDER_PROFILES_KEY)
                    .expect("preference read should succeed")
                    .expect("local provider profiles should be persisted")
            });

            assert!(serialized.contains("Local gateway"));
            assert!(serialized.contains("gpt-local"));
            assert!(!serialized.contains("sk-local-secret"));
        });
    }

    #[test]
    fn local_provider_profiles_build_local_model_choices() {
        App::test((), |mut app| async move {
            initialize_settings_for_tests(&mut app);
            let profiles = app.add_singleton_model(LocalAIProviderProfiles::new);

            profiles.update(&mut app, |profiles, ctx| {
                profiles.set_default_profile(
                    "Local gateway".to_string(),
                    "http://localhost:11434/v1".to_string(),
                    "gpt-local".to_string(),
                    LocalAIProviderCompatibility::OpenAIChatCompletions,
                    ctx,
                );
            });

            let serialized_models = profiles.read(&app, |profiles, ctx| {
                serde_json::to_string(&profiles.models_by_feature(ctx))
                    .expect("models should serialize")
            });

            assert!(serialized_models.contains("Local gateway"));
            assert!(serialized_models.contains("gpt-local"));
            assert!(!serialized_models.contains("RequiresUpgrade"));
            assert!(!serialized_models.contains("OutOfRequests"));
        });
    }
}
