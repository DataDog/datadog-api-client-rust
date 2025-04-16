// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the relationship to a collection of steps within an escalation policy. Contains an array of step data references.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyDataRelationshipsSteps {
    /// An array of references to the steps defined in this escalation policy.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyDataRelationshipsSteps {
    pub fn new() -> EscalationPolicyDataRelationshipsSteps {
        EscalationPolicyDataRelationshipsSteps {
            data: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Vec<crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItems>,
    ) -> Self {
        self.data = Some(value);
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

impl Default for EscalationPolicyDataRelationshipsSteps {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for EscalationPolicyDataRelationshipsSteps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyDataRelationshipsStepsVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyDataRelationshipsStepsVisitor {
            type Value = EscalationPolicyDataRelationshipsSteps;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Vec<crate::datadogV2::model::EscalationPolicyDataRelationshipsStepsDataItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = EscalationPolicyDataRelationshipsSteps {
                    data,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyDataRelationshipsStepsVisitor)
    }
}
