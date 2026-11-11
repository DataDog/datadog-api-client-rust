// Create a heatgrid widget with bin labels and formatted row labels
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::HeatgridColor;
use datadog_api_client::datadogV1::model::HeatgridColorBin;
use datadog_api_client::datadogV1::model::HeatgridColorConfig;
use datadog_api_client::datadogV1::model::HeatgridCustomColorSource;
use datadog_api_client::datadogV1::model::HeatgridDiscreteCustomColor;
use datadog_api_client::datadogV1::model::HeatgridDiscreteMode;
use datadog_api_client::datadogV1::model::HeatgridLabelColumn;
use datadog_api_client::datadogV1::model::HeatgridNestingDisplay;
use datadog_api_client::datadogV1::model::HeatgridSort;
use datadog_api_client::datadogV1::model::HeatgridSortBy;
use datadog_api_client::datadogV1::model::HeatgridSortByLabel;
use datadog_api_client::datadogV1::model::HeatgridSortByLabelProperty;
use datadog_api_client::datadogV1::model::HeatgridSortOrder;
use datadog_api_client::datadogV1::model::HeatgridTextFormatMatch;
use datadog_api_client::datadogV1::model::HeatgridTextFormatMatchType;
use datadog_api_client::datadogV1::model::HeatgridTextFormatReplace;
use datadog_api_client::datadogV1::model::HeatgridTextFormatReplaceAll;
use datadog_api_client::datadogV1::model::HeatgridTextFormatReplaceAllType;
use datadog_api_client::datadogV1::model::HeatgridTextFormatReplaceSubstring;
use datadog_api_client::datadogV1::model::HeatgridTextFormatReplaceSubstringType;
use datadog_api_client::datadogV1::model::HeatgridTextFormatRule;
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
                                    HeatgridSortBy::HeatgridSortByLabel(
                                        Box::new(
                                            HeatgridSortByLabel::new(
                                                HeatgridSortOrder::ASC,
                                                HeatgridSortByLabelProperty::LABEL,
                                            ),
                                        ),
                                    ),
                                ),
                                HeatgridWidgetDefinitionType::HEATGRID,
                            )
                                .color(
                                    HeatgridColorConfig::HeatgridDiscreteCustomColor(
                                        Box::new(
                                            HeatgridDiscreteCustomColor::new(
                                                vec![
                                                    HeatgridColorBin::new(
                                                        HeatgridColor::String("#00FF00".to_string()),
                                                    ).label("Normal".to_string()),
                                                    HeatgridColorBin::new(
                                                        HeatgridColor::String("#FF0000".to_string()),
                                                    )
                                                        .label("High".to_string())
                                                        .lower_bound(80.0 as f64)
                                                ],
                                                HeatgridDiscreteMode::DISCRETE,
                                                HeatgridCustomColorSource::CUSTOM,
                                            ),
                                        ),
                                    ),
                                )
                                .label_column(
                                    HeatgridLabelColumn
                                    ::new().text_formats(
                                        vec![
                                            HeatgridTextFormatRule::new(
                                                HeatgridTextFormatMatch::new(
                                                    HeatgridTextFormatMatchType::STARTS_WITH,
                                                    "api-".to_string(),
                                                ),
                                            ).replace(
                                                HeatgridTextFormatReplace::HeatgridTextFormatReplaceSubstring(
                                                    Box::new(
                                                        HeatgridTextFormatReplaceSubstring::new(
                                                            "api-".to_string(),
                                                            HeatgridTextFormatReplaceSubstringType::SUBSTRING,
                                                            "".to_string(),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            HeatgridTextFormatRule::new(
                                                HeatgridTextFormatMatch::new(
                                                    HeatgridTextFormatMatchType::IS,
                                                    "unknown".to_string(),
                                                ),
                                            ).replace(
                                                HeatgridTextFormatReplace::HeatgridTextFormatReplaceAll(
                                                    Box::new(
                                                        HeatgridTextFormatReplaceAll::new(
                                                            HeatgridTextFormatReplaceAllType::ALL,
                                                            "Other".to_string(),
                                                        ),
                                                    ),
                                                ),
                                            )
                                        ],
                                    ),
                                ),
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
