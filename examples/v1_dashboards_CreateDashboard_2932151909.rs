// Create a new dashboard with sankey widget and rum data source
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::SankeyRumDataSource;
use datadog_api_client::datadogV1::model::SankeyRumQuery;
use datadog_api_client::datadogV1::model::SankeyRumQueryMode;
use datadog_api_client::datadogV1::model::SankeyRumRequest;
use datadog_api_client::datadogV1::model::SankeyWidgetDefinition;
use datadog_api_client::datadogV1::model::SankeyWidgetDefinitionType;
use datadog_api_client::datadogV1::model::SankeyWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::FREE,
        "Example-Dashboard".to_string(),
        vec![
            Widget::new(WidgetDefinition::SankeyWidgetDefinition(Box::new(
                SankeyWidgetDefinition::new(
                    vec![SankeyWidgetRequest::SankeyRumRequest(Box::new(
                        SankeyRumRequest::new(
                            SankeyRumQuery::new(
                                SankeyRumDataSource::RUM,
                                SankeyRumQueryMode::SOURCE,
                                "@type:view".to_string(),
                            ),
                            SankeyWidgetDefinitionType::SANKEY,
                        ),
                    ))],
                    SankeyWidgetDefinitionType::SANKEY,
                )
                .title("".to_string())
                .title_align(WidgetTextAlign::LEFT)
                .title_size("16".to_string()),
            )))
            .layout(WidgetLayout::new(15, 47, 0, 0)),
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
