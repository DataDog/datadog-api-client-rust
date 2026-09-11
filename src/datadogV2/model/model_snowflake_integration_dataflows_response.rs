// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Snowflake, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SnowflakeIntegrationDataflowsResponse {
    /// Account-level usage metrics read from the Snowflake `ACCOUNT_USAGE` schema, covering storage usage, credit consumption, and query activity.
    #[serde(rename = "snowflake-account-usage-metrics")]
    pub snowflake_account_usage_metrics: Option<crate::datadogV2::model::SnowflakeAccountUsageMetricsIntegrationDataflowResponse>,
    /// Cost data aggregated from the Snowflake `ORGANIZATION_USAGE` schema. Requires [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>) to be set up for your organization, and the ORGANIZATION_BILLING_VIEWER database role on the Snowflake role.
    #[serde(rename = "snowflake-cloud-cost-metrics")]
    pub snowflake_cloud_cost_metrics: Option<crate::datadogV2::model::SnowflakeCloudCostMetricsIntegrationDataflowResponse>,
    /// Data Observability, which collects lineage and data quality information from your Snowflake databases so you can explore how data flows and detect and resolve quality issues.
    #[serde(rename = "snowflake-data-observability-quality-monitoring")]
    pub snowflake_data_observability_quality_monitoring: Option<crate::datadogV2::model::SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowResponse>,
    /// Records from your Snowflake event tables, used to monitor application behavior and identify issues. `enabled` turns the dataflow on and off as a whole, and the per-record-type toggles in `settings` select which kinds of record it collects while it is on. The Snowflake role needs usage granted on the database, the schema, and the event table itself.
    #[serde(rename = "snowflake-event-table-logs")]
    pub snowflake_event_table_logs: Option<crate::datadogV2::model::SnowflakeEventTableLogsIntegrationDataflowResponse>,
    /// Organization-level usage metrics read from the Snowflake `ORGANIZATION_USAGE` schema, covering the credit consumption of every account in the organization and the history of data transferred out of Snowflake. Reading that schema requires the ORGADMIN role.
    #[serde(rename = "snowflake-organization-usage-metrics")]
    pub snowflake_organization_usage_metrics: Option<crate::datadogV2::model::SnowflakeOrganizationUsageMetricsIntegrationDataflowResponse>,
    /// Per-query logs that let you identify long-running, poorly performing, and expensive queries.
    #[serde(rename = "snowflake-query-history-logs")]
    pub snowflake_query_history_logs: Option<crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowResponse>,
    /// Security logs from the Snowflake `ACCOUNT_USAGE` schema, for analyzing the security of your Snowflake account and running threat detection with [Cloud SIEM](<https://docs.datadoghq.com/security/cloud_siem/>).
    #[serde(rename = "snowflake-security-logs")]
    pub snowflake_security_logs: Option<crate::datadogV2::model::SnowflakeSecurityLogsIntegrationDataflowResponse>,
    /// Execution logs for your scheduled Snowflake tasks, covering start and end time, status, and any error message.
    #[serde(rename = "snowflake-task-history-logs")]
    pub snowflake_task_history_logs: Option<crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SnowflakeIntegrationDataflowsResponse {
    pub fn new() -> SnowflakeIntegrationDataflowsResponse {
        SnowflakeIntegrationDataflowsResponse {
            snowflake_account_usage_metrics: None,
            snowflake_cloud_cost_metrics: None,
            snowflake_data_observability_quality_monitoring: None,
            snowflake_event_table_logs: None,
            snowflake_organization_usage_metrics: None,
            snowflake_query_history_logs: None,
            snowflake_security_logs: None,
            snowflake_task_history_logs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn snowflake_account_usage_metrics(
        mut self,
        value: crate::datadogV2::model::SnowflakeAccountUsageMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_account_usage_metrics = Some(value);
        self
    }

    pub fn snowflake_cloud_cost_metrics(
        mut self,
        value: crate::datadogV2::model::SnowflakeCloudCostMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_cloud_cost_metrics = Some(value);
        self
    }

    pub fn snowflake_data_observability_quality_monitoring(
        mut self,
        value: crate::datadogV2::model::SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_data_observability_quality_monitoring = Some(value);
        self
    }

    pub fn snowflake_event_table_logs(
        mut self,
        value: crate::datadogV2::model::SnowflakeEventTableLogsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_event_table_logs = Some(value);
        self
    }

    pub fn snowflake_organization_usage_metrics(
        mut self,
        value: crate::datadogV2::model::SnowflakeOrganizationUsageMetricsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_organization_usage_metrics = Some(value);
        self
    }

    pub fn snowflake_query_history_logs(
        mut self,
        value: crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_query_history_logs = Some(value);
        self
    }

    pub fn snowflake_security_logs(
        mut self,
        value: crate::datadogV2::model::SnowflakeSecurityLogsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_security_logs = Some(value);
        self
    }

    pub fn snowflake_task_history_logs(
        mut self,
        value: crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowResponse,
    ) -> Self {
        self.snowflake_task_history_logs = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for SnowflakeIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SnowflakeIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for SnowflakeIntegrationDataflowsResponseVisitor {
            type Value = SnowflakeIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut snowflake_account_usage_metrics: Option<crate::datadogV2::model::SnowflakeAccountUsageMetricsIntegrationDataflowResponse> = None;
                let mut snowflake_cloud_cost_metrics: Option<
                    crate::datadogV2::model::SnowflakeCloudCostMetricsIntegrationDataflowResponse,
                > = None;
                let mut snowflake_data_observability_quality_monitoring: Option<crate::datadogV2::model::SnowflakeDataObservabilityQualityMonitoringIntegrationDataflowResponse> = None;
                let mut snowflake_event_table_logs: Option<
                    crate::datadogV2::model::SnowflakeEventTableLogsIntegrationDataflowResponse,
                > = None;
                let mut snowflake_organization_usage_metrics: Option<crate::datadogV2::model::SnowflakeOrganizationUsageMetricsIntegrationDataflowResponse> = None;
                let mut snowflake_query_history_logs: Option<
                    crate::datadogV2::model::SnowflakeQueryHistoryLogsIntegrationDataflowResponse,
                > = None;
                let mut snowflake_security_logs: Option<
                    crate::datadogV2::model::SnowflakeSecurityLogsIntegrationDataflowResponse,
                > = None;
                let mut snowflake_task_history_logs: Option<
                    crate::datadogV2::model::SnowflakeTaskHistoryLogsIntegrationDataflowResponse,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "snowflake-account-usage-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_account_usage_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-cloud-cost-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_cloud_cost_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-data-observability-quality-monitoring" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_data_observability_quality_monitoring =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-event-table-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_event_table_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-organization-usage-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_organization_usage_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-query-history-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_query_history_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-security-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_security_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snowflake-task-history-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            snowflake_task_history_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SnowflakeIntegrationDataflowsResponse {
                    snowflake_account_usage_metrics,
                    snowflake_cloud_cost_metrics,
                    snowflake_data_observability_quality_monitoring,
                    snowflake_event_table_logs,
                    snowflake_organization_usage_metrics,
                    snowflake_query_history_logs,
                    snowflake_security_logs,
                    snowflake_task_history_logs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SnowflakeIntegrationDataflowsResponseVisitor)
    }
}
