// Create a Nebius Cloud integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_nebius_cloud_integration::NebiusCloudIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthRequest;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountServiceAccountAuthType;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::NebiusCloudIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::NebiusCloudLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::NebiusCloudLogsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::NebiusCloudResourcesIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowRequest;

#[tokio::main]
async fn main() {
    let body =
        NebiusCloudIntegrationAccountCreateRequest::new(
            NebiusCloudIntegrationAccountCreateData::new(
                NebiusCloudIntegrationAccountCreateAttributes::new(
                    NebiusCloudIntegrationAccountAuthenticationRequest
                    ::NebiusCloudIntegrationAccountServiceAccountAuthRequest(
                        Box::new(
                            NebiusCloudIntegrationAccountServiceAccountAuthRequest::new(
                                NebiusCloudIntegrationAccountServiceAccountAuthType::NEBIUS_SERVICE_ACCOUNT,
                                "serviceaccount-xxxxxxxx".to_string(),
                            )
                                .private_key(
                                    r#"-----BEGIN PRIVATE KEY-----
MIIE...
-----END PRIVATE KEY-----"#.to_string(),
                                )
                                .public_key_id("publickey-yyyyyyyy".to_string())
                                .static_api_key("eyJhbGciOi...static-api-key".to_string()),
                        ),
                    ),
                    "nebius-prod".to_string(),
                    NebiusCloudIntegrationAccountSettingsRequest::new(
                        "tenant-12345678".to_string(),
                    ).account_tags(vec!["env:prod".to_string(), "team:ai".to_string()]),
                ).dataflows(
                    NebiusCloudIntegrationDataflowsRequest::new()
                        .nebius_cloud_cost_collection(
                            NebiusCloudCostCollectionIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    NebiusCloudCostCollectionIntegrationDataflowSettingsRequest::new()
                                        .access_key_id("AKID0123456789ABCDEF".to_string())
                                        .access_key_secret("wJalrXUtnFEMIstaticaccesskey".to_string())
                                        .bucket_name("nebius-cost-exports".to_string())
                                        .bucket_prefix_path("cost-export/2026/".to_string())
                                        .bucket_region("eu-north1".to_string()),
                                ),
                        )
                        .nebius_cloud_logs(
                            NebiusCloudLogsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    NebiusCloudLogsIntegrationDataflowSettingsRequest
                                    ::new().enabled_services(
                                        vec![
                                            "nebius-cloud-compute".to_string(),
                                            "nebius-cloud-kubernetes".to_string(),
                                            "default".to_string()
                                        ],
                                    ),
                                ),
                        )
                        .nebius_cloud_resources(NebiusCloudResourcesIntegrationDataflowRequest::new().enabled(true))
                        .nebius_cloud_service_quota_collection(
                            NebiusCloudServiceQuotaCollectionIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration
        .set_unstable_operation_enabled("v2.create_nebius_cloud_integration_account", true);
    let api = NebiusCloudIntegrationAPI::with_config(configuration);
    let resp = api.create_nebius_cloud_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
