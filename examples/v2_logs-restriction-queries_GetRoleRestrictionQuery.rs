// Get restriction query for a given role returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_logs_restriction_queries::LogsRestrictionQueriesAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetRoleRestrictionQuery", true);
    let api = LogsRestrictionQueriesAPI::with_config(configuration);
    let resp = api.get_role_restriction_query("role_id".to_string()).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
