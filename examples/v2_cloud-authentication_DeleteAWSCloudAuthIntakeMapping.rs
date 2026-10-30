// Delete an AWS cloud authentication intake mapping returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;

#[tokio::main]
async fn main() {
    // there is a valid "aws_cloud_auth_intake_mapping" in the system
    let aws_cloud_auth_intake_mapping_data_id =
        std::env::var("AWS_CLOUD_AUTH_INTAKE_MAPPING_DATA_ID").unwrap();
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.delete_aws_cloud_auth_intake_mapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api
        .delete_aws_cloud_auth_intake_mapping(aws_cloud_auth_intake_mapping_data_id.clone())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
