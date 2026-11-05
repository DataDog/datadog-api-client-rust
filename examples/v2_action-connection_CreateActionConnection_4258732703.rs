// Create an Anthropic Action Connection with an AWS Secrets Manager API token
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManager;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerReference;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerType;
use datadog_api_client::datadogV2::model::ActionConnectionAttributes;
use datadog_api_client::datadogV2::model::ActionConnectionData;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionIntegration;
use datadog_api_client::datadogV2::model::AnthropicAPIKey;
use datadog_api_client::datadogV2::model::AnthropicAPIKeyApiToken;
use datadog_api_client::datadogV2::model::AnthropicAPIKeyType;
use datadog_api_client::datadogV2::model::AnthropicCredentials;
use datadog_api_client::datadogV2::model::AnthropicIntegration;
use datadog_api_client::datadogV2::model::AnthropicIntegrationType;
use datadog_api_client::datadogV2::model::CreateActionConnectionRequest;

#[tokio::main]
async fn main() {
    // there is a valid "action_connection" in the system
    let action_connection_data_id = std::env::var("ACTION_CONNECTION_DATA_ID").unwrap();
    let body = CreateActionConnectionRequest::new(ActionConnectionData::new(
        ActionConnectionAttributes::new(
            ActionConnectionIntegration::AnthropicIntegration(Box::new(AnthropicIntegration::new(
                AnthropicCredentials::AnthropicAPIKey(Box::new(AnthropicAPIKey::new(
                    AnthropicAPIKeyApiToken::ActionConnectionAWSSecretsManagerReference(Box::new(
                        ActionConnectionAWSSecretsManagerReference::new(
                            "example/anthropic".to_string(),
                            ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                        )
                        .key("api_token".to_string()),
                    )),
                    AnthropicAPIKeyType::ANTHROPICAPIKEY,
                ))),
                AnthropicIntegrationType::ANTHROPIC,
            ))),
            "Anthropic Secrets Manager Connection exampleactionconnection".to_string(),
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
