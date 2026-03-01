// Create a geomap widget with conditional formats and text formats
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::DashboardReflowType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionCompute;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryDefinitionSearch;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventQueryGroupByConfig;
use datadog_api_client::datadogV1::model::FormulaAndFunctionEventsDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionResponseFormat;
use datadog_api_client::datadogV1::model::FormulaType;
use datadog_api_client::datadogV1::model::GeomapWidgetDefinition;
use datadog_api_client::datadogV1::model::GeomapWidgetDefinitionStyle;
use datadog_api_client::datadogV1::model::GeomapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::GeomapWidgetDefinitionView;
use datadog_api_client::datadogV1::model::GeomapWidgetRequest;
use datadog_api_client::datadogV1::model::GeomapWidgetRequestStyle;
use datadog_api_client::datadogV1::model::ListStreamColumn;
use datadog_api_client::datadogV1::model::ListStreamColumnWidth;
use datadog_api_client::datadogV1::model::ListStreamQuery;
use datadog_api_client::datadogV1::model::ListStreamSource;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatMatch;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatMatchType;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatPalette;
use datadog_api_client::datadogV1::model::TableWidgetTextFormatRule;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetComparator;
use datadog_api_client::datadogV1::model::WidgetConditionalFormat;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetFormula;
use datadog_api_client::datadogV1::model::WidgetFormulaSort;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetPalette;
use datadog_api_client::datadogV1::model::WidgetSort;
use datadog_api_client::datadogV1::model::WidgetSortBy;
use datadog_api_client::datadogV1::model::WidgetSortOrderBy;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::ORDERED,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::GeomapWidgetDefinition(
                        Box::new(
                            GeomapWidgetDefinition::new(
                                vec![
                                    GeomapWidgetRequest::new()
                                        .conditional_formats(
                                            vec![
                                                WidgetConditionalFormat::new(
                                                    WidgetComparator::GREATER_THAN,
                                                    WidgetPalette::WHITE_ON_GREEN,
                                                    1000.0,
                                                )
                                            ],
                                        )
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
                                                            FormulaAndFunctionEventsDataSource::RUM,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(
                                                                FormulaAndFunctionEventQueryGroupByConfig
                                                                ::FormulaAndFunctionEventQueryGroupByList(
                                                                    vec![],
                                                                ),
                                                            )
                                                            .indexes(vec!["*".to_string()])
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "@type:session".to_string(),
                                                                ),
                                                            ),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                        .sort(
                                            WidgetSortBy::new()
                                                .count(250)
                                                .order_by(
                                                    vec![
                                                        WidgetSortOrderBy::WidgetFormulaSort(
                                                            Box::new(
                                                                WidgetFormulaSort::new(
                                                                    0,
                                                                    WidgetSort::DESCENDING,
                                                                    FormulaType::FORMULA,
                                                                ),
                                                            ),
                                                        )
                                                    ],
                                                ),
                                        ),
                                    GeomapWidgetRequest::new()
                                        .columns(
                                            vec![
                                                ListStreamColumn::new(
                                                    "@network.client.geoip.location.latitude".to_string(),
                                                    ListStreamColumnWidth::AUTO,
                                                ),
                                                ListStreamColumn::new(
                                                    "@network.client.geoip.location.longitude".to_string(),
                                                    ListStreamColumnWidth::AUTO,
                                                ),
                                                ListStreamColumn::new(
                                                    "@network.client.geoip.country.iso_code".to_string(),
                                                    ListStreamColumnWidth::AUTO,
                                                ),
                                                ListStreamColumn::new(
                                                    "@network.client.geoip.subdivision.name".to_string(),
                                                    ListStreamColumnWidth::AUTO,
                                                )
                                            ],
                                        )
                                        .query(
                                            ListStreamQuery::new(ListStreamSource::LOGS_STREAM, "".to_string())
                                                .indexes(vec![])
                                                .storage("hot".to_string()),
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::EVENT_LIST)
                                        .style(GeomapWidgetRequestStyle::new().color_by("status".to_string()))
                                        .text_formats(
                                            vec![
                                                TableWidgetTextFormatRule::new(
                                                    TableWidgetTextFormatMatch::new(
                                                        TableWidgetTextFormatMatchType::IS,
                                                        "error".to_string(),
                                                    ),
                                                ).palette(TableWidgetTextFormatPalette::WHITE_ON_RED)
                                            ],
                                        )
                                ],
                                GeomapWidgetDefinitionStyle::new("hostmap_blues".to_string(), false),
                                GeomapWidgetDefinitionType::GEOMAP,
                                GeomapWidgetDefinitionView::new("NORTH_AMERICA".to_string()),
                            ).title("Log Count by Service and Source".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(6, 12, 0, 0))
            ],
        )
            .description(Some("Example-Dashboard".to_string()))
            .notify_list(Some(vec![]))
            .reflow_type(DashboardReflowType::FIXED)
            .tags(Some(vec![]))
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
