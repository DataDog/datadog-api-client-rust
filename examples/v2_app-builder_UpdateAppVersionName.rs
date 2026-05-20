// Name App Version returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppVersionNameType;
use datadog_api_client::datadogV2::model::UpdateAppVersionNameRequest;
use datadog_api_client::datadogV2::model::UpdateAppVersionNameRequestData;
use datadog_api_client::datadogV2::model::UpdateAppVersionNameRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id =
        uuid::Uuid::parse_str(&std::env::var("APP_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = UpdateAppVersionNameRequest::new().data(
        UpdateAppVersionNameRequestData::new()
            .attributes(UpdateAppVersionNameRequestDataAttributes::new(
                "v1.2.0 - bug fix release".to_string(),
            ))
            .type_(AppVersionNameType::VERSIONNAMES),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api
        .update_app_version_name(app_data_id.clone(), "latest".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
