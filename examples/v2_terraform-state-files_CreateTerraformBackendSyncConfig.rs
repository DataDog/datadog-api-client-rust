// Create a Terraform backend sync configuration returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_terraform_state_files::TerraformStateFilesAPI;
use datadog_api_client::datadogV2::model::TerraformBackendCreateAttributes;
use datadog_api_client::datadogV2::model::TerraformBackendCreateData;
use datadog_api_client::datadogV2::model::TerraformBackendCreateRequest;
use datadog_api_client::datadogV2::model::TerraformBackendKind;
use datadog_api_client::datadogV2::model::TerraformBackendType;

#[tokio::main]
async fn main() {
    let body = TerraformBackendCreateRequest::new(TerraformBackendCreateData::new(
        TerraformBackendCreateAttributes::new(
            "123456789012".to_string(),
            TerraformBackendKind::TERRAFORM,
            vec!["terraform-state-bucket".to_string()],
            "us-east-1".to_string(),
        ),
        TerraformBackendType::TERRAFORM_BACKENDS,
    ));
    let configuration = datadog::Configuration::new();
    let api = TerraformStateFilesAPI::with_config(configuration);
    let resp = api.create_terraform_backend_sync_config(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
