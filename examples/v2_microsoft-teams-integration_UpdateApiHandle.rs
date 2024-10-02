// Update handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleType;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateApiHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateApiHandleRequestData;

#[tokio::main]
async fn main() {
    let body =
        MicrosoftTeamsUpdateApiHandleRequest::new(MicrosoftTeamsUpdateApiHandleRequestData::new(
            MicrosoftTeamsApiHandleAttributes::new()
                .channel_id("fake-channel-id".to_string())
                .name("fake-handle-name".to_string())
                .team_id("00000000-0000-0000-0000-000000000000".to_string())
                .tenant_id("00000000-0000-0000-0000-000000000001".to_string()),
            MicrosoftTeamsApiHandleType::HANDLE,
        ));
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api.update_api_handle("handle_id".to_string(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
