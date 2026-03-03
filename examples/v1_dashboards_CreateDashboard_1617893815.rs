// Create a new dashboard with formula and function distribution widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DistributionWidgetDefinition;
use datadog_api_client::datadogV1::model::DistributionWidgetDefinitionType;
use datadog_api_client::datadogV1::model::DistributionWidgetRequest;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupBy;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupByConfig;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupBySort;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventsDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::QuerySortOrder;
use datadog_api_client::datadogV1::model::Widget;
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
                    WidgetDefinition::DistributionWidgetDefinition(
                        Box::new(
                            DistributionWidgetDefinition::new(
                                vec![
                                    DistributionWidgetRequest::new()
                                        .queries(
                                            vec![
                                                FormulaAndFunctionQueryDefinition
                                                ::FormulaAndFunctionEventQueryDefinition(
                                                    Box::new(
                                                        FormulaAndFunctionEventQueryDefinition::new(
                                                            FormulaAndFunctionEventQueryDefinitionCompute::new(
                                                                FormulaAndFunctionEventAggregation::AVG,
                                                            ).metric("@duration".to_string()),
                                                            FormulaAndFunctionEventsDataSource::LOGS,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(
                                                                FormulaAndFunctionEventQueryGroupByConfig
                                                                ::FormulaAndFunctionEventQueryGroupByList(
                                                                    vec![
                                                                        FormulaAndFunctionEventQueryGroupBy::new(
                                                                            "service".to_string(),
                                                                        )
                                                                            .limit(1000)
                                                                            .sort(
                                                                                FormulaAndFunctionEventQueryGroupBySort
                                                                                ::new(
                                                                                    FormulaAndFunctionEventAggregation
                                                                                    ::COUNT,
                                                                                ).order(QuerySortOrder::DESC),
                                                                            )
                                                                    ],
                                                                ),
                                                            )
                                                            .indexes(vec!["*".to_string()])
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "".to_string(),
                                                                ),
                                                            )
                                                            .storage("hot".to_string()),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                DistributionWidgetDefinitionType::DISTRIBUTION,
                            )
                                .time(WidgetTime::WidgetLegacyLiveSpan(Box::new(WidgetLegacyLiveSpan::new())))
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(15, 47, 0, 0))
            ],
        )
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
