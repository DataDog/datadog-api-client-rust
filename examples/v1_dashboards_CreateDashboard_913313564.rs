// Create a new dashboard with iframe widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::IFrameWidgetDefinition;
use datadog_api_client::datadogV1::model::IFrameWidgetDefinitionType;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::FREE,
        "Example-Dashboard".to_string(),
        vec![
            Widget::new(WidgetDefinition::IFrameWidgetDefinition(Box::new(
                IFrameWidgetDefinition::new(
                    IFrameWidgetDefinitionType::IFRAME,
                    "https://docs.datadoghq.com/api/latest/".to_string(),
                ),
            )))
            .layout(WidgetLayout::new(12, 12, 0, 0)),
        ],
    )
    .description(Some("".to_string()))
    .notify_list(Some(vec![]))
    .template_variables(Some(vec![]));
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
