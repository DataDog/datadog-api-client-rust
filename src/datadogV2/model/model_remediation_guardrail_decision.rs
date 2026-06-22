// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The guardrail decision applied to a plan or investigation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RemediationGuardrailDecision {
    /// The verdict a guardrail applied to a plan or investigation.
    #[serde(rename = "decision")]
    pub decision: Option<crate::datadogV2::model::RemediationGuardrailVerdict>,
    /// ID of the matching guardrail. Set when the decision is not denied; may be empty when denied because no rule matched.
    #[serde(rename = "guardrail_id")]
    pub guardrail_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RemediationGuardrailDecision {
    pub fn new() -> RemediationGuardrailDecision {
        RemediationGuardrailDecision {
            decision: None,
            guardrail_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn decision(mut self, value: crate::datadogV2::model::RemediationGuardrailVerdict) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn guardrail_id(mut self, value: String) -> Self {
        self.guardrail_id = Some(value);
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

impl Default for RemediationGuardrailDecision {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RemediationGuardrailDecision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RemediationGuardrailDecisionVisitor;
        impl<'a> Visitor<'a> for RemediationGuardrailDecisionVisitor {
            type Value = RemediationGuardrailDecision;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut decision: Option<crate::datadogV2::model::RemediationGuardrailVerdict> =
                    None;
                let mut guardrail_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "decision" => {
                            if v.is_null() {
                                continue;
                            }
                            decision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _decision) = decision {
                                match _decision {
                                    crate::datadogV2::model::RemediationGuardrailVerdict::UnparsedObject(_decision) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "guardrail_id" => {
                            if v.is_null() {
                                continue;
                            }
                            guardrail_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RemediationGuardrailDecision {
                    decision,
                    guardrail_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RemediationGuardrailDecisionVisitor)
    }
}
