// Create a new dashboard with topology_map data_streams widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinition;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TopologyQuery;
use datadog_api_client::datadogV1::model::TopologyQueryDataStreamsOrServiceMap;
use datadog_api_client::datadogV1::model::TopologyQueryDataStreamsOrServiceMapDataSource;
use datadog_api_client::datadogV1::model::TopologyRequest;
use datadog_api_client::datadogV1::model::TopologyRequestType;
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
            Widget::new(WidgetDefinition::TopologyMapWidgetDefinition(Box::new(
                TopologyMapWidgetDefinition::new(
                    vec![TopologyRequest::new()
                        .query(TopologyQuery::TopologyQueryDataStreamsOrServiceMap(
                            Box::new(
                                TopologyQueryDataStreamsOrServiceMap::new(
                                    TopologyQueryDataStreamsOrServiceMapDataSource::DATA_STREAMS,
                                    vec!["env:prod".to_string()],
                                )
                                .query_string("service:myservice".to_string())
                                .service("".to_string()),
                            ),
                        ))
                        .request_type(TopologyRequestType::TOPOLOGY)],
                    TopologyMapWidgetDefinitionType::TOPOLOGY_MAP,
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
