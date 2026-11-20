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
pub struct CoreWeaveCloudLogsIntegrationDataflowResponse {
    /// Whether Datadog collects this data. Absent when the dataflow has never been configured, in which case the documented default applies.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CoreWeaveCloudLogsIntegrationDataflowResponse {
    pub fn new() -> CoreWeaveCloudLogsIntegrationDataflowResponse {
        CoreWeaveCloudLogsIntegrationDataflowResponse {
            enabled: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for CoreWeaveCloudLogsIntegrationDataflowResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CoreWeaveCloudLogsIntegrationDataflowResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CoreWeaveCloudLogsIntegrationDataflowResponseVisitor;
        impl<'a> Visitor<'a> for CoreWeaveCloudLogsIntegrationDataflowResponseVisitor {
            type Value = CoreWeaveCloudLogsIntegrationDataflowResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
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
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = CoreWeaveCloudLogsIntegrationDataflowResponse {
                    enabled,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CoreWeaveCloudLogsIntegrationDataflowResponseVisitor)
    }
}
