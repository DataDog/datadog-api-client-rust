// Create a Snowflake integration account returns "Created" response
use datadog_api_client::datadog;
use datadog_api_client::datadogV2::api_snowflake_integration::SnowflakeIntegrationAPI;
use datadog_api_client::datadogV2::model::IntegrationAccountType;
use datadog_api_client::datadogV2::model::SnowflakeAccountUsageMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeCloudCostMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeEventTableLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeEventTableLogsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountAuthenticationRequest;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountCreateAttributes;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountCreateData;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountCreateRequest;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountPrivateKeyAuthType;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationAccountSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeIntegrationDataflowsRequest;
use datadog_api_client::datadogV2::model::SnowflakeOrganizationUsageMetricsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeSecurityLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeSecurityLogsIntegrationDataflowSettingsRequest;
use datadog_api_client::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowRequest;
use datadog_api_client::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest;

#[tokio::main]
async fn main() {
    let body =
        SnowflakeIntegrationAccountCreateRequest::new(
            SnowflakeIntegrationAccountCreateData::new(
                SnowflakeIntegrationAccountCreateAttributes::new(
                    SnowflakeIntegrationAccountAuthenticationRequest::new(
                        SnowflakeIntegrationAccountPrivateKeyAuthType::SNOWFLAKE_PRIVATE_KEY,
                        r#"-----BEGIN PRIVATE KEY-----
MIIE...
-----END PRIVATE KEY-----"#.to_string(),
                        "my-rsa-key".to_string(),
                    ).private_key_passphrase("your-private-key-passphrase".to_string()),
                    "prod-snowflake".to_string(),
                    SnowflakeIntegrationAccountSettingsRequest::new(
                        "myorg-myaccount".to_string(),
                        "datadog_user".to_string(),
                    ),
                ).dataflows(
                    SnowflakeIntegrationDataflowsRequest::new()
                        .snowflake_account_usage_metrics(
                            SnowflakeAccountUsageMetricsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeAccountUsageMetricsIntegrationDataflowSettingsRequest
                                    ::new().account_usage_metrics_aggregate_last_24h(false),
                                ),
                        )
                        .snowflake_cloud_cost_metrics(
                            SnowflakeCloudCostMetricsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeCloudCostMetricsIntegrationDataflowSettingsRequest
                                    ::new().query_tags("env,team,cost_center".to_string()),
                                ),
                        )
                        .snowflake_data_observability_quality_monitoring(
                            SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowSettingsRequest::new()
                                        .do_table_crawler_cron("0 */6 * * *".to_string())
                                        .sync_snowflake_system_database(true),
                                ),
                        )
                        .snowflake_event_table_logs(
                            SnowflakeEventTableLogsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeEventTableLogsIntegrationDataflowSettingsRequest::new()
                                        .event_table_events_enabled(true)
                                        .event_table_logs_enabled(true)
                                        .event_table_logs_interval_min(15)
                                        .event_table_span_events_enabled(false)
                                        .event_table_spans_enabled(false),
                                ),
                        )
                        .snowflake_organization_usage_metrics(
                            SnowflakeOrganizationUsageMetricsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeOrganizationUsageMetricsIntegrationDataflowSettingsRequest
                                    ::new().organization_usage_metrics_aggregate_last_24h(false),
                                ),
                        )
                        .snowflake_query_history_logs(
                            SnowflakeQueryHistoryLogsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeQueryHistoryLogsIntegrationDataflowSettingsRequest::new()
                                        .join_query_history_with_access_history_enabled(true)
                                        .query_history_logs_interval_min(15),
                                ),
                        )
                        .snowflake_security_logs(
                            SnowflakeSecurityLogsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeSecurityLogsIntegrationDataflowSettingsRequest
                                    ::new().security_logs_interval_min(60),
                                ),
                        )
                        .snowflake_task_history_logs(
                            SnowflakeTaskHistoryLogsIntegrationDataflowRequest::new()
                                .enabled(true)
                                .settings(
                                    SnowflakeTaskHistoryLogsIntegrationDataflowSettingsRequest
                                    ::new().task_history_logs_interval_min(30),
                                ),
                        ),
                ),
                IntegrationAccountType::INTEGRATION_ACCOUNT,
            ),
        );
    let mut configuration = datadog::Configuration::new();
    configuration.set_unstable_operation_enabled("v2.CreateSnowflakeIntegrationAccount", true);
    let api = SnowflakeIntegrationAPI::with_config(configuration);
    let resp = api.create_snowflake_integration_account(body).await;
    if let Ok(value) = resp {
        println!("{:#?}", value);
    } else {
        println!("{:#?}", resp.unwrap_err());
    }
}
