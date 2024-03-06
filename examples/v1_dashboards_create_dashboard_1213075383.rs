// Create a new dashboard with toplist widget
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::ToplistWidgetDefinition(
                        Box::new(
                            ToplistWidgetDefinition::new(
                                vec![
                                    ToplistWidgetRequest::new()
                                        .formulas(
                                            vec![
                                                WidgetFormula::new(
                                                    "query1".to_string(),
                                                ).limit(
                                                    WidgetFormulaLimit::new().count(10).order(QuerySortOrder::DESC),
                                                )
                                            ],
                                        )
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionMetricQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionMetricQueryDefinition::new(
                                                            FormulaAndFunctionMetricDataSource::METRICS,
                                                            "query1".to_string(),
                                                            "avg:system.cpu.user{*} by {service}".to_string(),
                                                        ).aggregator(FormulaAndFunctionMetricAggregation::AVG),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                ToplistWidgetDefinitionType::TOPLIST,
                            )
                                .style(
                                    ToplistWidgetStyle::new()
                                        .display(
                                            ToplistWidgetDisplay::ToplistWidgetStacked(
                                                Box::new(
                                                    ToplistWidgetStacked::new(
                                                        ToplistWidgetLegend::INLINE,
                                                        ToplistWidgetStackedType::STACKED,
                                                    ),
                                                ),
                                            ),
                                        )
                                        .scaling(ToplistWidgetScaling::RELATIVE),
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
            .description(Some("".to_string()))
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
