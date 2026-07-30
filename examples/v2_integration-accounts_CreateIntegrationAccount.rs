// Create an integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_integration_accounts::IntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountAttributes;
use datadog_api_client::datadogV2::model::IntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::IntegrationAccountIntegration;
use datadog_api_client::datadogV2::model::IntegrationAccountIntegrationId;
use datadog_api_client::datadogV2::model::IntegrationAccountInterfaceId;
use datadog_api_client::datadogV2::model::IntegrationAccountRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TwilioAuthentication;
use datadog_api_client::datadogV2::model::TwilioBasicAuth;
use datadog_api_client::datadogV2::model::TwilioBasicAuthType;
use datadog_api_client::datadogV2::model::TwilioDataflow;
use datadog_api_client::datadogV2::model::TwilioDataflowId;
use datadog_api_client::datadogV2::model::TwilioIntegration;
use datadog_api_client::datadogV2::model::TwilioIntegrationType;
use datadog_api_client::datadogV2::model::TwilioInterface;
use datadog_api_client::datadogV2::model::TwilioInterfaceType;
use datadog_api_client::datadogV2::model::TwilioSettings;

#[tokio::main]
async fn main() {
    let body = IntegrationAccountRequest::new(IntegrationAccountCreateData::new(
        IntegrationAccountAttributes::new(
            IntegrationAccountIntegration::TwilioIntegration(Box::new(TwilioIntegration::new(
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
                TwilioIntegrationType::TWILIO,
            ))),
            "twilio-prod".to_string(),
        ),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateIntegrationAccount", true);
    let api = IntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .create_integration_account(
            IntegrationAccountIntegrationId::TWILIO,
            IntegrationAccountInterfaceId::TWILIO,
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
