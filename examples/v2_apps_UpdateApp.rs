// Update App returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_apps::AppsAPI;
use datadog_api_client::datadogV2::model::UpdateAppRequest;
use datadog_api_client::datadogV2::model::UpdateAppRequestData;
use datadog_api_client::datadogV2::model::UpdateAppRequestDataAttributes;
use datadog_api_client::datadogV2::model::UpdateAppRequestDataType;

#[tokio::main]
async fn main() {
    // there is a valid "app" in the system
    let app_data_id = std::env::var("APP_DATA_ID").unwrap();
    let body = UpdateAppRequest::new().data(
        UpdateAppRequestData::new(UpdateAppRequestDataType::APPDEFINITIONS)
            .attributes(
                UpdateAppRequestDataAttributes::new()
                    .name("Updated Name".to_string())
                    .root_instance_name("grid0".to_string()),
            )
            .id(app_data_id.clone()),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateApp", true);
    let api = AppsAPI::with_config(configuration);
    let resp = api.update_app(app_data_id.clone(), body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
