// Create a new dashboard returns "OK" response
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
            "Example-Dashboard with Profile Metrics Query".to_string(),
            vec![
                Widget::new(
                    WidgetDefinition::TimeseriesWidgetDefinition(
                        Box::new(
                            TimeseriesWidgetDefinition::new(
                                vec![
                                    TimeseriesWidgetRequest
                                    ::new().profile_metrics_query(
                                        LogQueryDefinition::new()
                                            .compute(
                                                LogsQueryCompute::new(
                                                    "sum".to_string(),
                                                ).facet("@prof_core_cpu_cores".to_string()),
                                            )
                                            .group_by(
                                                vec![
                                                    LogQueryDefinitionGroupBy::new("service".to_string())
                                                        .limit(10)
                                                        .sort(
                                                            LogQueryDefinitionGroupBySort::new(
                                                                "sum".to_string(),
                                                                WidgetSort::DESCENDING,
                                                            ).facet("@prof_core_cpu_cores".to_string()),
                                                        )
                                                ],
                                            )
                                            .search(LogQueryDefinitionSearch::new("runtime:jvm".to_string())),
                                    )
                                ],
                                TimeseriesWidgetDefinitionType::TIMESERIES,
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
