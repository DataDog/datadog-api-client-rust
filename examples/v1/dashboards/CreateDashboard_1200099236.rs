// Create a new dashboard with hostmap widget
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
                    WidgetDefinition::HostMapWidgetDefinition(
                        Box::new(
                            HostMapWidgetDefinition::new(
                                HostMapWidgetDefinitionRequests
                                ::new().fill(HostMapRequest::new().q("avg:system.cpu.user{*} by {host}".to_string())),
                                HostMapWidgetDefinitionType::HOSTMAP,
                            )
                                .no_group_hosts(true)
                                .no_metric_hosts(true)
                                .node_type(WidgetNodeType::HOST)
                                .style(
                                    HostMapWidgetDefinitionStyle::new()
                                        .palette("green_to_orange".to_string())
                                        .palette_flip(false),
                                )
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(22, 47, 0, 0))
            ],
        )
            .description(Some(None))
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
