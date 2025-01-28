// Get all Workflows webhook handles returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::ListWorkflowsWebhookHandlesOptionalParams;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .list_workflows_webhook_handles(ListWorkflowsWebhookHandlesOptionalParams::default())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
