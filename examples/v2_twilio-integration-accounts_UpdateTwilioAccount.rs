// Update a Twilio integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_twilio_integration_accounts::TwilioIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TwilioAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::TwilioAccountUpdateData;
use datadog_api_client::datadogV2::model::TwilioAccountUpdateRequest;
use datadog_api_client::datadogV2::model::TwilioAuthentication;
use datadog_api_client::datadogV2::model::TwilioBasicAuth;
use datadog_api_client::datadogV2::model::TwilioBasicAuthType;
use datadog_api_client::datadogV2::model::TwilioDataflow;
use datadog_api_client::datadogV2::model::TwilioDataflowId;
use datadog_api_client::datadogV2::model::TwilioSettingsUpdate;

#[tokio::main]
async fn main() {
    let body = TwilioAccountUpdateRequest::new(TwilioAccountUpdateData::new(
        TwilioAccountUpdateAttributes::new()
            .authentication(TwilioAuthentication::TwilioBasicAuth(Box::new(
                TwilioBasicAuth::new(
                    "SKxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
                    "your-api-key-secret".to_string(),
                    TwilioBasicAuthType::BASIC,
                ),
            )))
            .dataflows(vec![
                TwilioDataflow::new(TwilioDataflowId::MESSAGES_LOGS).enabled(true)
            ])
            .name("twilio-prod".to_string())
            .settings(
                TwilioSettingsUpdate::new()
                    .account_sid("ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string())
                    .censor_logs(true),
            ),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateTwilioAccount", true);
    let api = TwilioIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_twilio_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
