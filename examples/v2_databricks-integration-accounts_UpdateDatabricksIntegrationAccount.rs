// Update a Databricks integration account returns "OK" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_databricks_integration_accounts::DatabricksIntegrationAccountsAPI;
use datadog_api_client::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksDataJobMonitoringIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksDataObservabilityIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountAuthenticationUpdate;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountOAuthAuthType;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountOAuthAuthUpdate;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountSettingsUpdate;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountUpdateAttributes;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountUpdateData;
use datadog_api_client::datadogV2::model::DatabricksIntegrationAccountUpdateRequest;
use datadog_api_client::datadogV2::model::DatabricksIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::DatabricksModelServingMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::DatabricksServerlessJobsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::IntegrationAccountType;

#[tokio::main]
async fn main() {
    let body =
        DatabricksIntegrationAccountUpdateRequest::new(
            DatabricksIntegrationAccountUpdateData::new(
                DatabricksIntegrationAccountUpdateAttributes::new()
                    .authentication(
                        DatabricksIntegrationAccountAuthenticationUpdate::DatabricksIntegrationAccountOAuthAuthUpdate(
                            Box::new(
                                DatabricksIntegrationAccountOAuthAuthUpdate::new(
                                    DatabricksIntegrationAccountOAuthAuthType::DATABRICKS_OAUTH,
                                    "5c10654a-b3a3-4840-b37f-f477590c70a0".to_string(),
                                )
                                    .azure_tenant_id("4d3bac44-0230-4732-9e70-cc00736f0a97".to_string())
                                    .client_secret("your-client-secret".to_string()),
                            ),
                        ),
                    )
                    .dataflows(
                        DatabricksIntegrationDataflowsRequest::new()
                            .databricks_cloud_cost_metrics(
                                DatabricksCloudCostMetricsIntegrationDataflowRequest::new()
                                    .enabled(true)
                                    .settings(
                                        DatabricksCloudCostMetricsIntegrationDataflowSettingsRequest
                                        ::new().ccm_collect_all_workspaces(true),
                                    ),
                            )
                            .databricks_data_job_monitoring(
                                DatabricksDataJobMonitoringIntegrationDataflowRequest::new()
                                    .enabled(true)
                                    .settings(
                                        DatabricksDataJobMonitoringIntegrationDataflowSettingsRequest::new()
                                            .dd_api_key_id("fe383f4e-09fc-46bf-8e10-4efdd453a646".to_string())
                                            .dd_api_key_secret("your-datadog-api-key".to_string())
                                            .djm_global_init_script_enabled(true)
                                            .script_gpum_enabled(true)
                                            .script_logs_enabled(true),
                                    ),
                            )
                            .databricks_data_observability(
                                DatabricksDataObservabilityIntegrationDataflowRequest::new()
                                    .enabled(true)
                                    .settings(
                                        DatabricksDataObservabilityIntegrationDataflowSettingsRequest::new()
                                            .do_crawlers_cron("0 * * * *".to_string())
                                            .sync_system_catalog(true),
                                    ),
                            )
                            .databricks_model_serving_metrics(
                                DatabricksModelServingMetricsIntegrationDataflowRequest::new().enabled(true),
                            )
                            .databricks_serverless_jobs(
                                DatabricksServerlessJobsIntegrationDataflowRequest::new().enabled(true),
                            ),
                    )
                    .name("My Databricks Workspace".to_string())
                    .settings(
                        DatabricksIntegrationAccountSettingsUpdate::new()
                            .system_tables_sql_warehouse_id("aba7c023d4172910".to_string())
                            .workspace_url("https://dbc-1234abcd.cloud.databricks.com".to_string()),
                    ),
                "a9a69c2e-4f8d-4e42-9c1a-2a7a2d3b7c6f".to_string(),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.UpdateDatabricksIntegrationAccount", true);
    let api = DatabricksIntegrationAccountsAPI::with_config(configuration);
    let resp = api
        .update_databricks_integration_account("account_id".to_string(), body)
        .await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
