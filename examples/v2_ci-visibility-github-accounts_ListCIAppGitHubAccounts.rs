// List GitHub CI Visibility status returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ci_visibility_git_hub_accounts::CIVisibilityGitHubAccountsAPI;

#[tokio::main]
async fn main() {
    let configuration = datadog::Configuration::new();
    let api = CIVisibilityGitHubAccountsAPI::with_config(configuration);
    let resp = api.list_ci_app_git_hub_accounts().await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
