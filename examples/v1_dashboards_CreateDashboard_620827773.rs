// Create a new dashboard with topology_map custom widget
use datadog_api_client::datadog;
use datadog_api_client::datadogV1::api_dashboards::DashboardsAPI;
use datadog_api_client::datadogV1::model::Dashboard;
use datadog_api_client::datadogV1::model::DashboardLayoutType;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricAggregation;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricDataSource;
use datadog_api_client::datadogV1::model::FormulaAndFunctionMetricQueryDefinition;
use datadog_api_client::datadogV1::model::FormulaAndFunctionQueryDefinition;
use datadog_api_client::datadogV1::model::NumberFormatUnit;
use datadog_api_client::datadogV1::model::NumberFormatUnitCanonical;
use datadog_api_client::datadogV1::model::NumberFormatUnitScaleType;
use datadog_api_client::datadogV1::model::TopologyMapInlineColumnKindScalar;
use datadog_api_client::datadogV1::model::TopologyMapInlineColumnKindString;
use datadog_api_client::datadogV1::model::TopologyMapInlineData;
use datadog_api_client::datadogV1::model::TopologyMapInlineEdgeColumn;
use datadog_api_client::datadogV1::model::TopologyMapInlineEdgeColumnId;
use datadog_api_client::datadogV1::model::TopologyMapInlineEdgesData;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeColumn;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeIdColumn;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeIdColumnId;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeStringColumn;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeStringColumnId;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeValueColumn;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodeValueColumnMeta;
use datadog_api_client::datadogV1::model::TopologyMapInlineNodesData;
use datadog_api_client::datadogV1::model::TopologyMapLayout;
use datadog_api_client::datadogV1::model::TopologyMapLayoutDirection;
use datadog_api_client::datadogV1::model::TopologyMapLayoutType;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinition;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinitionCustom;
use datadog_api_client::datadogV1::model::TopologyMapWidgetDefinitionType;
use datadog_api_client::datadogV1::model::TopologyRequestCustom;
use datadog_api_client::datadogV1::model::TopologyRequestCustomProjection;
use datadog_api_client::datadogV1::model::TopologyRequestCustomProjectionDimension;
use datadog_api_client::datadogV1::model::TopologyRequestCustomProjectionType;
use datadog_api_client::datadogV1::model::TopologyRequestCustomResponseFormat;
use datadog_api_client::datadogV1::model::Widget;
use datadog_api_client::datadogV1::model::WidgetDefinition;
use datadog_api_client::datadogV1::model::WidgetFormula;
use datadog_api_client::datadogV1::model::WidgetLayout;
use datadog_api_client::datadogV1::model::WidgetTextAlign;

#[tokio::main]
async fn main() {
    let body =
        Dashboard::new(
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TopologyMapWidgetDefinition(
                        Box::new(
                            TopologyMapWidgetDefinition::TopologyMapWidgetDefinitionCustom(
                                Box::new(
                                    TopologyMapWidgetDefinitionCustom::new(
                                        TopologyMapLayout::new(
                                            TopologyMapLayoutDirection::LEFT_TO_RIGHT,
                                            TopologyMapLayoutType::HIERARCHICAL,
                                        ),
                                        vec![
                                            TopologyRequestCustom::new(
                                                TopologyRequestCustomProjection::new(
                                                    vec![
                                                        TopologyRequestCustomProjectionDimension::new(
                                                            "a".to_string(),
                                                            "values[0]".to_string(),
                                                        )
                                                    ],
                                                    TopologyRequestCustomProjectionType::CUSTOM_TOPOLOGY_MAP_NODE,
                                                ),
                                                vec![
                                                    FormulaAndFunctionQueryDefinition
                                                    ::FormulaAndFunctionMetricQueryDefinition(
                                                        Box::new(
                                                            FormulaAndFunctionMetricQueryDefinition::new(
                                                                FormulaAndFunctionMetricDataSource::METRICS,
                                                                "a".to_string(),
                                                                "avg:system.cpu.user{*} by {service}".to_string(),
                                                            ).aggregator(FormulaAndFunctionMetricAggregation::AVG),
                                                        ),
                                                    )
                                                ],
                                                TopologyRequestCustomResponseFormat::SCALAR,
                                            ).formulas(vec![WidgetFormula::new("a".to_string())])
                                        ],
                                        TopologyMapWidgetDefinitionType::TOPOLOGY_MAP,
                                    )
                                        .data(
                                            TopologyMapInlineData::new()
                                                .edges(
                                                    TopologyMapInlineEdgesData::new(
                                                        vec![
                                                            TopologyMapInlineEdgeColumn::new(
                                                                TopologyMapInlineEdgeColumnId::SOURCE,
                                                                TopologyMapInlineColumnKindString::STRING,
                                                                vec!["service:web".to_string()],
                                                            ),
                                                            TopologyMapInlineEdgeColumn::new(
                                                                TopologyMapInlineEdgeColumnId::TARGET,
                                                                TopologyMapInlineColumnKindString::STRING,
                                                                vec!["service:db".to_string()],
                                                            )
                                                        ],
                                                    ),
                                                )
                                                .nodes(
                                                    TopologyMapInlineNodesData::new(
                                                        vec![
                                                            TopologyMapInlineNodeColumn::TopologyMapInlineNodeIdColumn(
                                                                Box::new(
                                                                    TopologyMapInlineNodeIdColumn::new(
                                                                        TopologyMapInlineNodeIdColumnId::ID,
                                                                        TopologyMapInlineColumnKindString::STRING,
                                                                        vec![
                                                                            "service:web".to_string(),
                                                                            "service:db".to_string()
                                                                        ],
                                                                    ),
                                                                ),
                                                            ),
                                                            TopologyMapInlineNodeColumn
                                                            ::TopologyMapInlineNodeStringColumn(
                                                                Box::new(
                                                                    TopologyMapInlineNodeStringColumn::new(
                                                                        TopologyMapInlineNodeStringColumnId::SUBTITLE,
                                                                        TopologyMapInlineColumnKindString::STRING,
                                                                        vec![Some("api-team".to_string()), None],
                                                                    ),
                                                                ),
                                                            ),
                                                            TopologyMapInlineNodeColumn
                                                            ::TopologyMapInlineNodeValueColumn(
                                                                Box::new(
                                                                    TopologyMapInlineNodeValueColumn::new(
                                                                        "values[1]".to_string(),
                                                                        TopologyMapInlineColumnKindScalar::SCALAR,
                                                                        vec![Some(1.5 as f64), None],
                                                                    ).meta(
                                                                        TopologyMapInlineNodeValueColumnMeta
                                                                        ::new().unit(
                                                                            NumberFormatUnit::NumberFormatUnitCanonical(
                                                                                Box::new(
                                                                                    NumberFormatUnitCanonical::new()
                                                                                        .type_(
                                                                                            NumberFormatUnitScaleType
                                                                                            ::CANONICAL_UNIT,
                                                                                        )
                                                                                        .unit_name(
                                                                                            "second".to_string(),
                                                                                        ),
                                                                                ),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                            )
                                                        ],
                                                    ),
                                                ),
                                        )
                                        .title("".to_string())
                                        .title_align(WidgetTextAlign::LEFT)
                                        .title_size("16".to_string()),
                                ),
                            ),
                        ),
                    ),
                ).layout(WidgetLayout::new(15, 47, 0, 0))
            ],
        )
            .description(Some("".to_string()))
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
