// Create a new dashboard with split graph widget
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
            "Example-Dashboard".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::SplitGraphWidgetDefinition(
                        Box::new(
                            SplitGraphWidgetDefinition::new(
                                SplitGraphVizSize::MD,
                                SplitGraphSourceWidgetDefinition::TimeseriesWidgetDefinition(
                                    Box::new(
                                        TimeseriesWidgetDefinition::new(
                                            vec![
                                                TimeseriesWidgetRequest::new()
                                                    .display_type(WidgetDisplayType::LINE)
                                                    .queries(
                                                        vec![
                                                            FormulaAndFunctionQueryDefinition
                                                            ::FormulaAndFunctionMetricQueryDefinition(
                                                                Box::new(
                                                                    FormulaAndFunctionMetricQueryDefinition::new(
                                                                        FormulaAndFunctionMetricDataSource::METRICS,
                                                                        "query1".to_string(),
                                                                        "avg:system.cpu.user{*}".to_string(),
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
                                            .title("".to_string())
                                            .title_align(WidgetTextAlign::LEFT)
                                            .title_size("16".to_string()),
                                    ),
                                ),
                                SplitConfig::new(
                                    24,
                                    SplitSort::new(
                                        WidgetSort::DESCENDING,
                                    ).compute(
                                        SplitConfigSortCompute::new("sum".to_string(), "system.cpu.user".to_string()),
                                    ),
                                    vec![SplitDimension::new("service".to_string())],
                                ).static_splits(
                                    vec![
                                        vec![
                                            SplitVectorEntryItem::new(
                                                "service".to_string(),
                                                vec!["cassandra".to_string()],
                                            ),
                                            SplitVectorEntryItem::new("datacenter".to_string(), vec![])
                                        ],
                                        vec![SplitVectorEntryItem::new("demo".to_string(), vec!["env".to_string()])]
                                    ],
                                ),
                                SplitGraphWidgetDefinitionType::SPLIT_GROUP,
                            )
                                .has_uniform_y_axes(true)
                                .title("".to_string()),
                        ),
                    ),
                ).layout(WidgetLayout::new(8, 12, 0, 0))
            ],
        )
            .description(Some("".to_string()))
            .is_read_only(false)
            .notify_list(Some(vec![]))
            .template_variables(Some(vec![]));
    let configuration = Configuration::new();
    let api = DashboardsAPI::with_config(configuration);
    let resp = api.create_dashboard(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
