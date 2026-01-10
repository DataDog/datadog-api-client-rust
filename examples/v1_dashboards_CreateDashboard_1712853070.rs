// Create a new dashboard with bar_chart widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::BarChartWidgetDefinition;
use datadog_api_client::datadogV1::model::BarChartWidgetDefinitionType;
use datadog_api_client::datadogV1::model::BarChartWidgetDisplay;
use datadog_api_client::datadogV1::model::BarChartWidgetLegend;
use datadog_api_client::datadogV1::model::BarChartWidgetRequest;
use datadog_api_client::datadogV1::model::BarChartWidgetScaling;
use datadog_api_client::datadogV1::model::BarChartWidgetStacked;
use datadog_api_client::datadogV1::model::BarChartWidgetStackedType;
use datadog_api_client::datadogV1::model::BarChartWidgetStyle;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::FormulaType;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetFormula;
use datadog_api_client::datadogV1::model::WidgetFormulaSort;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetLegacyLiveSpan;
use datadog_api_client::datadogV1::model::WidgetSort;
use datadog_api_client::datadogV1::model::WidgetSortBy;
use datadog_api_client::datadogV1::model::WidgetSortOrderBy;
use datadog_api_client::datadogV1::model::WidgetTextAlign;
use datadog_api_client::datadogV1::model::WidgetTime;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::BarChartWidgetDefinition(
                        Box::new(
                            BarChartWidgetDefinition::new(
                                vec![
                                    BarChartWidgetRequest::new()
                                        .formulas(vec![WidgetFormula::new("query1".to_string())])
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
                                        .sort(
                                            WidgetSortBy::new()
                                                .count(10)
                                                .order_by(
                                                    vec![
                                                        WidgetSortOrderBy::WidgetFormulaSort(
                                                            Box::new(
                                                                WidgetFormulaSort::new(
                                                                    0,
                                                                    WidgetSort::DESCENDING,
                                                                    FormulaType::FORMULA,
                                                                ),
                                                            ),
                                                        )
                                                    ],
                                                ),
                                        )
                                ],
                                BarChartWidgetDefinitionType::BAR_CHART,
                            )
                                .style(
                                    BarChartWidgetStyle::new()
                                        .display(
                                            BarChartWidgetDisplay::BarChartWidgetStacked(
                                                Box::new(
                                                    BarChartWidgetStacked::new(
                                                        BarChartWidgetStackedType::STACKED,
                                                    ).legend(BarChartWidgetLegend::INLINE),
                                                ),
                                            ),
                                        )
                                        .palette("dog_classic".to_string())
                                        .scaling(BarChartWidgetScaling::RELATIVE),
                                )
                                .time(WidgetTime::WidgetLegacyLiveSpan(Box::new(WidgetLegacyLiveSpan::new())))
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(15, 47, 0, 0))
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
