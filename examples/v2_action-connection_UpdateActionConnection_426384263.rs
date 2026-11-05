// Update a GCP Action Connection with a versioned AWS Secrets Manager reference
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_action_connection::ActionConnectionAPI;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManager;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerReference;
use datadog_api_client::datadogV2::model::ActionConnectionAWSSecretsManagerType;
use datadog_api_client::datadogV2::model::ActionConnectionAttributesUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionDataType;
use datadog_api_client::datadogV2::model::ActionConnectionDataUpdate;
use datadog_api_client::datadogV2::model::ActionConnectionIntegrationUpdate;
use datadog_api_client::datadogV2::model::GCPCredentialsUpdate;
use datadog_api_client::datadogV2::model::GCPIntegrationType;
use datadog_api_client::datadogV2::model::GCPIntegrationUpdate;
use datadog_api_client::datadogV2::model::GCPServiceAccountCredentialType;
use datadog_api_client::datadogV2::model::GCPServiceAccountUpdate;
use datadog_api_client::datadogV2::model::GCPServiceAccountUpdatePrivateKey;
use datadog_api_client::datadogV2::model::GCPServiceAccountUpdateServiceAccountEmail;
use datadog_api_client::datadogV2::model::UpdateActionConnectionRequest;

#[tokio::main]
async fn main() {
    // there is a valid "gcp_action_connection" in the system
    let gcp_action_connection_data_id = std::env::var("GCP_ACTION_CONNECTION_DATA_ID").unwrap();

    // there is a valid "action_connection" in the system
    let action_connection_data_id = std::env::var("ACTION_CONNECTION_DATA_ID").unwrap();
    let body =
        UpdateActionConnectionRequest::new(
            ActionConnectionDataUpdate::new(
                ActionConnectionAttributesUpdate::new()
                    .external_secrets_manager(
                        Some(
                            ActionConnectionAWSSecretsManager::new(
                                action_connection_data_id.clone(),
                                "us-east-1".to_string(),
                                ActionConnectionAWSSecretsManagerType::AWS_SECRETS_MANAGER,
                            ),
                        ),
                    )
                    .integration(
                        ActionConnectionIntegrationUpdate::GCPIntegrationUpdate(
                            Box::new(
                                GCPIntegrationUpdate::new(
                                    GCPIntegrationType::GCP,
                                ).credentials(
                                    GCPCredentialsUpdate::GCPServiceAccountUpdate(
                                        Box::new(
                                            GCPServiceAccountUpdate::new(
                                                GCPServiceAccountCredentialType::GCPSERVICEACCOUNT,
                                            )
                                                .private_key(
                                                    GCPServiceAccountUpdatePrivateKey
                                                    ::ActionConnectionAWSSecretsManagerReference(
                                                        Box::new(
                                                            ActionConnectionAWSSecretsManagerReference::new(
                                                                "example/gcp/service-account".to_string(),
                                                                ActionConnectionAWSSecretsManagerType
                                                                ::AWS_SECRETS_MANAGER,
                                                            )
                                                                .key("private_key".to_string())
                                                                .version_stage("AWSCURRENT".to_string()),
                                                        ),
                                                    ),
                                                )
                                                .service_account_email(
                                                    GCPServiceAccountUpdateServiceAccountEmail
                                                    ::ActionConnectionAWSSecretsManagerReference(
                                                        Box::new(
                                                            ActionConnectionAWSSecretsManagerReference::new(
                                                                "example/gcp/service-account".to_string(),
                                                                ActionConnectionAWSSecretsManagerType
                                                                ::AWS_SECRETS_MANAGER,
                                                            ).key("client_email".to_string()),
                                                        ),
                                                    ),
                                                ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ActionConnectionDataType::ACTION_CONNECTION,
            ),
        );
    let configuration = datadog::Configuration::new();
    let api = ActionConnectionAPI::with_config(configuration);
    let resp = api
        .update_action_connection(gcp_action_connection_data_id.clone(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
