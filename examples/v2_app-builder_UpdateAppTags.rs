// Update App Tags returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppTagsType;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequest;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequestData;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequestDataAttributes;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let body = UpdateAppTagsRequest::new().data(
        UpdateAppTagsRequestData::new()
            .attributes(UpdateAppTagsRequestDataAttributes::new(vec![
                "team:platform".to_string(),
                "service:ops".to_string(),
            ]))
            .type_(AppTagsType::TAGS),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .update_app_tags(
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
