// Create a target audience returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceCreateRequest;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceCreateRequestAttributes;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceCreateRequestData;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceType;

#[tokio::main]
async fn main() {
    let body =
        GoogleChatTargetAudienceCreateRequest::new(GoogleChatTargetAudienceCreateRequestData::new(
            GoogleChatTargetAudienceCreateRequestAttributes::new(
                "fake-audience-id-1".to_string(),
                "fake audience name 1".to_string(),
            ),
            GoogleChatTargetAudienceType::GOOGLE_CHAT_TARGET_AUDIENCE_TYPE,
        ));
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api
        .create_google_chat_target_audience("organization_binding_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
