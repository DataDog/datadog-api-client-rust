// Create a GitHub Costs integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_git_hub_costs_integration::GitHubCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthRequest;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        GitHubCostsIntegrationAccountCreateRequest::new(
            GitHubCostsIntegrationAccountCreateData::new(
                GitHubCostsIntegrationAccountCreateAttributes::new(
                    GitHubCostsIntegrationAccountAuthenticationRequest
                    ::GitHubCostsIntegrationAccountBearerTokenAuthRequest(
                        Box::new(
                            GitHubCostsIntegrationAccountBearerTokenAuthRequest::new(
                                GitHubCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                "your-token".to_string(),
                            ),
                        ),
                    ),
                    "github-costs-acme".to_string(),
                    GitHubCostsIntegrationAccountSettingsRequest::new(
                        "acme".to_string(),
                    ).tags("env:prod,team:finops".to_string()),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_git_hub_costs_integration_account", true);
    let api = GitHubCostsIntegrationAPI::with_config(configuration);
    let resp = api.create_git_hub_costs_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
