// Update App Tags returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppTagsType;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequest;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequestData;
use datadog_api_client::datadogV2::model::UpdateAppTagsRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id =
        uuid::Uuid::parse_str(&std::env::var("APP_DATA_ID").unwrap()).expect("Invalid UUID");
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
    let resp = api.update_app_tags(app_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
