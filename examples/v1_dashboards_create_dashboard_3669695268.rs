// Create a new dashboard with logs query table widget and storage parameter
use datadog_api_client::datadog::configuration::Configuration;
use datadog_api_client::datadogV1::api::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventsDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::QuerySortOrder;
use datadog_api_client::datadogV1::model::TableWidgetCellDisplayMode;
use datadog_api_client::datadogV1::model::TableWidgetDefinition;
use datadog_api_client::datadogV1::model::TableWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TableWidgetRequest;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetFormula;
use datadog_api_client::datadogV1::model::WidgetFormulaLimit;

#[tokio::main]
async fn main() {
    let body = Dashboard::new(
        DashboardLayoutType::ORDERED,
        "Example-Dashboard with query table widget and storage parameter".to_string(),
        vec![Widget::new(WidgetDefinition::TableWidgetDefinition(
            Box::new(TableWidgetDefinition::new(
                vec![TableWidgetRequest::new()
                    .formulas(vec![WidgetFormula::new("query1".to_string())
                        .cell_display_mode(TableWidgetCellDisplayMode::BAR)
                        .conditional_formats(vec![])
                        .limit(
                            WidgetFormulaLimit::new()
                                .count(50)
                                .order(QuerySortOrder::DESC),
                        )])
                    .queries(vec![
                        FormulaAndFunctionQueryDefinition::FormulaAndFunctionEventQueryDefinition(
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
                                .search(FormulaAndFunctionEventQueryDefinitionSearch::new(
                                    "".to_string(),
                                ))
                                .storage("online_archives".to_string()),
                            ),
                        ),
                    ])
                    .response_format(FormulaAndFunctionResponseFormat::SCALAR)],
                TableWidgetDefinitionType::QUERY_TABLE,
            )),
        ))],
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
