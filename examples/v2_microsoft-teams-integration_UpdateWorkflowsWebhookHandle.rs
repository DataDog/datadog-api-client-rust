// Update Workflows webhook handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateWorkflowsWebhookHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateWorkflowsWebhookHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleType;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsUpdateWorkflowsWebhookHandleRequest::new(
        MicrosoftTeamsUpdateWorkflowsWebhookHandleRequestData::new(
            MicrosoftTeamsWorkflowsWebhookHandleAttributes::new()
                .name("fake-handle-name".to_string())
                .url("https://fake.url.com".to_string()),
            MicrosoftTeamsWorkflowsWebhookHandleType::WORKFLOWS_WEBHOOK_HANDLE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_workflows_webhook_handle("handle_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
