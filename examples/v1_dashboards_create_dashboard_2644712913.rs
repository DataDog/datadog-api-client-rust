// Create a new dashboard with a query value widget using the percentile aggregator
use chrono::prelude::{DateTime, Utc};
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard with QVW Percentile Aggregator".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::QueryValueWidgetDefinition(
                        Box::new(
                            QueryValueWidgetDefinition::new(
                                vec![
                                    QueryValueWidgetRequest::new()
                                        .formulas(vec![WidgetFormula::new("query1".to_string())])
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionMetricQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionMetricQueryDefinition::new(
                                                            FormulaAndFunctionMetricDataSource::METRICS,
                                                            "query1".to_string(),
                                                            "p90:dist.dd.dogweb.latency{*}".to_string(),
                                                        ).aggregator(FormulaAndFunctionMetricAggregation::PERCENTILE),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                QueryValueWidgetDefinitionType::QUERY_VALUE,
                            )
                                .autoscale(true)
                                .precision(2)
                                .time(WidgetTime::new())
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(2, 2, 0, 0))
            ],
        );
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
