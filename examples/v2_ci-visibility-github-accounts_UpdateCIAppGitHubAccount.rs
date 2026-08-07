// Update GitHub CI Visibility status returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_ci_visibility_git_hub_accounts::CIVisibilityGitHubAccountsAPI;
use datadog_api_client::datadogV2::model::CIAppGitHubAccountType;
use datadog_api_client::datadogV2::model::CIAppGitHubAccountUpdateRequest;
use datadog_api_client::datadogV2::model::CIAppGitHubAccountUpdateRequestAttributes;
use datadog_api_client::datadogV2::model::CIAppGitHubAccountUpdateRequestData;
use datadog_api_client::datadogV2::model::CIAppGitHubAccountUpdateRequestRepository;

#[tokio::main]
async fn main() {
    let body = CIAppGitHubAccountUpdateRequest::new(CIAppGitHubAccountUpdateRequestData::new(
        CIAppGitHubAccountUpdateRequestAttributes::new("datadog".to_string())
            .enabled(true)
            .host("github.com".to_string())
            .repository(CIAppGitHubAccountUpdateRequestRepository::new(
                true,
                "shopist".to_string(),
            )),
        CIAppGitHubAccountType::CI_GITHUB_ACCOUNT,
    ));
    let configuration = datadog::Configuration::new();
    let api = CIVisibilityGitHubAccountsAPI::with_config(configuration);
    let resp = api.update_ci_app_git_hub_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
