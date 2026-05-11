// List web integration accounts returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_web_integrations::WebIntegrationsAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.ListWebIntegrationAccounts", true);
    let api = WebIntegrationsAPI::with_config(configuration);
    let resp = api
        .list_web_integration_accounts("integration_name".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
