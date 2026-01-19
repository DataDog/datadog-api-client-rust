// Update organization handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;
use datadog_api_client::datadogV2::model::GoogleChatOrganizationHandleType;
use datadog_api_client::datadogV2::model::GoogleChatUpdateOrganizationHandleRequest;
use datadog_api_client::datadogV2::model::GoogleChatUpdateOrganizationHandleRequestAttributes;
use datadog_api_client::datadogV2::model::GoogleChatUpdateOrganizationHandleRequestData;

#[tokio::main]
async fn main() {
    // there is a valid "organization_handle" in the system
    let organization_handle_data_id = std::env::var("ORGANIZATION_HANDLE_DATA_ID").unwrap();
    let body = GoogleChatUpdateOrganizationHandleRequest::new(
        GoogleChatUpdateOrganizationHandleRequestData::new(
            GoogleChatUpdateOrganizationHandleRequestAttributes::new()
                .name("fake-handle-name--updated".to_string()),
        ),
        GoogleChatOrganizationHandleType::GOOGLE_CHAT_ORGANIZATION_HANDLE_TYPE,
    );
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api
        .update_organization_handle(
            "e54cb570-c674-529c-769d-84b312288ed7".to_string(),
            organization_handle_data_id.clone(),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
