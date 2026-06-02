// Query aggregated waterfall returns "Successful response" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_rum_insights::RUMInsightsAPI;
use datadog_api_client::datadogV2::model::AggregatedWaterfallPerformanceCriteria;
use datadog_api_client::datadogV2::model::AggregatedWaterfallPerformanceCriteriaMetric;
use datadog_api_client::datadogV2::model::AggregatedWaterfallRequest;
use datadog_api_client::datadogV2::model::AggregatedWaterfallRequestAttributes;
use datadog_api_client::datadogV2::model::AggregatedWaterfallRequestData;
use datadog_api_client::datadogV2::model::AggregatedWaterfallRequestType;

#[tokio::main]
async fn main() {
    let body = AggregatedWaterfallRequest::new(AggregatedWaterfallRequestData::new(
        AggregatedWaterfallRequestAttributes::new(
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
        .filter("@session.type:user".to_string())
        .include_global_appearance(false),
        AggregatedWaterfallRequestType::AGGREGATED_WATERFALL,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.QueryAggregatedWaterfall", true);
    let api = RUMInsightsAPI::with_config(configuration);
    let resp = api.query_aggregated_waterfall(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
