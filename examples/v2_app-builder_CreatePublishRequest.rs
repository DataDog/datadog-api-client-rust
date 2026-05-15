// Create Publish Request returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::CreatePublishRequestRequest;
use datadog_api_client::datadogV2::model::CreatePublishRequestRequestData;
use datadog_api_client::datadogV2::model::CreatePublishRequestRequestDataAttributes;
use datadog_api_client::datadogV2::model::PublishRequestType;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = CreatePublishRequestRequest::new().data(
        CreatePublishRequestRequestData::new()
            .attributes(
                CreatePublishRequestRequestDataAttributes::new(
                    "Release v1.2 to production".to_string(),
                )
                .description("Adds new dashboard widgets and a few bug fixes.".to_string()),
            )
            .type_(PublishRequestType::PUBLISHREQUEST),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .create_publish_request(
            Uuid::parse_str("00000000-0000-0000-0000-000000000000").expect("invalid UUID"),
            body,
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
