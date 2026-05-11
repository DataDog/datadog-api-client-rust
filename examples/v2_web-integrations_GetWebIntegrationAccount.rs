// Get a web integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_web_integrations::WebIntegrationsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetWebIntegrationAccount", true);
    let api = WebIntegrationsAPI::with_config(configuration);
    let resp = api
        .get_web_integration_account("integration_name".to_string(), "account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
