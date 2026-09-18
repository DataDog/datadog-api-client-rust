// List Terraform backend sync configurations returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_terraform_state_files::ListTerraformBackendSyncConfigsOptionalParams;
use datadog_api_client::datadogV2::api_terraform_state_files::TerraformStateFilesAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = TerraformStateFilesAPI::with_config(configuration);
    let resp = api
        .list_terraform_backend_sync_configs(
            ListTerraformBackendSyncConfigsOptionalParams::default()
                .account_id("123456789012".to_string()),
        )
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
