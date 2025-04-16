// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines the attributes that can be updated for an escalation policy, such as description, name, resolution behavior, retries, and steps.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EscalationPolicyUpdateRequestDataAttributes {
    /// Provides a detailed text description of the escalation policy.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Specifies the name of the escalation policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates whether the page is automatically resolved when the policy ends.
    #[serde(rename = "resolve_page_on_policy_end")]
    pub resolve_page_on_policy_end: Option<bool>,
    /// Specifies how many times the escalation sequence is retried if there is no response.
    #[serde(rename = "retries")]
    pub retries: Option<i64>,
    /// A list of escalation steps, each defining assignment, escalation timeout, and targets.
    #[serde(rename = "steps")]
    pub steps: Vec<crate::datadogV2::model::EscalationPolicyUpdateRequestDataAttributesStepsItems>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl EscalationPolicyUpdateRequestDataAttributes {
    pub fn new(
        name: String,
        steps: Vec<crate::datadogV2::model::EscalationPolicyUpdateRequestDataAttributesStepsItems>,
    ) -> EscalationPolicyUpdateRequestDataAttributes {
        EscalationPolicyUpdateRequestDataAttributes {
            description: None,
            name,
            resolve_page_on_policy_end: None,
            retries: None,
            steps,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn resolve_page_on_policy_end(mut self, value: bool) -> Self {
        self.resolve_page_on_policy_end = Some(value);
        self
    }

    pub fn retries(mut self, value: i64) -> Self {
        self.retries = Some(value);
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

impl<'de> Deserialize<'de> for EscalationPolicyUpdateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct EscalationPolicyUpdateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for EscalationPolicyUpdateRequestDataAttributesVisitor {
            type Value = EscalationPolicyUpdateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut name: Option<String> = None;
                let mut resolve_page_on_policy_end: Option<bool> = None;
                let mut retries: Option<i64> = None;
                let mut steps: Option<Vec<crate::datadogV2::model::EscalationPolicyUpdateRequestDataAttributesStepsItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resolve_page_on_policy_end" => {
                            if v.is_null() {
                                continue;
                            }
                            resolve_page_on_policy_end =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retries" => {
                            if v.is_null() {
                                continue;
                            }
                            retries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "steps" => {
                            steps = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let steps = steps.ok_or_else(|| M::Error::missing_field("steps"))?;

                let content = EscalationPolicyUpdateRequestDataAttributes {
                    description,
                    name,
                    resolve_page_on_policy_end,
                    retries,
                    steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(EscalationPolicyUpdateRequestDataAttributesVisitor)
    }
}
