// List Snowflake integration accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_snowflake_integration::SnowflakeIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListSnowflakeIntegrationAccounts", true);
    let api = SnowflakeIntegrationAPI::with_config(configuration);
    let resp = api.list_snowflake_integration_accounts().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
