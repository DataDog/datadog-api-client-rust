// Update a GitHub personal access token Action Connection without replacing its
// secret
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributesUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionDataUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionIntegrationUpdate;
use datadog_api_client::datadogV2::model::GithubCredentialsUpdate;
use datadog_api_client::datadogV2::model::GithubIntegrationType;
use datadog_api_client::datadogV2::model::GithubIntegrationUpdate;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenType;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenUpdate;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenUpdateHostname;
use datadog_api_client::datadogV2::model::UpdateActionConnectionRequest;

#[tokio::main]
async fn main() {
    // there is a valid "github_pat_action_connection" in the system
    let github_pat_action_connection_data_id =
        std::env::var("GITHUB_PAT_ACTION_CONNECTION_DATA_ID").unwrap();
    let body =
        UpdateActionConnectionRequest::new(
            ActionConnectionDataUpdate::new(
                ActionConnectionAttributesUpdate::new()
                    .integration(
                        ActionConnectionIntegrationUpdate::GithubIntegrationUpdate(
                            Box::new(
                                GithubIntegrationUpdate::new(
                                    GithubIntegrationType::GITHUB,
                                ).credentials(
                                    GithubCredentialsUpdate::GithubPersonalAccessTokenUpdate(
                                        Box::new(
                                            GithubPersonalAccessTokenUpdate::new(
                                                GithubPersonalAccessTokenType::GITHUBPERSONALACCESSTOKEN,
                                            ).hostname(
                                                GithubPersonalAccessTokenUpdateHostname::ActionConnectionLiteralValue(
                                                    "github.updated.example.com".to_string(),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    )
                    .name("GitHub personal access token Updated exampleactionconnection".to_string()),
                ActionConnectionDataType::ACTION_CONNECTION,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .update_action_connection(github_pat_action_connection_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
