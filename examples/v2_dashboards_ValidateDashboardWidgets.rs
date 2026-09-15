// Validate dashboard widgets returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV2::model::DashboardWidgetValidationLayoutType;
use datadog_api_client::datadogV2::model::DashboardWidgetValidationReflowType;
use datadog_api_client::datadogV2::model::DashboardWidgetValidationRequest;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body = DashboardWidgetValidationRequest::new(
        DashboardWidgetValidationLayoutType::ORDERED,
        vec![BTreeMap::from([])],
    )
    .reflow_type(DashboardWidgetValidationReflowType::AUTO);
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ValidateDashboardWidgets", true);
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.validate_dashboard_widgets(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
