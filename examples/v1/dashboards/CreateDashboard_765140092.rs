// Create a new dashboard with a query value widget using timeseries background
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::*;
use std::ops::Add;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH,
};

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard with QVW Timeseries Background".to_string(),
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
                                                            "sum:my.cool.count.metric{*}".to_string(),
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
                                .timeseries_background(
                                    TimeseriesBackground::new(
                                        TimeseriesBackgroundType::AREA,
                                    ).yaxis(WidgetAxis::new().include_zero(true)),
                                )
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
