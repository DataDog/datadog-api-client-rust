// Delete a Twilio integration account returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_twilio_integration_accounts::TwilioIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::TwilioInterfaceType;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteTwilioIntegrationAccount", true);
    let api = TwilioIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .delete_twilio_integration_account(TwilioInterfaceType::TWILIO, "account_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
