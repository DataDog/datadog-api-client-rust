// Create a new dashboard with scatterplot widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::ScatterPlotWidgetDefinition;
use datadog_api_client::datadogV1::model::ScatterPlotWidgetDefinitionRequests;
use datadog_api_client::datadogV1::model::ScatterPlotWidgetDefinitionType;
use datadog_api_client::datadogV1::model::ScatterplotDimension;
use datadog_api_client::datadogV1::model::ScatterplotTableRequest;
use datadog_api_client::datadogV1::model::ScatterplotWidgetFormula;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetAxis;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetLegacyLiveSpan;
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
                    WidgetDefinition::ScatterPlotWidgetDefinition(
                        Box::new(
                            ScatterPlotWidgetDefinition::new(
                                ScatterPlotWidgetDefinitionRequests
                                ::new().table(
                                    ScatterplotTableRequest::new()
                                        .formulas(
                                            vec![
                                                ScatterplotWidgetFormula::new(
                                                    ScatterplotDimension::X,
                                                    "query1".to_string(),
                                                ).alias("".to_string()),
                                                ScatterplotWidgetFormula::new(
                                                    ScatterplotDimension::Y,
                                                    "query2".to_string(),
                                                ).alias("".to_string())
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
                                                ),
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionMetricQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionMetricQueryDefinition::new(
                                                            FormulaAndFunctionMetricDataSource::METRICS,
                                                            "query2".to_string(),
                                                            "avg:system.mem.used{*} by {service}".to_string(),
                                                        ).aggregator(FormulaAndFunctionMetricAggregation::AVG),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR),
                                ),
                                ScatterPlotWidgetDefinitionType::SCATTERPLOT,
                            )
                                .color_by_groups(vec![])
                                .time(WidgetTime::WidgetLegacyLiveSpan(Box::new(WidgetLegacyLiveSpan::new())))
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string())
                                .xaxis(
                                    WidgetAxis::new()
                                        .include_zero(true)
                                        .max("auto".to_string())
                                        .min("auto".to_string())
                                        .scale("linear".to_string()),
                                )
                                .yaxis(
                                    WidgetAxis::new()
                                        .include_zero(true)
                                        .max("auto".to_string())
                                        .min("auto".to_string())
                                        .scale("linear".to_string()),
                                ),
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
