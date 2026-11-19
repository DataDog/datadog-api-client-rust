// Remove an owner from a Model Lab project returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_model_lab_api::ModelLabAPIAPI;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.remove_model_lab_project_owner", true);
    let api = ModelLabAPIAPI::with_config(configuration);
    let resp = api
        .remove_model_lab_project_owner(
            1,
            Uuid::parse_str("00000000-0000-0000-0000-000000000001").expect("invalid UUID"),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
