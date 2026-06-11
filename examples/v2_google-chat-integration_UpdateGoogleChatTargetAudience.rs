// Update a target audience returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceType;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceUpdateRequest;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::GoogleChatTargetAudienceUpdateRequestData;

#[tokio::main]
async fn main() {
    let body =
        GoogleChatTargetAudienceUpdateRequest::new(GoogleChatTargetAudienceUpdateRequestData::new(
            GoogleChatTargetAudienceUpdateRequestAttributes::new()
                .audience_id("fake-audience-id-1".to_string())
                .audience_name("fake audience name 1".to_string()),
            GoogleChatTargetAudienceType::GOOGLE_CHAT_TARGET_AUDIENCE_TYPE,
        ));
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api
        .update_google_chat_target_audience(
            "organization_binding_id".to_string(),
            "target_audience_id".to_string(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
