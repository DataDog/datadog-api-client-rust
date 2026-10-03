// Create a GitHub cloud auth intake mapping returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_cloud_authentication::CloudAuthenticationAPI;
use datadog_api_client::datadogV2::model::GitHubCloudAuthIntakeMappingCreateAttributes;
use datadog_api_client::datadogV2::model::GitHubCloudAuthIntakeMappingCreateData;
use datadog_api_client::datadogV2::model::GitHubCloudAuthIntakeMappingCreateRequest;
use datadog_api_client::datadogV2::model::GitHubCloudAuthIntakeMappingType;
use datadog_api_client::datadogV2::model::GitHubOIDCClaimPatterns;

#[tokio::main]
async fn main() {
    let body = GitHubCloudAuthIntakeMappingCreateRequest::new(
        GitHubCloudAuthIntakeMappingCreateData::new(
            GitHubCloudAuthIntakeMappingCreateAttributes::new(
                GitHubOIDCClaimPatterns::new(
                    "repo:test_owner/test_repo:(ref:refs/heads/main|pull_request)".to_string(),
                )
                .actor("octocat".to_string())
                .actor_id("1234567".to_string())
                .enterprise("test_enterprise".to_string())
                .enterprise_id("42".to_string())
                .environment("production".to_string())
                .event_name("push".to_string())
                .job_workflow_ref(
                    "test_owner/test_repo/.github/workflows/jobs.yml@refs/heads/main".to_string(),
                )
                .ref_("refs/heads/main".to_string())
                .ref_type("branch".to_string())
                .repository("test_owner/test_repo".to_string())
                .repository_id("123456789".to_string())
                .repository_owner("test_owner".to_string())
                .repository_owner_id("987654321".to_string())
                .repository_visibility("public".to_string())
                .runner_environment("github-hosted".to_string())
                .workflow("CI".to_string())
                .workflow_ref(
                    "test_owner/test_repo/.github/workflows/ci.yml@refs/heads/main".to_string(),
                ),
            ),
            GitHubCloudAuthIntakeMappingType::GITHUB_OIDC_AUTH_INTAKE_MAPPING,
        ),
    );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateGitHubCloudAuthIntakeMapping", true);
    let api = CloudAuthenticationAPI::with_config(configuration);
    let resp = api.create_git_hub_cloud_auth_intake_mapping(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
