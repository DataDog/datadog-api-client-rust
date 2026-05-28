// Create a new dashboard with hostmap infra widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinition;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinitionRequests;
use datadog_api_client::datadogV1::model::HostMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::HostMapWidgetDimension;
use datadog_api_client::datadogV1::model::HostMapWidgetFormula;
use datadog_api_client::datadogV1::model::HostMapWidgetGroupBy;
use datadog_api_client::datadogV1::model::HostMapWidgetInfrastructureRequestRequestType;
use datadog_api_client::datadogV1::model::HostMapWidgetInfrastructureStyle;
use datadog_api_client::datadogV1::model::HostMapWidgetNodeType;
use datadog_api_client::datadogV1::model::HostMapWidgetScalarRequest;
use datadog_api_client::datadogV1::model::HostMapWidgetScalarRequestResponseFormat;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

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
                                HostMapWidgetDefinitionRequests::new()
                                    .enrichments(
                                        vec![
                                            HostMapWidgetScalarRequest::new(
                                                vec![
                                                    HostMapWidgetFormula::new(
                                                        HostMapWidgetDimension::FILL,
                                                        "query1".to_string(),
                                                    )
                                                ],
                                                vec![
                                                    FormulaAndFunctionQueryDefinition
                                                    ::FormulaAndFunctionMetricQueryDefinition(
                                                        Box::new(
                                                            FormulaAndFunctionMetricQueryDefinition::new(
                                                                FormulaAndFunctionMetricDataSource::METRICS,
                                                                "query1".to_string(),
                                                                "avg:system.cpu.user{*} by {host}".to_string(),
                                                            ),
                                                        ),
                                                    )
                                                ],
                                                HostMapWidgetScalarRequestResponseFormat::SCALAR,
                                            )
                                        ],
                                    )
                                    .filter("env:prod".to_string())
                                    .group_by(
                                        vec![
                                            HostMapWidgetGroupBy::new("tags".to_string()).key("service".to_string())
                                        ],
                                    )
                                    .node_type(HostMapWidgetNodeType::HOST)
                                    .request_type(
                                        HostMapWidgetInfrastructureRequestRequestType::INFRASTRUCTURE_HOSTMAP,
                                    )
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
                        ),
                    ),
                ).layout(WidgetLayout::new(22, 47, 0, 0))
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
