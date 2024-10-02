// Update api handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_microsoft_teams_integration::MicrosoftTeamsIntegrationAPI;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleAttributes;
use datadog_api_client::datadogV2::model::MicrosoftTeamsApiHandleType;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateApiHandleRequest;
use datadog_api_client::datadogV2::model::MicrosoftTeamsUpdateApiHandleRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "api_handle" in the system
    let api_handle_data_id = std::env::var("API_HANDLE_DATA_ID").unwrap();
    let body =
        MicrosoftTeamsUpdateApiHandleRequest::new(MicrosoftTeamsUpdateApiHandleRequestData::new(
            MicrosoftTeamsApiHandleAttributes::new().name("fake-handle-name--updated".to_string()),
            MicrosoftTeamsApiHandleType::HANDLE,
        ));
    let configuration = datadog::Configuration::new();
    let api = MicrosoftTeamsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_api_handle(api_handle_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
