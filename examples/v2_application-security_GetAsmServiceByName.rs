// Get Application Security details for a service returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_application_security::ApplicationSecurityAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetAsmServiceByName", true);
    let api = ApplicationSecurityAPI::with_config(configuration);
    let resp = api
        .get_asm_service_by_name("service_filter".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
