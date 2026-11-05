// Create a GCP Action Connection with AWS Secrets Manager references
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
use datadog_api_client::datadogV2::model::GCPCredentials;
use datadog_api_client::datadogV2::model::GCPIntegration;
use datadog_api_client::datadogV2::model::GCPIntegrationType;
use datadog_api_client::datadogV2::model::GCPServiceAccount;
use datadog_api_client::datadogV2::model::GCPServiceAccountCredentialType;
use datadog_api_client::datadogV2::model::GCPServiceAccountPrivateKey;
use datadog_api_client::datadogV2::model::GCPServiceAccountServiceAccountEmail;

#[tokio::main]
async fn main() {
    // there is a valid "action_connection" in the system
    let action_connection_data_id = std::env::var("ACTION_CONNECTION_DATA_ID").unwrap();
    let body =
        CreateActionConnectionRequest::new(
            ActionConnectionData::new(
                ActionConnectionAttributes::new(
                    ActionConnectionIntegration::GCPIntegration(
                        Box::new(
                            GCPIntegration::new(
                                GCPCredentials::GCPServiceAccount(
                                    Box::new(
                                        GCPServiceAccount::new(
                                            GCPServiceAccountPrivateKey::ActionConnectionAWSSecretsManagerReference(
                                                Box::new(
                                                    ActionConnectionAWSSecretsManagerReference::new(
                                                        "example/gcp/service-account".to_string(),
                                                        ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                                                    ).key("private_key".to_string()),
                                                ),
                                            ),
                                            GCPServiceAccountServiceAccountEmail
                                            ::ActionConnectionAWSSecretsManagerReference(
                                                Box::new(
                                                    ActionConnectionAWSSecretsManagerReference::new(
                                                        "example/gcp/service-account".to_string(),
                                                        ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                                                    ).key("client_email".to_string()),
                                                ),
                                            ),
                                            GCPServiceAccountCredentialType::GCPSERVICEACCOUNT,
                                        ),
                                    ),
                                ),
                                GCPIntegrationType::GCP,
                            ),
                        ),
                    ),
                    "GCP Secrets Manager Connection exampleactionconnection".to_string(),
                ).external_secrets_manager(
                    Some(
                        ActionConnectionAWSSecretsManager::new(
                            action_connection_data_id.clone(),
                            "us-east-1".to_string(),
                            ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                        ),
                    ),
                ),
                ActionConnectionDataType::ACTION_CONNECTION,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api.create_action_connection(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
