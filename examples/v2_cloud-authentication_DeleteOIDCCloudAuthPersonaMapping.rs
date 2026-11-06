// Delete a Custom OIDC persona mapping returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "oidc_cloud_auth_persona_mapping" in the system
    let oidc_cloud_auth_persona_mapping_data_id =
        uuid::Uuid::parse_str(&std::env::var("OIDC_CLOUD_AUTH_PERSONA_MAPPING_DATA_ID").unwrap())
            .expect("Invalid UUID");
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_oidc_cloud_auth_persona_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api
        .delete_oidc_cloud_auth_persona_mapping(oidc_cloud_auth_persona_mapping_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
