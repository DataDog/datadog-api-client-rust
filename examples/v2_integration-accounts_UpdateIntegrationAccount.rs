// Update an integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_integration_accounts::IntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountIntegrationId;
use datadog_api_client::datadogV2::model::IntegrationAccountIntegrationUpdate;
use datadog_api_client::datadogV2::model::IntegrationAccountInterfaceId;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::IntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::IntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::IntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::TwilioAuthentication;
use datadog_api_client::datadogV2::model::TwilioBasicAuth;
use datadog_api_client::datadogV2::model::TwilioBasicAuthType;
use datadog_api_client::datadogV2::model::TwilioDataflow;
use datadog_api_client::datadogV2::model::TwilioDataflowId;
use datadog_api_client::datadogV2::model::TwilioIntegrationType;
use datadog_api_client::datadogV2::model::TwilioIntegrationUpdate;
use datadog_api_client::datadogV2::model::TwilioInterfaceType;
use datadog_api_client::datadogV2::model::TwilioInterfaceUpdate;
use datadog_api_client::datadogV2::model::TwilioSettingsUpdate;

#[tokio::main]
async fn main() {
    let body = IntegrationAccountUpdateRequest::new(IntegrationAccountUpdateData::new(
        IntegrationAccountUpdateAttributes::new()
            .integration(
                IntegrationAccountIntegrationUpdate::TwilioIntegrationUpdate(Box::new(
                    TwilioIntegrationUpdate::new(TwilioIntegrationType::TWILIO).interface(
                        TwilioInterfaceUpdate::new(TwilioInterfaceType::TWILIO)
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
                            .settings(
                                TwilioSettingsUpdate::new()
                                    .account_sid("ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string())
                                    .censor_logs(true),
                            ),
                    ),
                )),
            )
            .name("twilio-prod".to_string()),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateIntegrationAccount", true);
    let api = IntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_integration_account(
            IntegrationAccountIntegrationId::TWILIO,
            IntegrationAccountInterfaceId::TWILIO,
            "account_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
