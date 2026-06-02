// Query aggregated long tasks returns "Successful response" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_insights::RUMInsightsAPI;
use datadog_api_client::datadogV2::model::AggregatedLongTasksRequest;
use datadog_api_client::datadogV2::model::AggregatedLongTasksRequestAttributes;
use datadog_api_client::datadogV2::model::AggregatedLongTasksRequestData;
use datadog_api_client::datadogV2::model::AggregatedLongTasksRequestType;
use datadog_api_client::datadogV2::model::AggregatedWaterfallPerformanceCriteria;
use datadog_api_client::datadogV2::model::AggregatedWaterfallPerformanceCriteriaMetric;

#[tokio::main]
async fn main() {
    let body = AggregatedLongTasksRequest::new(AggregatedLongTasksRequestData::new(
        AggregatedLongTasksRequestAttributes::new(
            "ccbc53b1-74f2-496b-bdd7-9a8fa7b7376b".to_string(),
            1762437564,
            20,
            1762523964,
            "/account/login(/:type)".to_string(),
        )
        .criteria(
            AggregatedWaterfallPerformanceCriteria::new(
                AggregatedWaterfallPerformanceCriteriaMetric::LARGEST_CONTENTFUL_PAINT,
            )
            .max(5.0 as f64)
            .min(2.5 as f64),
        )
        .filter("@session.type:user".to_string()),
        AggregatedLongTasksRequestType::AGGREGATED_LONG_TASKS,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryAggregatedLongTasks", true);
    let api = RUMInsightsAPI::with_config(configuration);
    let resp = api.query_aggregated_long_tasks(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
