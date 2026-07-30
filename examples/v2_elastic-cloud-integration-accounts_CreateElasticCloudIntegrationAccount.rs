// Create an Elastic Cloud integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_integration_accounts::ElasticCloudIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::ElasticCloudAuthentication;
use datadog_api_client::datadogV2::model::ElasticCloudBasicAuth;
use datadog_api_client::datadogV2::model::ElasticCloudBasicAuthType;
use datadog_api_client::datadogV2::model::ElasticCloudDataflow;
use datadog_api_client::datadogV2::model::ElasticCloudDataflowId;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::ElasticCloudIntegrationAccountRequest;
use datadog_api_client::datadogV2::model::ElasticCloudInterface;
use datadog_api_client::datadogV2::model::ElasticCloudInterfaceId;
use datadog_api_client::datadogV2::model::ElasticCloudMonitoringInterface;
use datadog_api_client::datadogV2::model::ElasticCloudMonitoringInterfaceType;
use datadog_api_client::datadogV2::model::ElasticCloudSettings;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ElasticCloudIntegrationAccountRequest::new(ElasticCloudIntegrationAccountCreateData::new(
            ElasticCloudIntegrationAccountAttributes::new(
                ElasticCloudInterface::ElasticCloudMonitoringInterface(Box::new(
                    ElasticCloudMonitoringInterface::new(
                        ElasticCloudAuthentication::ElasticCloudBasicAuth(Box::new(
                            ElasticCloudBasicAuth::new(
                                "your-password".to_string(),
                                ElasticCloudBasicAuthType::BASIC,
                                "datadog".to_string(),
                            ),
                        )),
                        ElasticCloudMonitoringInterfaceType::ELASTIC_CLOUD,
                    )
                    .dataflows(vec![ElasticCloudDataflow::new(
                        ElasticCloudDataflowId::METRICS,
                    )
                    .enabled(true)])
                    .settings(
                        ElasticCloudSettings::new(
                            "https://example.es.us-central1.gcp.cloud.es.io:9243".to_string(),
                        )
                        .cat_allocation_stats_enabled(false)
                        .detailed_index_stats_enabled(false)
                        .index_stats_enabled(false)
                        .pending_task_stats_enabled(false)
                        .pshard_graceful_to_enabled(false)
                        .pshard_stats_enabled(false)
                        .slm_stats_enabled(false)
                        .tags(vec!["env:prod".to_string()]),
                    ),
                )),
                "elastic-cloud-prod".to_string(),
            ),
            IntegrationAccountType::INTEGRATION_ACCOUNT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateElasticCloudIntegrationAccount", true);
    let api = ElasticCloudIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .create_elastic_cloud_integration_account(ElasticCloudInterfaceId::ELASTIC_CLOUD, body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
