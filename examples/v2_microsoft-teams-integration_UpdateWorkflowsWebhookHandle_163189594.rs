// Update workflow webhook handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateWorkflowsWebhookHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateWorkflowsWebhookHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsWorkflowsWebhookHandleType;

#[tokio::main]
async fn main() {
    // there is a valid "workflows_webhook_handle" in the system
    let workflows_webhook_handle_data_id =
        std::env::var("WORKFLOWS_WEBHOOK_HANDLE_DATA_ID").unwrap();
    let body = MicrosoftTeamsUpdateWorkflowsWebhookHandleRequest::new(
        MicrosoftTeamsUpdateWorkflowsWebhookHandleRequestData::new(
            MicrosoftTeamsWorkflowsWebhookHandleAttributes::new()
                .name("fake-handle-name--updated".to_string()),
            MicrosoftTeamsWorkflowsWebhookHandleType::WORKFLOWS_WEBHOOK_HANDLE,
        ),
    );
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_workflows_webhook_handle(workflows_webhook_handle_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
