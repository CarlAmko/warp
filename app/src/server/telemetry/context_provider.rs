use warp_core::telemetry::{TelemetryContextModel, TelemetryContextProvider};
use warpui::{AppContext, ModelContext, SingletonEntity};

use crate::auth::AuthStateProvider;

pub struct AppTelemetryContextProvider {}
pub struct NoopTelemetryContextProvider {}

impl AppTelemetryContextProvider {
    pub fn new_context_provider(
        _ctx: &mut ModelContext<TelemetryContextModel>,
    ) -> TelemetryContextModel {
        Box::new(Self {})
    }
}

impl TelemetryContextProvider for AppTelemetryContextProvider {
    fn user_id(&self, ctx: &AppContext) -> Option<String> {
        let auth_state = AuthStateProvider::as_ref(ctx).get();
        auth_state.user_id().map(|uid| uid.as_string())
    }

    fn anonymous_id(&self, ctx: &AppContext) -> String {
        let auth_state = AuthStateProvider::as_ref(ctx).get();
        auth_state.anonymous_id()
    }
}

impl NoopTelemetryContextProvider {
    pub fn new_context_provider(
        _ctx: &mut ModelContext<TelemetryContextModel>,
    ) -> TelemetryContextModel {
        Box::new(Self {})
    }
}

impl TelemetryContextProvider for NoopTelemetryContextProvider {
    fn user_id(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    fn anonymous_id(&self, _ctx: &AppContext) -> String {
        String::new()
    }
}
