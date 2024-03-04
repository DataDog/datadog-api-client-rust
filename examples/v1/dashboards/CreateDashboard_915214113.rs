// Create a new dashboard with geomap widget
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
            DashboardLayoutType::FREE,
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::GeomapWidgetDefinition(
                        Box::new(
                            GeomapWidgetDefinition::new(
                                vec![
                                    GeomapWidgetRequest::new()
                                        .formulas(
                                            vec![
                                                WidgetFormula::new(
                                                    "query1".to_string(),
                                                ).limit(
                                                    WidgetFormulaLimit::new().count(250).order(QuerySortOrder::DESC),
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
                                                            FormulaAndFunctionEventsDataSource::RUM,
                                                            "query1".to_string(),
                                                        )
                                                            .group_by(
                                                                vec![
                                                                    FormulaAndFunctionEventQueryGroupBy::new(
                                                                        "@geo.country_iso_code".to_string(),
                                                                    )
                                                                        .limit(250)
                                                                        .sort(
                                                                            FormulaAndFunctionEventQueryGroupBySort
                                                                            ::new(
                                                                                FormulaAndFunctionEventAggregation
                                                                                ::COUNT,
                                                                            ).order(QuerySortOrder::DESC),
                                                                        )
                                                                ],
                                                            )
                                                            .indexes(vec!["*".to_string()])
                                                            .search(
                                                                FormulaAndFunctionEventQueryDefinitionSearch::new(
                                                                    "".to_string(),
                                                                ),
                                                            ),
                                                    ),
                                                )
                                            ],
                                        )
                                        .response_format(FormulaAndFunctionResponseFormat::SCALAR)
                                ],
                                GeomapWidgetDefinitionStyle::new("hostmap_blues".to_string(), false),
                                GeomapWidgetDefinitionType::GEOMAP,
                                GeomapWidgetDefinitionView::new("WORLD".to_string()),
                            )
                                .time(WidgetTime::new())
                                .title("".to_string())
                                .title_align(WidgetTextAlign::LEFT)
                                .title_size("16".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(30, 47, 0, 0))
            ],
        )
            .description(Some(None))
            .is_read_only(false)
            .notify_list(Some(vec![]))
            .template_variables(Some(vec![]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(Some(value)) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
