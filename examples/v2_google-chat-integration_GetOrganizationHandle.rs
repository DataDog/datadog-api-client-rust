// Get organization handle returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_google_chat_integration::GoogleChatIntegrationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "organization_handle" in the system
    let organization_handle_data_id = std::env::var("ORGANIZATION_HANDLE_DATA_ID").unwrap();
    let configuration = datadog::Configuration::new();
    let api = GoogleChatIntegrationAPI::with_config(configuration);
    let resp = api
        .get_organization_handle(
            "e54cb570-c674-529c-769d-84b312288ed7".to_string(),
            organization_handle_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
