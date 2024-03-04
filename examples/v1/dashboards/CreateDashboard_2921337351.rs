// Create a new dashboard with trace_service widget
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::ServiceSummaryWidgetDefinition(
                        Box::new(
                            ServiceSummaryWidgetDefinition::new(
                                "none".to_string(),
                                "".to_string(),
                                "".to_string(),
                                ServiceSummaryWidgetDefinitionType::TRACE_SERVICE,
                            )
                                .display_format(WidgetServiceSummaryDisplayFormat::TWO_COLUMN)
                                .show_breakdown(true)
                                .show_distribution(true)
                                .show_errors(true)
                                .show_hits(true)
                                .show_latency(true)
                                .show_resource_list(false)
                                .size_format(WidgetSizeFormat::MEDIUM)
                                .time(WidgetTime::new())
                                .title("Service Summary".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(72, 72, 0, 0))
            ],
        )
            .description(Some("".to_string()))
            .is_read_only(false)
            .notify_list(Some(vec![]))
            .template_variables(Some(vec![]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
