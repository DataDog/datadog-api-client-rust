// Read an owned temporary grant request
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_usage_metering::UsageMeteringAPI;

#[tokio::main]
async fn main() {
    // there is a valid "temporary_grant_request" in the system
    let temporary_grant_request_data_id =
        uuid::Uuid::parse_str(&std::env::var("TEMPORARY_GRANT_REQUEST_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.public_create_temporary_grant_request", true);
    configuration.set_unstable_operation_enabled("v2.public_get_my_temporary_grant_request", true);
    let api = UsageMeteringAPI::with_config(configuration);
    let resp = api
        .public_get_my_temporary_grant_request(temporary_grant_request_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
