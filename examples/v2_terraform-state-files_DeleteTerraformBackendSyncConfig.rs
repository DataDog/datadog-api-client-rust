// Delete a Terraform backend sync configuration returns "No Content" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_terraform_state_files::TerraformStateFilesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TerraformStateFilesAPI::with_config(configuration);
    let resp = api
        .delete_terraform_backend_sync_config("9007199254740993".to_string())
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
