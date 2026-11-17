// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Logs from your Nebius tenant: Kubernetes control plane and audit logs, tenant-level audit events, compute serial console logs, cloud apps and AI inference logs, PostgreSQL logs, and MLflow logs, collected from the services named in `settings.enabled_services`. `enabled_services` is the sole source of truth for what is collected; `enabled` set to `false` stops log collection, while `true` on its own changes nothing. Defaults to no services.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NebiusCloudLogsIntegrationDataflowRequest {
    /// Whether Datadog collects logs. Defaults to `false`. Set to `false` to stop log collection; sending `true` on its own changes nothing, because `settings.enabled_services` is the sole source of truth.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Settings of the logs dataflow. Only the fields provided are changed.
    #[serde(rename = "settings")]
    pub settings:
        Option<crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowSettingsRequest>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NebiusCloudLogsIntegrationDataflowRequest {
    pub fn new() -> NebiusCloudLogsIntegrationDataflowRequest {
        NebiusCloudLogsIntegrationDataflowRequest {
            enabled: None,
            settings: None,
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn settings(
        mut self,
        value: crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowSettingsRequest,
    ) -> Self {
        self.settings = Some(value);
        self
    }
}

impl Default for NebiusCloudLogsIntegrationDataflowRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NebiusCloudLogsIntegrationDataflowRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NebiusCloudLogsIntegrationDataflowRequestVisitor;
        impl<'a> Visitor<'a> for NebiusCloudLogsIntegrationDataflowRequestVisitor {
            type Value = NebiusCloudLogsIntegrationDataflowRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut settings: Option<
                    crate::datadogV2::model::NebiusCloudLogsIntegrationDataflowSettingsRequest,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "settings" => {
                            if v.is_null() {
                                continue;
                            }
                            settings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }

                let content = NebiusCloudLogsIntegrationDataflowRequest {
                    enabled,
                    settings,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NebiusCloudLogsIntegrationDataflowRequestVisitor)
    }
}
