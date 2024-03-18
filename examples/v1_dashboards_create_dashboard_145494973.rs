// Create a new dashboard with apm resource stats widget
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
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TableWidgetDefinition(
                        Box::new(
                            TableWidgetDefinition::new(
                                vec![
                                    TableWidgetRequest::new()
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionApmResourceStatsQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionApmResourceStatsQueryDefinition::new(
                                                            FormulaAndFunctionApmResourceStatsDataSource
                                                            ::APM_RESOURCE_STATS,
                                                            "ci".to_string(),
                                                            "query1".to_string(),
                                                            "cassandra".to_string(),
                                                            FormulaAndFunctionApmResourceStatName::HITS,
                                                        )
                                                            .group_by(vec!["resource_name".to_string()])
                                                            .operation_name("cassandra.query".to_string())
                                                            .primary_tag_name("datacenter".to_string())
                                                            .primary_tag_value("edge-eu1.prod.dog".to_string()),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                TableWidgetDefinitionType::QUERY_TABLE,
                            )
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(4, 4, 0, 0))
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
