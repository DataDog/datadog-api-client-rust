// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Service logs for CoreWeave-managed components in each CoreWeave region: the CKS control plane (such as `kube-apiserver` and `etcd`), CoreWeave add-on services in the `cw-*` and `kube-system` namespaces, SUNK, and node system logs such as `kubelet` and `containerd`. Logs from your own workloads and Kubernetes audit logs are not collected. Logs are sent with `source:coreweave`, with CoreWeave log labels such as `namespace`, `app`, and `systemd_unit` added as tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoreWeaveCloudLogsIntegrationDataflowRequest {
    /// Whether Datadog collects this data. Defaults to `false`; set to `true` to start collection.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudLogsIntegrationDataflowRequest {
    pub fn new() -> CoreWeaveCloudLogsIntegrationDataflowRequest {
        CoreWeaveCloudLogsIntegrationDataflowRequest {
            enabled: None,
            _unparsed: false,
        }
    }
    
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    
}


impl Default for CoreWeaveCloudLogsIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudLogsIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudLogsIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudLogsIntegrationDataflowRequestVisitor {
            type Value = CoreWeaveCloudLogsIntegrationDataflowRequest;

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

                let content = CoreWeaveCloudLogsIntegrationDataflowRequest {
                    enabled,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudLogsIntegrationDataflowRequestVisitor)
    }
}
