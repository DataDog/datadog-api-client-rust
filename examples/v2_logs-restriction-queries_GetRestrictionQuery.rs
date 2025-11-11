// Get a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "restriction_query" in the system
    let restriction_query_data_id = std::env::var("RESTRICTION_QUERY_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api
        .get_restriction_query(restriction_query_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
