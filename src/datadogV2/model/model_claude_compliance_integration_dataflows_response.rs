// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data Datadog collects from Claude Compliance, keyed by dataflow ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ClaudeComplianceIntegrationDataflowsResponse {
    /// Audit activity logs from the Anthropic Compliance API, such as SSO sign-ins, API key lifecycle events, member and role changes, and Claude chat and file access. [Cloud SIEM](<https://docs.datadoghq.com/security/cloud_siem/>) uses these logs to detect security-sensitive activity. Requires an Anthropic Enterprise plan with the Compliance API enabled in the organization's Data and privacy settings. Without it, this dataflow collects no data. This dataflow is always on and cannot be turned off, so it appears in responses only.
    #[serde(rename = "anthropic-compliance-logs")]
    pub anthropic_compliance_logs:
        Option<crate::datadogV2::model::ClaudeComplianceComplianceLogsIntegrationDataflowResponse>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ClaudeComplianceIntegrationDataflowsResponse {
    pub fn new() -> ClaudeComplianceIntegrationDataflowsResponse {
        ClaudeComplianceIntegrationDataflowsResponse {
            anthropic_compliance_logs: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn anthropic_compliance_logs(
        mut self,
        value: crate::datadogV2::model::ClaudeComplianceComplianceLogsIntegrationDataflowResponse,
    ) -> Self {
        self.anthropic_compliance_logs = Some(value);
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

impl Default for ClaudeComplianceIntegrationDataflowsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ClaudeComplianceIntegrationDataflowsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ClaudeComplianceIntegrationDataflowsResponseVisitor;
        impl<'a> Visitor<'a> for ClaudeComplianceIntegrationDataflowsResponseVisitor {
            type Value = ClaudeComplianceIntegrationDataflowsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut anthropic_compliance_logs: Option<crate::datadogV2::model::ClaudeComplianceComplianceLogsIntegrationDataflowResponse> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "anthropic-compliance-logs" => {
                            if v.is_null() {
                                continue;
                            }
                            anthropic_compliance_logs =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ClaudeComplianceIntegrationDataflowsResponse {
                    anthropic_compliance_logs,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ClaudeComplianceIntegrationDataflowsResponseVisitor)
    }
}
