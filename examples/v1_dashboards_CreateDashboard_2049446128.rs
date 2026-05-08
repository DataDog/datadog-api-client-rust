// Create a new dashboard with point_plot widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DataProjectionQuery;
use datadog_api_client::datadogV1::model::DataProjectionRequestType;
use datadog_api_client::datadogV1::model::PointPlotDimension;
use datadog_api_client::datadogV1::model::PointPlotProjection;
use datadog_api_client::datadogV1::model::PointPlotProjectionDimension;
use datadog_api_client::datadogV1::model::PointPlotProjectionType;
use datadog_api_client::datadogV1::model::PointPlotWidgetDefinition;
use datadog_api_client::datadogV1::model::PointPlotWidgetDefinitionType;
use datadog_api_client::datadogV1::model::PointPlotWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard".to_string(),
        vec![Widget::new(WidgetDefinition::PointPlotWidgetDefinition(
            Box::new(
                PointPlotWidgetDefinition::new(
                    vec![PointPlotWidgetRequest::new(
                        PointPlotProjection::new(
                            vec![
                                PointPlotProjectionDimension::new(
                                    "host".to_string(),
                                    PointPlotDimension::GROUP,
                                ),
                                PointPlotProjectionDimension::new(
                                    "@duration".to_string(),
                                    PointPlotDimension::Y,
                                ),
                            ],
                            PointPlotProjectionType::POINT_PLOT,
                        ),
                        DataProjectionQuery::new(
                            "logs".to_string(),
                            "service:web-store".to_string(),
                        ),
                        DataProjectionRequestType::DATA_PROJECTION,
                    )],
                    PointPlotWidgetDefinitionType::POINT_PLOT,
                )
                .title("".to_string())
                .title_align(WidgetTextAlign::LEFT)
                .title_size("16".to_string()),
            ),
        ))],
    );
    let configuration = datadog::Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
