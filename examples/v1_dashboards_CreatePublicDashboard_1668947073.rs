// Create a shared dashboard with a group template variable returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::DashboardGlobalTime;
use datadog_api_client::datadogV1::model::DashboardGlobalTimeLiveSpan;
use datadog_api_client::datadogV1::model::DashboardShareType;
use datadog_api_client::datadogV1::model::DashboardType;
use datadog_api_client::datadogV1::model::SelectableTemplateVariableItems;
use datadog_api_client::datadogV1::model::SharedDashboard;

#[tokio::main]
async fn main() {
    // there is a valid "dashboard" in the system
    let dashboard_id = std::env::var("DASHBOARD_ID").unwrap();
    let body = SharedDashboard::new(dashboard_id.clone(), DashboardType::CUSTOM_TIMEBOARD)
        .global_time(
            DashboardGlobalTime::new().live_span(DashboardGlobalTimeLiveSpan::PAST_ONE_HOUR),
        )
        .selectable_template_vars(Some(vec![SelectableTemplateVariableItems::new()
            .default_value("*".to_string())
            .name("group_by_var".to_string())
            .type_(Some("group".to_string()))
            .visible_tags(Some(vec![
                "selectableValue1".to_string(),
                "selectableValue2".to_string(),
            ]))]))
        .share_type(Some(DashboardShareType::OPEN));
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_public_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
