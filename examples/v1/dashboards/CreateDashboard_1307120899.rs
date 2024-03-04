// Create a new timeseries widget with ci_tests data source
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
            "Example-Dashboard with ci_tests datasource".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TimeseriesWidgetDefinition(
                        Box::new(
                            TimeseriesWidgetDefinition::new(
                                vec![
                                    TimeseriesWidgetRequest::new()
                                        .display_type(WidgetDisplayType::LINE)
                                        .formulas(vec![WidgetFormula::new("query1".to_string())])
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionEventQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionEventQueryDefinition::new(
                                                            FormulaAndFunctionEventQueryDefinitionCompute::new(
                                                                FormulaAndFunctionEventAggregation::COUNT,
                                                            ),
                                                            FormulaAndFunctionEventsDataSource::CI_TESTS,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(vec![])
                                                            .indexes(vec!["*".to_string()])
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "test_level:test".to_string(),
                                                                ),
                                                            ),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::TIMESERIES)
                                        .style(
                                            WidgetRequestStyle::new()
                                                .line_type(WidgetLineType::SOLID)
                                                .line_width(WidgetLineWidth::NORMAL)
                                                .palette("dog_classic".to_string()),
                                        )
                                ],
                                TimeseriesWidgetDefinitionType::TIMESERIES,
                            )
                                .legend_columns(
                                    vec![
                                        TimeseriesWidgetLegendColumn::AVG,
                                        TimeseriesWidgetLegendColumn::MIN,
                                        TimeseriesWidgetLegendColumn::MAX,
                                        TimeseriesWidgetLegendColumn::VALUE,
                                        TimeseriesWidgetLegendColumn::SUM
                                    ],
                                )
                                .legend_layout(TimeseriesWidgetLegendLayout::AUTO)
                                .show_legend(true)
                                .time(WidgetTime::new())
                                .title("".to_string()),
                        ),
                    ),
                )
            ],
        ).reflow_type(DashboardReflowType::AUTO);
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
