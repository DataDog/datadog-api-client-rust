// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A targeting rule in a saved filter. A rule matches when all its conditions match.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SavedFilterTargetingRule {
    /// Conditions combined with AND within this targeting rule.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::SavedFilterCondition>,
    /// The ID of the targeting rule.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SavedFilterTargetingRule {
    pub fn new(
        conditions: Vec<crate::datadogV2::model::SavedFilterCondition>,
        id: uuid::Uuid,
    ) -> SavedFilterTargetingRule {
        SavedFilterTargetingRule {
            conditions,
            id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SavedFilterTargetingRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SavedFilterTargetingRuleVisitor;
        impl<'a> Visitor<'a> for SavedFilterTargetingRuleVisitor {
            type Value = SavedFilterTargetingRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditions: Option<Vec<crate::datadogV2::model::SavedFilterCondition>> =
                    None;
                let mut id: Option<uuid::Uuid> = None;
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
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = SavedFilterTargetingRule {
                    conditions,
                    id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SavedFilterTargetingRuleVisitor)
    }
}
