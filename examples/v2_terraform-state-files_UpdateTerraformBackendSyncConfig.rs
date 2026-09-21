// Update a Terraform backend sync configuration returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_terraform_state_files::TerraformStateFilesAPI;
use datadog_api_client::datadogV2::model::TerraformBackendType;
use datadog_api_client::datadogV2::model::TerraformBackendUpdateAttributes;
use datadog_api_client::datadogV2::model::TerraformBackendUpdateData;
use datadog_api_client::datadogV2::model::TerraformBackendUpdateRequest;

#[tokio::main]
async fn main() {
    let body = TerraformBackendUpdateRequest::new(TerraformBackendUpdateData::new(
        TerraformBackendUpdateAttributes::new(vec!["terraform-state-bucket".to_string()]),
        "9007199254740993".to_string(),
        TerraformBackendType::TERRAFORM_BACKENDS,
    ));
    let configuration = datadog::Configuration::new();
    let api = TerraformStateFilesAPI::with_config(configuration);
    let resp = api
        .update_terraform_backend_sync_config("9007199254740993".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
