// Delete an AWS cloud authentication persona mapping returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.DeleteAWSCloudAuthPersonaMapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api
        .delete_aws_cloud_auth_persona_mapping("persona_mapping_id".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
