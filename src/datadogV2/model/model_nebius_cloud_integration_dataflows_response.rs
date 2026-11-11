// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Nebius Cloud, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationDataflowsResponse {
    /// Cloud cost data read from a Nebius Object Storage bucket export and used by [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Requires a bucket holding your cost export and a Nebius access key pair that can read it; without both this dataflow collects no data.
    #[serde(rename = "nebius-cloud-cost-collection")]
    pub nebius_cloud_cost_collection:
        Option<crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowResponse>,
    /// Logs from your Nebius tenant: Kubernetes control plane and audit logs, tenant-level audit events, compute serial console logs, cloud apps and AI inference logs, PostgreSQL logs, and MLflow logs, collected from the services named in `settings.enabled_services`.
    #[serde(rename = "nebius-cloud-logs")]
    pub nebius_cloud_logs:
        Option<crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowResponse>,
    /// Metadata about resources in your Nebius tenant, such as compute instances and Kubernetes clusters.
    #[serde(rename = "nebius-cloud-resources")]
    pub nebius_cloud_resources:
        Option<crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowResponse>,
    /// Usage metrics tracked against your Nebius service quotas, such as `nebius_cloud.servicequota.percentage_used` and `nebius_cloud.servicequota.limit`.
    #[serde(rename = "nebius-cloud-service-quota-collection")]
    pub nebius_cloud_service_quota_collection: Option<
        crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowResponse,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationDataflowsResponse {
    pub fn new() -> NebiusCloudIntegrationDataflowsResponse {
        NebiusCloudIntegrationDataflowsResponse {
            nebius_cloud_cost_collection: None,
            nebius_cloud_logs: None,
            nebius_cloud_resources: None,
            nebius_cloud_service_quota_collection: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn nebius_cloud_cost_collection(
        mut self,
        value: crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowResponse,
    ) -> Self {
        self.nebius_cloud_cost_collection = Some(value);
        self
    }

    pub fn nebius_cloud_logs(
        mut self,
        value: crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowResponse,
    ) -> Self {
        self.nebius_cloud_logs = Some(value);
        self
    }

    pub fn nebius_cloud_resources(
        mut self,
        value: crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowResponse,
    ) -> Self {
        self.nebius_cloud_resources = Some(value);
        self
    }

    pub fn nebius_cloud_service_quota_collection(
        mut self,
        value: crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowResponse,
    ) -> Self {
        self.nebius_cloud_service_quota_collection = Some(value);
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

impl Default for NebiusCloudIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationDataflowsResponseVisitor {
            type Value = NebiusCloudIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nebius_cloud_cost_collection: Option<
                    crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowResponse,
                > = None;
                let mut nebius_cloud_logs: Option<
                    crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowResponse,
                > = None;
                let mut nebius_cloud_resources: Option<
                    crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowResponse,
                > = None;
                let mut nebius_cloud_service_quota_collection: Option<crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "nebius-cloud-cost-collection" => {
                            if v.is_null() {
                                continue;
                            }
                            nebius_cloud_cost_collection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nebius-cloud-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            nebius_cloud_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nebius-cloud-resources" => {
                            if v.is_null() {
                                continue;
                            }
                            nebius_cloud_resources =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "nebius-cloud-service-quota-collection" => {
                            if v.is_null() {
                                continue;
                            }
                            nebius_cloud_service_quota_collection =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NebiusCloudIntegrationDataflowsResponse {
                    nebius_cloud_cost_collection,
                    nebius_cloud_logs,
                    nebius_cloud_resources,
                    nebius_cloud_service_quota_collection,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationDataflowsResponseVisitor)
    }
}
