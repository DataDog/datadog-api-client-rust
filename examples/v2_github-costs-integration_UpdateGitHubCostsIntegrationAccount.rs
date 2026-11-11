// Update a GitHub Costs integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_git_hub_costs_integration::GitHubCostsIntegrationAPI;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthType;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountBearerTokenAuthUpdate;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::GitHubCostsIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        GitHubCostsIntegrationAccountUpdateRequest::new(
            GitHubCostsIntegrationAccountUpdateData::new(
                GitHubCostsIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        GitHubCostsIntegrationAccountAuthenticationUpdate
                        ::GitHubCostsIntegrationAccountBearerTokenAuthUpdate(
                            Box::new(
                                GitHubCostsIntegrationAccountBearerTokenAuthUpdate::new(
                                    GitHubCostsIntegrationAccountBearerTokenAuthType::BEARER_TOKEN,
                                ).token("your-token".to_string()),
                            ),
                        ),
                    )
                    .name("github-costs-acme".to_string())
                    .settings(
                        GitHubCostsIntegrationAccountSettingsUpdate::new()
                            .enterprise_name("acme".to_string())
                            .tags("env:prod,team:finops".to_string()),
                    ),
                "4b7c2e1a-6f3d-4e8b-9a52-1d0c8f7e3b64".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.update_git_hub_costs_integration_account", true);
    let api = GitHubCostsIntegrationAPI::with_config(configuration);
    let resp = api
        .update_git_hub_costs_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
