// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the evaluated conversation, used to select the policy and evaluator behavior.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardMeta {
    /// The coding agent that produced the conversation. By default, a non-empty
    /// value selects the coding-agent evaluator profile, while an omitted or empty
    /// value selects the default profile. Known identifiers include `claude-code`,
    /// `codex`, `cursor`, and `github-copilot`; other identifiers are accepted.
    /// The coding-agent profile assumes the caller can ask the user to confirm
    /// Supply Chain Firewall warnings; the default profile assumes it cannot.
    #[serde(rename = "coding_agent")]
    pub coding_agent: Option<String>,
    /// Request-only confidence threshold override. A number sets the threshold
    /// for every threat category; an object supplies a global threshold and
    /// optional per-category overrides. Omit this field to use the stored
    /// sensitivity policy. This field is not returned in evaluation responses.
    #[serde(rename = "confidence_threshold")]
    pub confidence_threshold: Option<crate::datadogV2::model::AIGuardConfidenceThresholdOverride>,
    /// The environment the conversation belongs to.
    #[serde(rename = "env")]
    pub env: Option<String>,
    /// Whether to run the Sensitive Data Scanner for this request, overriding the stored policy.
    #[serde(rename = "is_sds_enabled_override")]
    pub is_sds_enabled_override: Option<bool>,
    /// The name of the service the conversation belongs to.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// A non-empty map replaces stored tool policies for this request only,
    /// keyed by tool name. Any called tool not present resolves to EVALUATE.
    /// Only takes effect when tool policies are enabled for the organization.
    #[serde(rename = "tool_policies")]
    pub tool_policies: Option<
        std::collections::BTreeMap<String, crate::datadogV2::model::AIGuardToolPolicyAction>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardMeta {
    pub fn new() -> AIGuardMeta {
        AIGuardMeta {
            coding_agent: None,
            confidence_threshold: None,
            env: None,
            is_sds_enabled_override: None,
            service: None,
            tool_policies: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn coding_agent(mut self, value: String) -> Self {
        self.coding_agent = Some(value);
        self
    }

    pub fn confidence_threshold(
        mut self,
        value: crate::datadogV2::model::AIGuardConfidenceThresholdOverride,
    ) -> Self {
        self.confidence_threshold = Some(value);
        self
    }

    pub fn env(mut self, value: String) -> Self {
        self.env = Some(value);
        self
    }

    pub fn is_sds_enabled_override(mut self, value: bool) -> Self {
        self.is_sds_enabled_override = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn tool_policies(
        mut self,
        value: std::collections::BTreeMap<String, crate::datadogV2::model::AIGuardToolPolicyAction>,
    ) -> Self {
        self.tool_policies = Some(value);
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

impl Default for AIGuardMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AIGuardMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardMetaVisitor;
        impl<'a> Visitor<'a> for AIGuardMetaVisitor {
            type Value = AIGuardMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut coding_agent: Option<String> = None;
                let mut confidence_threshold: Option<
                    crate::datadogV2::model::AIGuardConfidenceThresholdOverride,
                > = None;
                let mut env: Option<String> = None;
                let mut is_sds_enabled_override: Option<bool> = None;
                let mut service: Option<String> = None;
                let mut tool_policies: Option<
                    std::collections::BTreeMap<
                        String,
                        crate::datadogV2::model::AIGuardToolPolicyAction,
                    >,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "coding_agent" => {
                            if v.is_null() {
                                continue;
                            }
                            coding_agent =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confidence_threshold" => {
                            if v.is_null() {
                                continue;
                            }
                            confidence_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _confidence_threshold) = confidence_threshold {
                                match _confidence_threshold {
                                    crate::datadogV2::model::AIGuardConfidenceThresholdOverride::UnparsedObject(_confidence_threshold) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "env" => {
                            if v.is_null() {
                                continue;
                            }
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_sds_enabled_override" => {
                            if v.is_null() {
                                continue;
                            }
                            is_sds_enabled_override =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tool_policies" => {
                            if v.is_null() {
                                continue;
                            }
                            tool_policies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AIGuardMeta {
                    coding_agent,
                    confidence_threshold,
                    env,
                    is_sds_enabled_override,
                    service,
                    tool_policies,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardMetaVisitor)
    }
}
