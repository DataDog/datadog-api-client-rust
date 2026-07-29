// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A query-based condition for an incident rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentRuleQueryCondition {
    /// The normalized query string.
    #[serde(
        rename = "normalized_query",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub normalized_query: Option<Option<String>>,
    /// The raw query string.
    #[serde(
        rename = "raw_query",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub raw_query: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentRuleQueryCondition {
    pub fn new() -> IncidentRuleQueryCondition {
        IncidentRuleQueryCondition {
            normalized_query: None,
            raw_query: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn normalized_query(mut self, value: Option<String>) -> Self {
        self.normalized_query = Some(value);
        self
    }

    pub fn raw_query(mut self, value: Option<String>) -> Self {
        self.raw_query = Some(value);
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

impl Default for IncidentRuleQueryCondition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IncidentRuleQueryCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentRuleQueryConditionVisitor;
        impl<'a> Visitor<'a> for IncidentRuleQueryConditionVisitor {
            type Value = IncidentRuleQueryCondition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut normalized_query: Option<Option<String>> = None;
                let mut raw_query: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "normalized_query" => {
                            normalized_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "raw_query" => {
                            raw_query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IncidentRuleQueryCondition {
                    normalized_query,
                    raw_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentRuleQueryConditionVisitor)
    }
}
