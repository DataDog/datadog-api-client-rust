// Create a new dashboard with logs query table widget and storage parameter
use chrono::prelude::*;
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::*;
use datadog_api_client::datadogV1::model::*;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard with query table widget and storage parameter".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TableWidgetDefinition(
                        Box::new(
                            TableWidgetDefinition::new(
                                vec![
                                    TableWidgetRequest::new()
                                        .formulas(
                                            vec![
                                                WidgetFormula::new("query1".to_string())
                                                    .cell_display_mode(TableWidgetCellDisplayMode::BAR)
                                                    .conditional_formats(vec![])
                                                    .limit(
                                                        WidgetFormulaLimit::new()
                                                            .count(50)
                                                            .order(QuerySortOrder::DESC),
                                                    )
                                            ],
                                        )
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionEventQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionEventQueryDefinition::new(
                                                            FormulaAndFunctionEventQueryDefinitionCompute::new(
                                                                FormulaAndFunctionEventAggregation::COUNT,
                                                            ),
                                                            FormulaAndFunctionEventsDataSource::LOGS,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(vec![])
                                                            .indexes(vec!["*".to_string()])
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "".to_string(),
                                                                ),
                                                            )
                                                            .storage("online_archives".to_string()),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                TableWidgetDefinitionType::QUERY_TABLE,
                            ),
                        ),
                    ),
                )
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
