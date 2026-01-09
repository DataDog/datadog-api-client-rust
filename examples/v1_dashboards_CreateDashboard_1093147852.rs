// Create a new dashboard with distribution widget with markers and num_buckets
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DistributionWidgetDefinition;
use datadog_api_client::datadogV1::model::DistributionWidgetDefinitionType;
use datadog_api_client::datadogV1::model::DistributionWidgetRequest;
use datadog_api_client::datadogV1::model::DistributionWidgetXAxis;
use datadog_api_client::datadogV1::model::DistributionWidgetYAxis;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetMarker;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::DistributionWidgetDefinition(
                        Box::new(
                            DistributionWidgetDefinition::new(
                                vec![
                                    DistributionWidgetRequest::new()
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
                                DistributionWidgetDefinitionType::DISTRIBUTION,
                            )
                                .markers(
                                    vec![
                                        WidgetMarker::new("50".to_string()).display_type("percentile".to_string()),
                                        WidgetMarker::new("99".to_string()).display_type("percentile".to_string()),
                                        WidgetMarker::new("90".to_string()).display_type("percentile".to_string())
                                    ],
                                )
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string())
                                .xaxis(
                                    DistributionWidgetXAxis::new()
                                        .include_zero(true)
                                        .max("auto".to_string())
                                        .min("auto".to_string())
                                        .num_buckets(55)
                                        .scale("linear".to_string()),
                                )
                                .yaxis(
                                    DistributionWidgetYAxis::new()
                                        .include_zero(true)
                                        .max("auto".to_string())
                                        .min("auto".to_string())
                                        .scale("linear".to_string()),
                                ),
                        ),
                    ),
                ).layout(WidgetLayout::new(4, 4, 0, 0))
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
