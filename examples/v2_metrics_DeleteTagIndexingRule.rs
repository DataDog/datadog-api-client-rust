// Delete a tag indexing rule returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_metrics::MetricsAPI;

#[tokio::main]
async fn main() {
    // there is a valid "tag_indexing_rule" in the system
    let tag_indexing_rule_data_id = std::env::var("TAG_INDEXING_RULE_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteTagIndexingRule", true);
    let api = MetricsAPI::with_config(configuration);
    let resp = api
        .delete_tag_indexing_rule(tag_indexing_rule_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
