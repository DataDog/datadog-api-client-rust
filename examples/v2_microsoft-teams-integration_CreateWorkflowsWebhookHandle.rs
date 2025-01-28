// Create Workflows webhook handle returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsCreateWorkflowsWebhookHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleRequestAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleType;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsCreateWorkflowsWebhookHandleRequest::new(
        MicrosoftTeamsWorkflowsWebhookHandleRequestData::new(
            MicrosoftTeamsWorkflowsWebhookHandleRequestAttributes::new(
                "fake-handle-name".to_string(),
                "https://fake.url.com".to_string(),
            ),
            MicrosoftTeamsWorkflowsWebhookHandleType::WORKFLOWS_WEBHOOK_HANDLE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api.create_workflows_webhook_handle(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
