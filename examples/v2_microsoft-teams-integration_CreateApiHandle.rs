// Create handle returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleRequestAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleRequestData;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleType;
use datadog_api_client::datadogV2::model::MicrosoftTeamsCreateApiHandleRequest;

#[tokio::main]
async fn main() {
    let body = MicrosoftTeamsCreateApiHandleRequest::new(MicrosoftTeamsApiHandleRequestData::new(
        MicrosoftTeamsApiHandleRequestAttributes::new(
            "fake-channel-id".to_string(),
            "fake-handle-name".to_string(),
            "00000000-0000-0000-0000-000000000000".to_string(),
            "00000000-0000-0000-0000-000000000001".to_string(),
        ),
        MicrosoftTeamsApiHandleType::HANDLE,
    ));
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api.create_api_handle(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
