// Create organization handle returns "CREATED" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;
use datadog_api_client::datadogV2::model::GoogleChatCreateOrganizationHandleRequest;
use datadog_api_client::datadogV2::model::GoogleChatCreateOrganizationHandleRequestAttributes;
use datadog_api_client::datadogV2::model::GoogleChatCreateOrganizationHandleRequestData;
use datadog_api_client::datadogV2::model::GoogleChatOrganizationHandleType;

#[tokio::main]
async fn main() {
    let body = GoogleChatCreateOrganizationHandleRequest::new(
        GoogleChatCreateOrganizationHandleRequestData::new(
            GoogleChatCreateOrganizationHandleRequestAttributes::new(
                "Example-Google-Chat-Integration".to_string(),
                "spaces/AAQA-zFIks8".to_string(),
            ),
        ),
        GoogleChatOrganizationHandleType::GOOGLE_CHAT_ORGANIZATION_HANDLE_TYPE,
    );
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api
        .create_organization_handle("e54cb570-c674-529c-769d-84b312288ed7".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
