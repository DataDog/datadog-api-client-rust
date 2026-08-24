// Create a Twilio integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_twilio_integration_accounts::TwilioIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TwilioAlertsLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioEventsLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::TwilioMessagesLogsIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body = TwilioIntegrationAccountCreateRequest::new(TwilioIntegrationAccountCreateData::new(
        TwilioIntegrationAccountCreateAttributes::new(
            TwilioIntegrationAccountAuthenticationRequest::IntegrationAccountBasicAuthRequest(
                Box::new(IntegrationAccountBasicAuthRequest::new(
                    IntegrationAccountBasicAuthType::BASIC,
                    "your-password".to_string(),
                    "datadog".to_string(),
                )),
            ),
            "twilio-prod".to_string(),
            TwilioIntegrationAccountSettingsRequest::new(
                "ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
            )
            .censor_logs(true),
        )
        .dataflows(
            TwilioIntegrationDataflowsRequest::new()
                .twilio_alerts_logs(TwilioAlertsLogsIntegrationDataflowRequest::new().enabled(true))
                .twilio_call_summaries_logs(
                    TwilioCallSummariesLogsIntegrationDataflowRequest::new().enabled(true),
                )
                .twilio_cloud_cost_metrics(
                    TwilioCloudCostMetricsIntegrationDataflowRequest::new().enabled(true),
                )
                .twilio_events_logs(TwilioEventsLogsIntegrationDataflowRequest::new().enabled(true))
                .twilio_messages_logs(
                    TwilioMessagesLogsIntegrationDataflowRequest::new().enabled(true),
                ),
        ),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateTwilioIntegrationAccount", true);
    let api = TwilioIntegrationAccountsAPI::with_config(configuration);
    let resp = api.create_twilio_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
