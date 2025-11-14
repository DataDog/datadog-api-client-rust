// List roles for a restriction query returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::ListRestrictionQueryRolesOptionalParams;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;

#[tokio::main]
async fn main() {
    // there is a valid "restriction_query" in the system
    let restriction_query_data_id = std::env::var("RESTRICTION_QUERY_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api
        .list_restriction_query_roles(
            restriction_query_data_id.clone(),
            ListRestrictionQueryRolesOptionalParams::default(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
