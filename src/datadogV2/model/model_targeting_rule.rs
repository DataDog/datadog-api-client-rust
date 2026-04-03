// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Targeting rule details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TargetingRule {
    /// Conditions evaluated by this targeting rule.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::Condition>,
    /// The timestamp when the targeting rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The unique identifier of the targeting rule.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The timestamp when the targeting rule was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TargetingRule {
    pub fn new(
        conditions: Vec<crate::datadogV2::model::Condition>,
        created_at: chrono::DateTime<chrono::Utc>,
        id: uuid::Uuid,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> TargetingRule {
        TargetingRule {
            conditions,
            created_at,
            id,
            updated_at,
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

impl<'de> Deserialize<'de> for TargetingRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TargetingRuleVisitor;
        impl<'a> Visitor<'a> for TargetingRuleVisitor {
            type Value = TargetingRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conditions: Option<Vec<crate::datadogV2::model::Condition>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
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
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = TargetingRule {
                    conditions,
                    created_at,
                    id,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TargetingRuleVisitor)
    }
}
