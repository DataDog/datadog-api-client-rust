// Update an Elastic Cloud integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::ElasticCloudDetailedIndexStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIndexStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPendingTaskStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPrimaryShardGracefulTimeoutIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudPrimaryShardStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudShardAllocationStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::ElasticCloudSlmStatsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthType;
use datadog_api_client::datadogV2::model::IntegrationAccountBasicAuthUpdate;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ElasticCloudIntegrationAccountUpdateRequest::new(
            ElasticCloudIntegrationAccountUpdateData::new(
                ElasticCloudIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        ElasticCloudIntegrationAccountAuthenticationUpdate::IntegrationAccountBasicAuthUpdate(
                            Box::new(
                                IntegrationAccountBasicAuthUpdate::new(IntegrationAccountBasicAuthType::BASIC)
                                    .password("your-password".to_string())
                                    .username("datadog".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
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
                            .elastic_cloud_slm_stats(
                                ElasticCloudSlmStatsIntegrationDataflowRequest::new().enabled(true),
                            ),
                    )
                    .name("elastic-cloud-prod".to_string())
                    .settings(
                        ElasticCloudIntegrationAccountSettingsUpdate::new()
                            .tags("env:prod,team:saasint".to_string())
                            .url("https://example.es.us-central1.gcp.cloud.es.io:9243".to_string()),
                    ),
                "953a0060-81ec-4221-aed4-d4733b59cd96".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateElasticCloudIntegrationAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_elastic_cloud_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
