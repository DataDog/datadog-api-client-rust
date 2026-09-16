// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The result of the AI Guard evaluation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AIGuardEvaluateResponse {
    /// The action recommendation from the AI Guard evaluation.
    #[serde(rename = "action")]
    pub action: crate::datadogV2::model::AIGuardAction,
    /// The overall threat probability score across all evaluated tags.
    #[serde(rename = "global_prob")]
    pub global_prob: Option<f64>,
    /// Whether blocking mode is enabled for this organization.
    #[serde(rename = "is_blocking_enabled")]
    pub is_blocking_enabled: bool,
    /// A human-readable explanation of the action recommendation.
    #[serde(rename = "reason")]
    pub reason: String,
    /// Sensitive data findings detected in the evaluated conversation.
    #[serde(rename = "sds_findings")]
    pub sds_findings: Option<Vec<crate::datadogV2::model::AIGuardSdsFinding>>,
    /// Probability scores for each evaluated threat tag.
    #[serde(rename = "tag_probs")]
    pub tag_probs: std::collections::BTreeMap<String, f64>,
    /// Security threat tags detected in the evaluated conversation.
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AIGuardEvaluateResponse {
    pub fn new(
        action: crate::datadogV2::model::AIGuardAction,
        is_blocking_enabled: bool,
        reason: String,
        tag_probs: std::collections::BTreeMap<String, f64>,
        tags: Vec<String>,
    ) -> AIGuardEvaluateResponse {
        AIGuardEvaluateResponse {
            action,
            global_prob: None,
            is_blocking_enabled,
            reason,
            sds_findings: None,
            tag_probs,
            tags,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn global_prob(mut self, value: f64) -> Self {
        self.global_prob = Some(value);
        self
    }

    pub fn sds_findings(mut self, value: Vec<crate::datadogV2::model::AIGuardSdsFinding>) -> Self {
        self.sds_findings = Some(value);
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

impl<'de> Deserialize<'de> for AIGuardEvaluateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AIGuardEvaluateResponseVisitor;
        impl<'a> Visitor<'a> for AIGuardEvaluateResponseVisitor {
            type Value = AIGuardEvaluateResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<crate::datadogV2::model::AIGuardAction> = None;
                let mut global_prob: Option<f64> = None;
                let mut is_blocking_enabled: Option<bool> = None;
                let mut reason: Option<String> = None;
                let mut sds_findings: Option<Vec<crate::datadogV2::model::AIGuardSdsFinding>> =
                    None;
                let mut tag_probs: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action) = action {
                                match _action {
                                    crate::datadogV2::model::AIGuardAction::UnparsedObject(
                                        _action,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "global_prob" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            global_prob =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_blocking_enabled" => {
                            is_blocking_enabled =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reason" => {
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sds_findings" => {
                            if v.is_null() {
                                continue;
                            }
                            sds_findings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_probs" => {
                            tag_probs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let is_blocking_enabled = is_blocking_enabled
                    .ok_or_else(|| M::Error::missing_field("is_blocking_enabled"))?;
                let reason = reason.ok_or_else(|| M::Error::missing_field("reason"))?;
                let tag_probs = tag_probs.ok_or_else(|| M::Error::missing_field("tag_probs"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = AIGuardEvaluateResponse {
                    action,
                    global_prob,
                    is_blocking_enabled,
                    reason,
                    sds_findings,
                    tag_probs,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AIGuardEvaluateResponseVisitor)
    }
}
