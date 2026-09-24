// Create a Databricks integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_databricks_integration::DatabricksIntegrationAPI;
use datadog_api_client::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityJobsMonitoringIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountOAuthAuthRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        DatabricksIntegrationAccountCreateRequest::new(
            DatabricksIntegrationAccountCreateData::new(
                DatabricksIntegrationAccountCreateAttributes::new(
                    DatabricksIntegrationAccountAuthenticationRequest::DatabricksIntegrationAccountOAuthAuthRequest(
                        Box::new(
                            DatabricksIntegrationAccountOAuthAuthRequest::new(
                                DatabricksIntegrationAccountOAuthAuthType::DATABRICKS_OAUTH,
                                "5c10654a-b3a3-4840-b37f-f477590c70a0".to_string(),
                                "your-client-secret".to_string(),
                            ).azure_tenant_id("4d3bac44-0230-4732-9e70-cc00736f0a97".to_string()),
                        ),
                    ),
                    "My Databricks Workspace".to_string(),
                    DatabricksIntegrationAccountSettingsRequest::new(
                        "https://dbc-1234abcd.cloud.databricks.com".to_string(),
                    ).system_tables_sql_warehouse_id("aba7c023d4172910".to_string()),
                ).dataflows(
                    DatabricksIntegrationDataflowsRequest::new()
                        .databricks_cloud_cost_metrics(
                            DatabricksCloudCostMetricsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest
                                    ::new().ccm_collect_all_workspaces(true),
                                ),
                        )
                        .databricks_data_observability_jobs_monitoring(
                            DatabricksDataObservabilityJobsMonitoringIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    DatabricksDataObservabilityJobsMonitoringIntegrationDataflowSettingsRequest::new()
                                        .dd_api_key_id("fe383f4e-09fc-46bf-8e10-4efdd453a646".to_string())
                                        .dd_api_key_secret("your-datadog-api-key".to_string())
                                        .djm_global_init_script_enabled(true)
                                        .script_gpum_enabled(true)
                                        .script_logs_enabled(true)
                                        .serverless_jobs_enabled(true),
                                ),
                        )
                        .databricks_data_observability_quality_monitoring(
                            DatabricksDataObservabilityQualityMonitoringIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    DatabricksDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest
                                    ::new()
                                        .do_crawlers_cron("0 * * * *".to_string())
                                        .sync_system_catalog(true),
                                ),
                        )
                        .databricks_model_serving_metrics(
                            DatabricksModelServingMetricsIntegrationDataflowRequest::new().enabled(true),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateDatabricksIntegrationAccount", true);
    let api = DatabricksIntegrationAPI::with_config(configuration);
    let resp = api.create_databricks_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
