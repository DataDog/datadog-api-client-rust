// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Metrics from your CoreWeave organization: CoreWeave service metrics collected through its PromQL-compatible Prometheus endpoint, plus the `coreweave.*` namespaces from CoreWeave products such as Kubernetes, Database, Distributed File Storage, Inference, and Object Storage. Compute metrics (`coreweave.bare_metal`, `coreweave.gpu`, `coreweave.nccl`, and `coreweave.node`) are in Preview. Container metrics are not collected; install the Datadog Agent on your clusters for those.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudMetricsIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `true`; set to `false` to stop collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudMetricsIntegrationDataflowRequest {
    pub fn new() -> CoreWeaveCloudMetricsIntegrationDataflowRequest {
        CoreWeaveCloudMetricsIntegrationDataflowRequest {
            enabled: None,
            _unparsed: false,
        }
    }
    
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    
}


impl Default for CoreWeaveCloudMetricsIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudMetricsIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudMetricsIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudMetricsIntegrationDataflowRequestVisitor {
            type Value = CoreWeaveCloudMetricsIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }

                let content = CoreWeaveCloudMetricsIntegrationDataflowRequest {
                    enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudMetricsIntegrationDataflowRequestVisitor)
    }
}
