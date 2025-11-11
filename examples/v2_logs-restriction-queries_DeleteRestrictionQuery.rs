// Delete a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api
        .delete_restriction_query("restriction_query_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
