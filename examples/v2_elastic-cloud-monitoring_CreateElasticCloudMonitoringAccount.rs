// Create an Elastic Cloud monitoring account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_elastic_cloud_monitoring::ElasticCloudMonitoringAPI;
use datadog_api_client::datadogV2::model::ElasticCloudAuthentication;
use datadog_api_client::datadogV2::model::ElasticCloudBasicAuth;
use datadog_api_client::datadogV2::model::ElasticCloudBasicAuthType;
use datadog_api_client::datadogV2::model::ElasticCloudDataflow;
use datadog_api_client::datadogV2::model::ElasticCloudDataflowId;
use datadog_api_client::datadogV2::model::ElasticCloudMonitoringAccountAttributes;
use datadog_api_client::datadogV2::model::ElasticCloudMonitoringAccountCreateData;
use datadog_api_client::datadogV2::model::ElasticCloudMonitoringAccountRequest;
use datadog_api_client::datadogV2::model::ElasticCloudSettings;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        ElasticCloudMonitoringAccountRequest::new(ElasticCloudMonitoringAccountCreateData::new(
            ElasticCloudMonitoringAccountAttributes::new(
                ElasticCloudAuthentication::ElasticCloudBasicAuth(Box::new(
                    ElasticCloudBasicAuth::new(
                        "your-password".to_string(),
                        ElasticCloudBasicAuthType::BASIC,
                        "datadog".to_string(),
                    ),
                )),
                "elastic-cloud-prod".to_string(),
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
            IntegrationAccountType::INTEGRATION_ACCOUNT,
        ));
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateElasticCloudMonitoringAccount", true);
    let api = ElasticCloudMonitoringAPI::with_config(configuration);
    let resp = api.create_elastic_cloud_monitoring_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
