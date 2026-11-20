// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Data Datadog collects from CoreWeave, keyed by dataflow id.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudIntegrationDataflowsResponse {
    /// Service logs for CoreWeave-managed components in each CoreWeave region: the CKS control plane (such as `kube-apiserver` and `etcd`), CoreWeave add-on services in the `cw-*` and `kube-system` namespaces, SUNK, and node system logs such as `kubelet` and `containerd`. Logs from your own workloads and Kubernetes audit logs are not collected. Logs are sent with `source:coreweave`, with CoreWeave log labels such as `namespace`, `app`, and `systemd_unit` added as tags.
    #[serde(rename = "coreweave-cloud-logs")]
    pub coreweave_cloud_logs: Option<crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowResponse>,
    /// Metrics from your CoreWeave organization: CoreWeave service metrics collected through its PromQL-compatible Prometheus endpoint, plus the `coreweave.*` namespaces from CoreWeave products such as Kubernetes, Database, Distributed File Storage, Inference, and Object Storage. Compute metrics (`coreweave.bare_metal`, `coreweave.gpu`, `coreweave.nccl`, and `coreweave.node`) are in Preview. Container metrics are not collected; install the Datadog Agent on your clusters for those.
    #[serde(rename = "coreweave-cloud-metrics")]
    pub coreweave_cloud_metrics: Option<crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudIntegrationDataflowsResponse {
    pub fn new() -> CoreWeaveCloudIntegrationDataflowsResponse {
        CoreWeaveCloudIntegrationDataflowsResponse {
            coreweave_cloud_logs: None,
            coreweave_cloud_metrics: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn coreweave_cloud_logs(mut self, value: crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowResponse) -> Self {
        self.coreweave_cloud_logs = Some(value);
        self
    }
    
    pub fn coreweave_cloud_metrics(mut self, value: crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowResponse) -> Self {
        self.coreweave_cloud_metrics = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for CoreWeaveCloudIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudIntegrationDataflowsResponseVisitor {
            type Value = CoreWeaveCloudIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut coreweave_cloud_logs: Option<crate::datadogV2::model::CoreWeaveCloudLogsIntegrationDataflowResponse> = None;
                let mut coreweave_cloud_metrics: Option<crate::datadogV2::model::CoreWeaveCloudMetricsIntegrationDataflowResponse> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = CoreWeaveCloudIntegrationDataflowsResponse {
                    coreweave_cloud_logs,
                    coreweave_cloud_metrics,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudIntegrationDataflowsResponseVisitor)
    }
}
