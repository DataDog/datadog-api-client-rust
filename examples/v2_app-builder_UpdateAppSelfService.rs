// Update App Self-Service Status returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_app_builder::AppBuilderAPI;
use datadog_api_client::datadogV2::model::AppSelfServiceType;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequest;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequestData;
use datadog_api_client::datadogV2::model::UpdateAppSelfServiceRequestDataAttributes;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id =
        uuid::Uuid::parse_str(&std::env::var("APP_DATA_ID").unwrap()).expect("Invalid UUID");
    let body = UpdateAppSelfServiceRequest::new().data(
        UpdateAppSelfServiceRequestData::new()
            .attributes(UpdateAppSelfServiceRequestDataAttributes::new(true))
            .type_(AppSelfServiceType::SELFSERVICE),
    );
    let configuration = datadog::Configuration::new();
    let api = AppBuilderAPI::with_config(configuration);
    let resp = api.update_app_self_service(app_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
