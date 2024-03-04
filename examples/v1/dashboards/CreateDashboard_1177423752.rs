// Create a new dashboard with heatmap widget
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
                    WidgetDefinition::HeatMapWidgetDefinition(
                        Box::new(
                            HeatMapWidgetDefinition::new(
                                vec![
                                    HeatMapWidgetRequest::new()
                                        .q("avg:system.cpu.user{*} by {service}".to_string())
                                        .style(WidgetStyle::new().palette("dog_classic".to_string()))
                                ],
                                HeatMapWidgetDefinitionType::HEATMAP,
                            )
                                .time(WidgetTime::new())
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(15, 47, 0, 0))
            ],
        )
            .description(Some(None))
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
