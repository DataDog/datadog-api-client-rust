// Create a new dashboard with formulas and functions events query using flat
// group by fields
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupByConfig;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupByFields;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventsDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinition;
use datadog_api_client::datadogV1::model::TimeseriesWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TimeseriesWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetLayout;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard with events flat group_by fields".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TimeseriesWidgetDefinition(
                        Box::new(
                            TimeseriesWidgetDefinition::new(
                                vec![
                                    TimeseriesWidgetRequest::new()
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionEventQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionEventQueryDefinition::new(
                                                            FormulaAndFunctionEventQueryDefinitionCompute::new(
                                                                FormulaAndFunctionEventAggregation::COUNT,
                                                            ),
                                                            FormulaAndFunctionEventsDataSource::EVENTS,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(
                                                                FormulaAndFunctionEventQueryGroupByConfig
                                                                ::FormulaAndFunctionEventQueryGroupByFields(
                                                                    Box::new(
                                                                        FormulaAndFunctionEventQueryGroupByFields::new(
                                                                            vec![
                                                                                "service".to_string(),
                                                                                "host".to_string()
                                                                            ],
                                                                        ).limit(10),
                                                                    ),
                                                                ),
                                                            )
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "".to_string(),
                                                                ),
                                                            ),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::TIMESERIES)
                                ],
                                TimeseriesWidgetDefinitionType::TIMESERIES,
                            ),
                        ),
                    ),
                ).layout(WidgetLayout::new(2, 4, 0, 0))
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
