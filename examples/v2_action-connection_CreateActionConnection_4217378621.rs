// Create an HTTP Action Connection with an AWS Secrets Manager token
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManager;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerReference;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerType;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;
use datadog_api_client::datadogV2::model::HTTPBody;
use datadog_api_client::datadogV2::model::HTTPCredentials;
use datadog_api_client::datadogV2::model::HTTPHeader;
use datadog_api_client::datadogV2::model::HTTPIntegration;
use datadog_api_client::datadogV2::model::HTTPIntegrationType;
use datadog_api_client::datadogV2::model::HTTPToken;
use datadog_api_client::datadogV2::model::HTTPTokenAuth;
use datadog_api_client::datadogV2::model::HTTPTokenAuthType;
use datadog_api_client::datadogV2::model::HTTPTokenValue;
use datadog_api_client::datadogV2::model::TokenType;
use datadog_api_client::datadogV2::model::UrlParam;

#[tokio::main]
async fn main() {
    // there is a valid "action_connection" in the system
    let action_connection_data_id = std::env::var("ACTION_CONNECTION_DATA_ID").unwrap();
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::HTTPIntegration(Box::new(HTTPIntegration::new(
                "https://example.com".to_string(),
                HTTPCredentials::HTTPTokenAuth(Box::new(
                    HTTPTokenAuth::new(HTTPTokenAuthType::HTTPTOKENAUTH)
                        .body(
                            HTTPBody::new()
                                .content(r#"{"example":"literal"}"#.to_string())
                                .content_type("application/json".to_string()),
                        )
                        .headers(vec![HTTPHeader::new(
                            "ApiKey".to_string(),
                            "literal-value".to_string(),
                        )])
                        .tokens(vec![HTTPToken::new(
                            "token".to_string(),
                            TokenType::SECRET,
                            HTTPTokenValue::ActionConnectionAWSSecretsManagerReference(Box::new(
                                ActionConnectionAWSSecretsManagerReference::new(
                                    "example/http".to_string(),
                                    ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                                )
                                .key("token".to_string()),
                            )),
                        )])
                        .url_parameters(vec![UrlParam::new(
                            "apiKey".to_string(),
                            "literal-value".to_string(),
                        )]),
                )),
                HTTPIntegrationType::HTTP,
            ))),
            "HTTP Secrets Manager Connection exampleactionconnection".to_string(),
        )
        .external_secrets_manager(Some(ActionConnectionAWSSecretsManager::new(
            action_connection_data_id.clone(),
            "us-east-1".to_string(),
            ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
        ))),
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
