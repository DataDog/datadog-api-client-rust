// Create a new dashboard with sankey widget and network data source
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::SankeyNetworkDataSource;
use datadog_api_client::datadogV1::model::SankeyNetworkQuery;
use datadog_api_client::datadogV1::model::SankeyNetworkRequest;
use datadog_api_client::datadogV1::model::SankeyNetworkRequestType;
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
                    vec![SankeyWidgetRequest::SankeyNetworkRequest(Box::new(
                        SankeyNetworkRequest::new(
                            SankeyNetworkQuery::new(
                                SankeyNetworkDataSource::NETWORK,
                                vec!["source".to_string(), "destination".to_string()],
                                100,
                                "*".to_string(),
                            ),
                            SankeyNetworkRequestType::NETFLOW_SANKEY,
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
