// Update a Twilio integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_twilio_integration_accounts::TwilioIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthUpdate;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::TwilioAlertsLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioCallSummariesLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioEventsLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::TwilioIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::TwilioIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::TwilioMessagesLogsIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body = TwilioIntegrationAccountUpdateRequest::new(TwilioIntegrationAccountUpdateData::new(
        TwilioIntegrationAccountUpdateAttributes::new()
            .authentication(
                TwilioIntegrationAccountAuthenticationUpdate::IntegrationAccountBasicAuthUpdate(
                    Box::new(
                        IntegrationAccountBasicAuthUpdate::new(
                            IntegrationAccountBasicAuthType::BASIC,
                        )
                        .password("your-password".to_string())
                        .username("datadog".to_string()),
                    ),
                ),
            )
            .dataflows(
                TwilioIntegrationDataflowsRequest::new()
                    .twilio_alerts_logs(
                        TwilioAlertsLogsIntegrationDataflowRequest::new().enabled(true),
                    )
                    .twilio_call_summaries_logs(
                        TwilioCallSummariesLogsIntegrationDataflowRequest::new().enabled(true),
                    )
                    .twilio_cloud_cost_metrics(
                        TwilioCloudCostMetricsIntegrationDataflowRequest::new().enabled(true),
                    )
                    .twilio_events_logs(
                        TwilioEventsLogsIntegrationDataflowRequest::new().enabled(true),
                    )
                    .twilio_messages_logs(
                        TwilioMessagesLogsIntegrationDataflowRequest::new().enabled(true),
                    ),
            )
            .name("twilio-prod".to_string())
            .settings(
                TwilioIntegrationAccountSettingsUpdate::new()
                    .account_sid("ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string())
                    .censor_logs(true),
            ),
        "953a0060-81ec-4221-aed4-d4733b59cd96".to_string(),
        IntegrationAccountType::INTEGRATION_ACCOUNT,
    ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateTwilioIntegrationAccount", true);
    let api = TwilioIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_twilio_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
