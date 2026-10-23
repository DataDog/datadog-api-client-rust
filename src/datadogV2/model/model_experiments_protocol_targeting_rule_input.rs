// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A targeting rule in the protocol template.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsProtocolTargetingRuleInput {
    /// Conditions in this rule.
    #[serde(
        rename = "conditions",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub conditions: Option<Option<Vec<crate::datadogV2::model::ExperimentsProtocolConditionInput>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsProtocolTargetingRuleInput {
    pub fn new() -> ExperimentsProtocolTargetingRuleInput {
        ExperimentsProtocolTargetingRuleInput {
            conditions: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conditions(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ExperimentsProtocolConditionInput>>,
    ) -> Self {
        self.conditions = Some(value);
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

impl Default for ExperimentsProtocolTargetingRuleInput {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsProtocolTargetingRuleInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsProtocolTargetingRuleInputVisitor;
        impl<'a> Visitor<'a> for ExperimentsProtocolTargetingRuleInputVisitor {
            type Value = ExperimentsProtocolTargetingRuleInput;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditions: Option<
                    Option<Vec<crate::datadogV2::model::ExperimentsProtocolConditionInput>>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conditions" => {
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsProtocolTargetingRuleInput {
                    conditions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsProtocolTargetingRuleInputVisitor)
    }
}
