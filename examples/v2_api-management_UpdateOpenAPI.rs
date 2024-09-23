// Update an API returns "API updated successfully" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_api_management::APIManagementAPI;
use datadog_api_client::datadogV2::api_api_management::UpdateOpenAPIOptionalParams;
use std::fs;

#[tokio::main]
async fn main() {
    // there is a valid "managed_api" in the system
    let managed_api_data_id = uuid::Uuid::parse_str(&std::env::var("MANAGED_API_DATA_ID").unwrap())
        .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateOpenAPI", true);
    let api = APIManagementAPI::with_config(configuration);
    let resp = api
        .update_open_api(
            managed_api_data_id.clone(),
            UpdateOpenAPIOptionalParams::default()
                .openapi_spec_file(fs::read("openapi-spec.yaml").unwrap()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
