// Create a heatgrid widget with a preset gradient
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::HeatgridColorConfig;
use datadog_api_client::datadogV1::model::HeatgridGradientMode;
use datadog_api_client::datadogV1::model::HeatgridGradientPresetColor;
use datadog_api_client::datadogV1::model::HeatgridLabelColumn;
use datadog_api_client::datadogV1::model::HeatgridLabelColumnWidth;
use datadog_api_client::datadogV1::model::HeatgridLegend;
use datadog_api_client::datadogV1::model::HeatgridNestingDisplay;
use datadog_api_client::datadogV1::model::HeatgridPresetColorSource;
use datadog_api_client::datadogV1::model::HeatgridSort;
use datadog_api_client::datadogV1::model::HeatgridSortAggregation;
use datadog_api_client::datadogV1::model::HeatgridSortBy;
use datadog_api_client::datadogV1::model::HeatgridSortByValue;
use datadog_api_client::datadogV1::model::HeatgridSortByValueProperty;
use datadog_api_client::datadogV1::model::HeatgridSortOrder;
use datadog_api_client::datadogV1::model::HeatgridWidgetDefinition;
use datadog_api_client::datadogV1::model::HeatgridWidgetDefinitionType;
use datadog_api_client::datadogV1::model::HeatgridWidgetFormula;
use datadog_api_client::datadogV1::model::HeatgridWidgetRequest;
use datadog_api_client::datadogV1::model::HeatgridWidgetResponseFormat;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::HeatgridWidgetDefinition(
                        Box::new(
                            HeatgridWidgetDefinition::new(
                                vec![
                                    HeatgridWidgetRequest::new(
                                        vec![
                                            FormulaAndFunctionQueryDefinition::FormulaAndFunctionMetricQueryDefinition(
                                                Box::new(
                                                    FormulaAndFunctionMetricQueryDefinition::new(
                                                        FormulaAndFunctionMetricDataSource::METRICS,
                                                        "query1".to_string(),
                                                        "avg:system.cpu.user{*} by {host}".to_string(),
                                                    ),
                                                ),
                                            )
                                        ],
                                        HeatgridWidgetResponseFormat::TIMESERIES,
                                    ).formulas(vec![HeatgridWidgetFormula::new("query1".to_string())])
                                ],
                                HeatgridSort::new(
                                    HeatgridNestingDisplay::FLAT,
                                    HeatgridSortBy::HeatgridSortByValue(
                                        Box::new(
                                            HeatgridSortByValue::new(
                                                HeatgridSortAggregation::AVG,
                                                HeatgridSortOrder::DESC,
                                                HeatgridSortByValueProperty::VALUE,
                                            ),
                                        ),
                                    ),
                                ),
                                HeatgridWidgetDefinitionType::HEATGRID,
                            )
                                .color(
                                    HeatgridColorConfig::HeatgridGradientPresetColor(
                                        Box::new(
                                            HeatgridGradientPresetColor::new(
                                                HeatgridGradientMode::GRADIENT,
                                                "classic".to_string(),
                                                HeatgridPresetColorSource::PRESET,
                                            ),
                                        ),
                                    ),
                                )
                                .label_column(HeatgridLabelColumn::new(HeatgridLabelColumnWidth::M))
                                .legend(HeatgridLegend::new().show_caption(true)),
                        ),
                    ),
                )
            ],
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
