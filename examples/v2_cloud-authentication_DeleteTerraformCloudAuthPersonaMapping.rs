// Delete an HCP Terraform persona mapping returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "terraform_cloud_auth_persona_mapping" in the system
    let terraform_cloud_auth_persona_mapping_data_id =
        std::env::var("TERRAFORM_CLOUD_AUTH_PERSONA_MAPPING_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.delete_terraform_cloud_auth_persona_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api
        .delete_terraform_cloud_auth_persona_mapping(
            terraform_cloud_auth_persona_mapping_data_id.clone(),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
