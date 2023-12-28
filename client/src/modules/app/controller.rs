use super::service::app_service;

pub async fn app_controller() -> &'static str {
    app_service().await
}
