// List tag indexing rules for a metric returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListTagIndexingRulesForMetric", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .list_tag_indexing_rules_for_metric("ExampleMetric".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
