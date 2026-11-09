// Create a GitHub App Action Connection
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;
use datadog_api_client::datadogV2::model::GithubApp;
use datadog_api_client::datadogV2::model::GithubAppGithubAppId;
use datadog_api_client::datadogV2::model::GithubAppGithubInstallationId;
use datadog_api_client::datadogV2::model::GithubAppHostname;
use datadog_api_client::datadogV2::model::GithubAppPrivateKey;
use datadog_api_client::datadogV2::model::GithubAppType;
use datadog_api_client::datadogV2::model::GithubCredentials;
use datadog_api_client::datadogV2::model::GithubIntegration;
use datadog_api_client::datadogV2::model::GithubIntegrationType;

#[tokio::main]
async fn main() {
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::GithubIntegration(Box::new(GithubIntegration::new(
                GithubCredentials::GithubApp(Box::new(
                    GithubApp::new(
                        GithubAppGithubAppId::ActionConnectionLiteralValue("123456".to_string()),
                        GithubAppGithubInstallationId::ActionConnectionLiteralValue(
                            "789012".to_string(),
                        ),
                        GithubAppPrivateKey::ActionConnectionLiteralValue(
                            "example-private-key".to_string(),
                        ),
                        GithubAppType::GITHUBAPP,
                    )
                    .hostname(GithubAppHostname::ActionConnectionLiteralValue(
                        "github.example.com".to_string(),
                    )),
                )),
                GithubIntegrationType::GITHUB,
            ))),
            "GitHub App Connection exampleactionconnection".to_string(),
        ),
        ActionConnectionDataType::ACTION_CONNECTION,
    ));
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api.create_action_connection(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
