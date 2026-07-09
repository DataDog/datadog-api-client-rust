// Create a new dashboard with hostmap DDSQL widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DatasetListQuery;
use datadog_api_client::datadogV1::model::DatasetListQueryDataSourceType;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinition;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinitionRequestType;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinitionRequests;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::HostMapWidgetDimension;
use datadog_api_client::datadogV1::model::HostMapWidgetInfrastructureStyle;
use datadog_api_client::datadogV1::model::HostMapWidgetProjection;
use datadog_api_client::datadogV1::model::HostMapWidgetProjectionDimensionMapping;
use datadog_api_client::datadogV1::model::HostMapWidgetProjectionType;
use datadog_api_client::datadogV1::model::PublishedDatasetProvider;
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
            Widget::new(WidgetDefinition::HostMapWidgetDefinition(Box::new(
                HostMapWidgetDefinition::new(
                    HostMapWidgetDefinitionRequests::new()
                        .limit(1000)
                        .projection(HostMapWidgetProjection::new(
                            vec![
                                HostMapWidgetProjectionDimensionMapping::new(
                                    "entity_id".to_string(),
                                    HostMapWidgetDimension::NODE,
                                ),
                                HostMapWidgetProjectionDimensionMapping::new(
                                    "parent_id".to_string(),
                                    HostMapWidgetDimension::GROUP,
                                ),
                                HostMapWidgetProjectionDimensionMapping::new(
                                    "cpu_usage".to_string(),
                                    HostMapWidgetDimension::FILL,
                                ),
                            ],
                            HostMapWidgetProjectionType::HOSTMAP,
                        ))
                        .query(DatasetListQuery::new(
                            DatasetListQueryDataSourceType::DATASET,
                            "abc-123-def".to_string(),
                            PublishedDatasetProvider::DDSQL_QUERY,
                        ))
                        .request_type(HostMapWidgetDefinitionRequestType::DATA_PROJECTION)
                        .style(
                            HostMapWidgetInfrastructureStyle::new()
                                .palette("green_to_orange".to_string())
                                .palette_flip(false),
                        ),
                    HostMapWidgetDefinitionType::HOSTMAP,
                )
                .title("".to_string())
                .title_align(WidgetTextAlign::LEFT)
                .title_size("16".to_string()),
            )))
            .layout(WidgetLayout::new(22, 47, 0, 0)),
        ],
    )
    .description(None)
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
