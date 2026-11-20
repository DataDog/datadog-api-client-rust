// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Data Datadog collects from CoreWeave, keyed by dataflow id. Each dataflow turns on a distinct kind of collection: set `enabled` to start or stop it. Defaults listed on each dataflow apply when the account is created; on update, omitted dataflow ids keep their current values. Collection runs with the API access token in `authentication`, whose user must have the Observability Viewer role; a dataflow enabled without that role is stored but collects no data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationDataflowsRequest {
    /// Service logs for CoreWeave-managed components in each CoreWeave region: the CKS control plane (such as `kube-apiserver` and `etcd`), CoreWeave add-on services in the `cw-*` and `kube-system` namespaces, SUNK, and node system logs such as `kubelet` and `containerd`. Logs from your own workloads and Kubernetes audit logs are not collected. Logs are sent with `source:coreweave`, with CoreWeave log labels such as `namespace`, `app`, and `systemd_unit` added as tags.
    #[serde(rename = "coreweave-cloud-logs")]
    pub coreweave_cloud_logs: Option<crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowRequest>,
    /// Metrics from your CoreWeave organization: CoreWeave service metrics collected through its PromQL-compatible Prometheus endpoint, plus the `coreweave.*` namespaces from CoreWeave products such as Kubernetes, Database, Distributed File Storage, Inference, and Object Storage. Compute metrics (`coreweave.bare_metal`, `coreweave.gpu`, `coreweave.nccl`, and `coreweave.node`) are in Preview. Container metrics are not collected; install the Datadog Agent on your clusters for those.
    #[serde(rename = "coreweave-cloud-metrics")]
    pub coreweave_cloud_metrics: Option<crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationDataflowsRequest {
    pub fn new() -> CoreWeaveCloudIntegrationDataflowsRequest {
        CoreWeaveCloudIntegrationDataflowsRequest {
            coreweave_cloud_logs: None,
            coreweave_cloud_metrics: None,
            _unparsed: false,
        }
    }
    
    pub fn coreweave_cloud_logs(mut self, value: crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowRequest) -> Self {
        self.coreweave_cloud_logs = Some(value);
        self
    }
    
    pub fn coreweave_cloud_metrics(mut self, value: crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowRequest) -> Self {
        self.coreweave_cloud_metrics = Some(value);
        self
    }
    
}


impl Default for CoreWeaveCloudIntegrationDataflowsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationDataflowsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationDataflowsRequestVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationDataflowsRequestVisitor {
            type Value = CoreWeaveCloudIntegrationDataflowsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut coreweave_cloud_logs: Option<crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowRequest> = None;
                let mut coreweave_cloud_metrics: Option<crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "coreweave-cloud-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            coreweave_cloud_logs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "coreweave-cloud-metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            coreweave_cloud_metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = CoreWeaveCloudIntegrationDataflowsRequest {
                    coreweave_cloud_logs,
                    coreweave_cloud_metrics,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationDataflowsRequestVisitor)
    }
}
