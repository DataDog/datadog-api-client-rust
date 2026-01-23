// List ServiceNow templates returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_service_now_integration::ServiceNowIntegrationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListServiceNowTemplates", true);
    let api = ServiceNowIntegrationAPI::with_config(configuration);
    let resp = api.list_service_now_templates().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
