// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Settings of the logs dataflow. Only the fields provided are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudLogsIntegrationDataflowSettingsRequest {
    /// Nebius log integrations to collect. One of `nebius-cloud-compute`, `nebius-cloud-kubernetes`, `nebius-cloud-mlflow`, `nebius-cloud-postgresql`, `nebius-cloud-applications`, or `default` for the streams shared by every service. A non-empty list is the sole source of truth and replaces the stored list: those services are collected and any others stop. An empty list stops log collection entirely. Omit it to keep the stored list.
    #[serde(rename = "enabled_services")]
    pub enabled_services: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudLogsIntegrationDataflowSettingsRequest {
    pub fn new() -> NebiusCloudLogsIntegrationDataflowSettingsRequest {
        NebiusCloudLogsIntegrationDataflowSettingsRequest {
            enabled_services: None,
            _unparsed: false,
        }
    }

    pub fn enabled_services(mut self, value: Vec<String>) -> Self {
        self.enabled_services = Some(value);
        self
    }
}

impl Default for NebiusCloudLogsIntegrationDataflowSettingsRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudLogsIntegrationDataflowSettingsRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudLogsIntegrationDataflowSettingsRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudLogsIntegrationDataflowSettingsRequestVisitor {
            type Value = NebiusCloudLogsIntegrationDataflowSettingsRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled_services: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled_services" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled_services =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudLogsIntegrationDataflowSettingsRequest {
                    enabled_services,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudLogsIntegrationDataflowSettingsRequestVisitor)
    }
}
