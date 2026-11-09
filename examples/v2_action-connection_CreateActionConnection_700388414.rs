// Create a GitHub personal access token Action Connection
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;
use datadog_api_client::datadogV2::model::GithubCredentials;
use datadog_api_client::datadogV2::model::GithubIntegration;
use datadog_api_client::datadogV2::model::GithubIntegrationType;
use datadog_api_client::datadogV2::model::GithubPersonalAccessToken;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenAccessToken;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenHostname;
use datadog_api_client::datadogV2::model::GithubPersonalAccessTokenType;

#[tokio::main]
async fn main() {
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::GithubIntegration(Box::new(GithubIntegration::new(
                GithubCredentials::GithubPersonalAccessToken(Box::new(
                    GithubPersonalAccessToken::new(
                        GithubPersonalAccessTokenAccessToken::ActionConnectionLiteralValue(
                            "example-token".to_string(),
                        ),
                        GithubPersonalAccessTokenType::GITHUBPERSONALACCESSTOKEN,
                    )
                    .hostname(
                        GithubPersonalAccessTokenHostname::ActionConnectionLiteralValue(
                            "github.example.com".to_string(),
                        ),
                    ),
                )),
                GithubIntegrationType::GITHUB,
            ))),
            "GitHub personal access token Connection exampleactionconnection".to_string(),
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
