// Create an Elastic Cloud integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::ElasticCloudDetailedIndexStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIndexStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPendingTaskStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPrimaryShardStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudShardAllocationStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudSlmStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ElasticCloudIntegrationAccountCreateRequest::new(
            ElasticCloudIntegrationAccountCreateData::new(
                ElasticCloudIntegrationAccountCreateAttributes::new(
                    ElasticCloudIntegrationAccountAuthenticationRequest::IntegrationAccountBasicAuthRequest(
                        Box::new(
                            IntegrationAccountBasicAuthRequest::new(
                                IntegrationAccountBasicAuthType::BASIC,
                                "your-password".to_string(),
                                "datadog".to_string(),
                            ),
                        ),
                    ),
                    "elastic-cloud-prod".to_string(),
                    ElasticCloudIntegrationAccountSettingsRequest::new(
                        "https://example.es.us-central1.gcp.cloud.es.io:9243".to_string(),
                    ).tags("env:prod,team:saasint".to_string()),
                ).dataflows(
                    ElasticCloudIntegrationDataflowsRequest::new()
                        .elastic_cloud_detailed_index_stats(
                            ElasticCloudDetailedIndexStatsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_index_stats(
                            ElasticCloudIndexStatsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_pending_task_stats(
                            ElasticCloudPendingTaskStatsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_primary_shard_graceful_timeout(
                            ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_primary_shard_stats(
                            ElasticCloudPrimaryShardStatsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_shard_allocation_stats(
                            ElasticCloudShardAllocationStatsIntegrationDataflowRequest::new().enabled(true),
                        )
                        .elastic_cloud_slm_stats(ElasticCloudSlmStatsIntegrationDataflowRequest::new().enabled(true)),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateElasticCloudIntegrationAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api.create_elastic_cloud_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
