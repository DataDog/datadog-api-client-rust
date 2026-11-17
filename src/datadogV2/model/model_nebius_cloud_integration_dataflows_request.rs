// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Nebius Cloud, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to stop or start it, and use `settings` to configure what it collects. Defaults listed on each dataflow apply when the account is created; on update, omitted fields keep their current values. Every dataflow except `nebius-cloud-cost-collection` runs as the service account in `authentication`, using its authorized key pair and the static API key; enabling one of them without those keys is rejected with a `422` response, and one enabled with keys that later expire is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudIntegrationDataflowsRequest {
    /// Cloud cost data read from a Nebius Object Storage bucket export and used by [Cloud Cost Management](<https://docs.datadoghq.com/cloud_cost_management/>). Enabling it requires a bucket holding your cost export and a Nebius access key pair that can read it; a request that enables this dataflow without them is rejected with a `422` response.
    #[serde(rename = "nebius-cloud-cost-collection")]
    pub nebius_cloud_cost_collection:
        Option<crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowRequest>,
    /// Logs from your Nebius tenant: Kubernetes control plane and audit logs, tenant-level audit events, compute serial console logs, cloud apps and AI inference logs, PostgreSQL logs, and MLflow logs, collected from the services named in `settings.enabled_services`. `enabled_services` is the sole source of truth for what is collected; `enabled` set to `false` stops log collection, while `true` on its own changes nothing. Defaults to no services.
    #[serde(rename = "nebius-cloud-logs")]
    pub nebius_cloud_logs:
        Option<crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowRequest>,
    /// Metadata about resources in your Nebius tenant, such as compute instances and Kubernetes clusters.
    #[serde(rename = "nebius-cloud-resources")]
    pub nebius_cloud_resources:
        Option<crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowRequest>,
    /// Usage metrics tracked against your Nebius service quotas, such as `nebius_cloud.servicequota.percentage_used` and `nebius_cloud.servicequota.limit`.
    #[serde(rename = "nebius-cloud-service-quota-collection")]
    pub nebius_cloud_service_quota_collection: Option<
        crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowRequest,
    >,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudIntegrationDataflowsRequest {
    pub fn new() -> NebiusCloudIntegrationDataflowsRequest {
        NebiusCloudIntegrationDataflowsRequest {
            nebius_cloud_cost_collection: None,
            nebius_cloud_logs: None,
            nebius_cloud_resources: None,
            nebius_cloud_service_quota_collection: None,
            _unparsed: false,
        }
    }

    pub fn nebius_cloud_cost_collection(
        mut self,
        value: crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowRequest,
    ) -> Self {
        self.nebius_cloud_cost_collection = Some(value);
        self
    }

    pub fn nebius_cloud_logs(
        mut self,
        value: crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowRequest,
    ) -> Self {
        self.nebius_cloud_logs = Some(value);
        self
    }

    pub fn nebius_cloud_resources(
        mut self,
        value: crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowRequest,
    ) -> Self {
        self.nebius_cloud_resources = Some(value);
        self
    }

    pub fn nebius_cloud_service_quota_collection(
        mut self,
        value: crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowRequest,
    ) -> Self {
        self.nebius_cloud_service_quota_collection = Some(value);
        self
    }
}

impl Default for NebiusCloudIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudIntegrationDataflowsRequestVisitor {
            type Value = NebiusCloudIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut nebius_cloud_cost_collection: Option<
                    crate::datadogV2::model::NebiusCloudCostCollectionIntegrationDataflowRequest,
                > = None;
                let mut nebius_cloud_logs: Option<
                    crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowRequest,
                > = None;
                let mut nebius_cloud_resources: Option<
                    crate::datadogV2::model::NebiusCloudResourcesIntegrationDataflowRequest,
                > = None;
                let mut nebius_cloud_service_quota_collection: Option<crate::datadogV2::model::NebiusCloudServiceQuotaCollectionIntegrationDataflowRequest> = None;
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
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudIntegrationDataflowsRequest {
                    nebius_cloud_cost_collection,
                    nebius_cloud_logs,
                    nebius_cloud_resources,
                    nebius_cloud_service_quota_collection,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudIntegrationDataflowsRequestVisitor)
    }
}
