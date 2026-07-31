// Create a Twilio integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_twilio_integration_accounts::TwilioIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TwilioAuthentication;
use datadog_api_client::datadogV2::model::TwilioBasicAuth;
use datadog_api_client::datadogV2::model::TwilioBasicAuthType;
use datadog_api_client::datadogV2::model::TwilioDataflow;
use datadog_api_client::datadogV2::model::TwilioDataflowId;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountAttributes;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountRequest;
use datadog_api_client::datadogV2::model::TwilioInterface;
use datadog_api_client::datadogV2::model::TwilioInterfaceType;
use datadog_api_client::datadogV2::model::TwilioSettings;

#[tokio::main]
async fn main() {
    let body = TwilioIntegrationAccountRequest::new(TwilioIntegrationAccountCreateData::new(
        TwilioIntegrationAccountAttributes::new(
            TwilioInterface::new(
                TwilioAuthentication::TwilioBasicAuth(Box::new(TwilioBasicAuth::new(
                    "SKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                    "your-api-key-secret".to_string(),
                    TwilioBasicAuthType::BASIC,
                ))),
                TwilioInterfaceType::TWILIO,
            )
            .dataflows(vec![
                TwilioDataflow::new(TwilioDataflowId::MESSAGES_LOGS).enabled(true)
            ])
            .settings(
                TwilioSettings::new("ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string())
                    .censor_logs(true),
            ),
            "twilio-prod".to_string(),
        ),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTwilioIntegrationAccount", true);
    let api = TwilioIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .create_twilio_integration_account(TwilioInterfaceType::TWILIO, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
