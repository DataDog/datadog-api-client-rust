// Create a new dashboard with topology_map data_streams widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinition;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinitionDataStreams;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TopologyQueryDataStreams;
use datadog_api_client::datadogV1::model::TopologyQueryDataStreamsDataSource;
use datadog_api_client::datadogV1::model::TopologyRequestDataStreams;
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
                TopologyMapWidgetDefinition::TopologyMapWidgetDefinitionDataStreams(Box::new(
                    TopologyMapWidgetDefinitionDataStreams::new(
                        vec![TopologyRequestDataStreams::new()
                            .query(
                                TopologyQueryDataStreams::new(
                                    TopologyQueryDataStreamsDataSource::DATA_STREAMS,
                                    vec!["env:prod".to_string()],
                                    "".to_string(),
                                )
                                .query_string("service:myservice".to_string()),
                            )
                            .request_type(TopologyRequestType::TOPOLOGY)],
                        TopologyMapWidgetDefinitionType::TOPOLOGY_MAP,
                    )
                    .title("".to_string())
                    .title_align(WidgetTextAlign::LEFT)
                    .title_size("16".to_string()),
                )),
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
