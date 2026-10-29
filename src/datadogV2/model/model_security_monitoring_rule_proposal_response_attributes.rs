// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the rule proposal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleProposalResponseAttributes {
    /// The reason no rule could be generated. Only present when the status is `failed`.
    #[serde(rename = "error")]
    pub error: Option<crate::datadogV2::model::SecurityMonitoringRuleProposalError>,
    /// The proposed Cloud SIEM detection rule in the same format as the create a detection
    /// rule endpoint. It is not saved in Cloud SIEM. Only present when the status is
    /// `completed`. Review the rule, then create it with the [create a
    /// detection rule](<https://docs.datadoghq.com/api/latest/security-monitoring/#create-a-detection-rule>)
    /// endpoint.
    #[serde(rename = "rule")]
    pub rule: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// The status of the rule proposal. `pending` and `in_progress` mean the proposed rule is
    /// still being generated. Poll the endpoint again later. `completed` and `failed` are final:
    /// `completed` proposals contain the proposed rule in `rule`, and `failed` proposals
    /// contain the reason in `error`.
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleProposalResponseAttributes {
    pub fn new(status: String) -> SecurityMonitoringRuleProposalResponseAttributes {
        SecurityMonitoringRuleProposalResponseAttributes {
            error: None,
            rule: None,
            status,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleProposalError,
    ) -> Self {
        self.error = Some(value);
        self
    }

    pub fn rule(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.rule = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleProposalResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleProposalResponseAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleProposalResponseAttributesVisitor {
            type Value = SecurityMonitoringRuleProposalResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleProposalError,
                > = None;
                let mut rule: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut status: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error" => {
                            if v.is_null() {
                                continue;
                            }
                            error = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = SecurityMonitoringRuleProposalResponseAttributes {
                    error,
                    rule,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleProposalResponseAttributesVisitor)
    }
}
