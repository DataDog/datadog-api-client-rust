// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents the relationship of an escalation policy step to its targets.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyStepRelationships {
    /// A list of escalation targets for a step
    #[serde(rename = "targets")]
    pub targets: Option<crate::datadogV2::model::EscalationTargets>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyStepRelationships {
    pub fn new() -> EscalationPolicyStepRelationships {
        EscalationPolicyStepRelationships {
            targets: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn targets(mut self, value: crate::datadogV2::model::EscalationTargets) -> Self {
        self.targets = Some(value);
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

impl Default for EscalationPolicyStepRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EscalationPolicyStepRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyStepRelationshipsVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyStepRelationshipsVisitor {
            type Value = EscalationPolicyStepRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut targets: Option<crate::datadogV2::model::EscalationTargets> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "targets" => {
                            if v.is_null() {
                                continue;
                            }
                            targets = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EscalationPolicyStepRelationships {
                    targets,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyStepRelationshipsVisitor)
    }
}
