// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Audit activity logs from the Anthropic Compliance API, such as SSO sign-ins, API key lifecycle events, member and role changes, and Claude chat and file access. [Cloud SIEM](<https://docs.datadoghq.com/security/cloud_siem/>) uses these logs to detect security-sensitive activity. Requires an Anthropic Enterprise plan with the Compliance API enabled in the organization's Data and privacy settings. Without it, this dataflow collects no data. This dataflow is always on and cannot be turned off, so it appears in responses only.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
    /// Whether Datadog collects this data. Always `true`, because this collection cannot be turned off.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Read-only collection status of a dataflow.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
    pub fn new() -> ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
        ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
            enabled: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountDataflowStatus,
    ) -> Self {
        self.status = Some(value);
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

impl Default for ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClaudeComplianceComplianceLogsIntegrationDataflowResponseVisitor;
        impl<'a> Visitor<'a> for ClaudeComplianceComplianceLogsIntegrationDataflowResponseVisitor {
            type Value = ClaudeComplianceComplianceLogsIntegrationDataflowResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ClaudeComplianceComplianceLogsIntegrationDataflowResponse {
                    enabled,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ClaudeComplianceComplianceLogsIntegrationDataflowResponseVisitor)
    }
}
