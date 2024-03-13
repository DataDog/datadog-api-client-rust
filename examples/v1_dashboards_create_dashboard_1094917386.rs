// Create a new dashboard with manage_status widget and show_priority parameter
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::FREE,
        "Example-Dashboard".to_string(),
        vec![
            Widget::new(WidgetDefinition::MonitorSummaryWidgetDefinition(Box::new(
                MonitorSummaryWidgetDefinition::new(
                    "".to_string(),
                    MonitorSummaryWidgetDefinitionType::MANAGE_STATUS,
                )
                .color_preference(WidgetColorPreference::TEXT)
                .count(50)
                .display_format(WidgetMonitorSummaryDisplayFormat::COUNTS_AND_LIST)
                .hide_zero_counts(true)
                .show_last_triggered(false)
                .show_priority(false)
                .sort(WidgetMonitorSummarySort::PRIORITY_ASCENDING)
                .start(0)
                .summary_type(WidgetSummaryType::MONITORS),
            )))
            .layout(WidgetLayout::new(25, 50, 0, 0)),
        ],
    )
    .description(Some("".to_string()))
    .is_read_only(false)
    .notify_list(Some(vec![]))
    .template_variables(Some(vec![]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}