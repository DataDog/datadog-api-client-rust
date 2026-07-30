// Get an integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_integration_accounts::IntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountIntegrationId;
use datadog_api_client::datadogV2::model::IntegrationAccountInterfaceId;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.GetIntegrationAccount", true);
    let api = IntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .get_integration_account(
            IntegrationAccountIntegrationId::TWILIO,
            IntegrationAccountInterfaceId::TWILIO,
            "account_id".to_string(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
