// Create a new dashboard with timeseries widget with custom_unit
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DashboardReflowType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::NumberFormatUnit;
use datadog_api_client::datadogV1::model::NumberFormatUnitCanonical;
use datadog_api_client::datadogV1::model::NumberFormatUnitScale;
use datadog_api_client::datadogV1::model::NumberFormatUnitScaleType;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinition;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TimeseriesWidgetLegendLayout;
use datadog_api_client::datadogV1::model::TimeseriesWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetDisplayType;
use datadog_api_client::datadogV1::model::WidgetFormula;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetLegacyLiveSpan;
use datadog_api_client::datadogV1::model::WidgetNumberFormat;
use datadog_api_client::datadogV1::model::WidgetTextAlign;
use datadog_api_client::datadogV1::model::WidgetTime;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TimeseriesWidgetDefinition(
                        Box::new(
                            TimeseriesWidgetDefinition::new(
                                vec![
                                    TimeseriesWidgetRequest::new()
                                        .display_type(WidgetDisplayType::LINE)
                                        .formulas(
                                            vec![
                                                WidgetFormula::new(
                                                    "query1".to_string(),
                                                ).number_format(
                                                    WidgetNumberFormat::new()
                                                        .unit(
                                                            NumberFormatUnit::NumberFormatUnitCanonical(
                                                                Box::new(
                                                                    NumberFormatUnitCanonical::new()
                                                                        .type_(
                                                                            NumberFormatUnitScaleType::CANONICAL_UNIT,
                                                                        )
                                                                        .unit_name("fraction".to_string()),
                                                                ),
                                                            ),
                                                        )
                                                        .unit_scale(
                                                            Some(
                                                                NumberFormatUnitScale::new()
                                                                    .type_(NumberFormatUnitScaleType::CANONICAL_UNIT)
                                                                    .unit_name("apdex".to_string()),
                                                            ),
                                                        ),
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
                                                            "avg:system.cpu.user{*}".to_string(),
                                                        ),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::TIMESERIES)
                                ],
                                TimeseriesWidgetDefinitionType::TIMESERIES,
                            )
                                .legend_layout(TimeseriesWidgetLegendLayout::AUTO)
                                .show_legend(true)
                                .time(WidgetTime::WidgetLegacyLiveSpan(Box::new(WidgetLegacyLiveSpan::new())))
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(5, 12, 0, 0))
            ],
        )
            .description(Some("".to_string()))
            .notify_list(Some(vec![]))
            .reflow_type(DashboardReflowType::FIXED)
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
