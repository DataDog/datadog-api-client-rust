// Update a shared dashboard with selectable_template_vars returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::DashboardGlobalTimeLiveSpan;
use datadog_api_client::datadogV1::model::DashboardShareType;
use datadog_api_client::datadogV1::model::SelectableTemplateVariableItems;
use datadog_api_client::datadogV1::model::SharedDashboardUpdateRequest;
use datadog_api_client::datadogV1::model::SharedDashboardUpdateRequestGlobalTime;

#[tokio::main]
async fn main() {
    // there is a valid "shared_dashboard" in the system
    let shared_dashboard_token = std::env::var("SHARED_DASHBOARD_TOKEN").unwrap();
    let body = SharedDashboardUpdateRequest::new()
        .global_time(Some(
            SharedDashboardUpdateRequestGlobalTime::new()
                .live_span(DashboardGlobalTimeLiveSpan::PAST_FIFTEEN_MINUTES),
        ))
        .selectable_template_vars(Some(vec![SelectableTemplateVariableItems::new()
            .default_value("*".to_string())
            .name("group_by_var".to_string())
            .type_(Some("group".to_string()))
            .visible_tags(Some(vec![
                "selectableValue1".to_string(),
                "selectableValue2".to_string(),
            ]))]))
        .share_list(Some(vec![]))
        .share_type(Some(DashboardShareType::OPEN));
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api
        .update_public_dashboard(shared_dashboard_token.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
