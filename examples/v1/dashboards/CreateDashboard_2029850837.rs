// Create a new dashboard with log_stream widget
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
                    WidgetDefinition::LogStreamWidgetDefinition(
                        Box::new(
                            LogStreamWidgetDefinition::new(LogStreamWidgetDefinitionType::LOG_STREAM)
                                .columns(vec!["host".to_string(), "service".to_string()])
                                .indexes(vec!["main".to_string()])
                                .message_display(WidgetMessageDisplay::EXPANDED_MEDIUM)
                                .query("".to_string())
                                .show_date_column(true)
                                .show_message_column(true)
                                .sort(WidgetFieldSort::new("time".to_string(), WidgetSort::DESCENDING))
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(36, 47, 0, 0))
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
