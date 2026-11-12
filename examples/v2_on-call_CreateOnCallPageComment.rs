// Create On-Call Page Comment returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_on_call::OnCallAPI;
use datadog_api_client::datadogV2::model::CreateOnCallPageCommentRequest;
use datadog_api_client::datadogV2::model::CreateOnCallPageCommentRequestAttributes;
use datadog_api_client::datadogV2::model::CreateOnCallPageCommentRequestData;
use datadog_api_client::datadogV2::model::OnCallPageCommentType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreateOnCallPageCommentRequest::new(
        CreateOnCallPageCommentRequestData::new(
            CreateOnCallPageCommentRequestAttributes::new(
                "Comment from the API client integration tests.".to_string(),
            ),
            OnCallPageCommentType::COMMENTS,
        )
        .id(Uuid::parse_str("3c5f5b5e-3a3e-4f8c-9b1d-2f6a7e8d9c0b").expect("invalid UUID")),
    );
    let configuration = datadog::Configuration::new();
    let api = OnCallAPI::with_config(configuration);
    let resp = api
        .create_on_call_page_comment("f63eb677-07ce-46ab-b2e0-80ca1ffe91bf".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
