// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a case view, including the filter query and optional notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseViewAttributes {
    /// Timestamp when the view was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the view was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    /// A human-readable name for the view, displayed in the Case Management UI.
    #[serde(rename = "name")]
    pub name: String,
    /// The identifier of a notification rule linked to this view. When set, users subscribed to the view receive alerts for matching cases.
    #[serde(rename = "np_rule_id")]
    pub np_rule_id: Option<String>,
    /// The search query that determines which cases appear in this view. Uses the same syntax as the Case Management search bar (for example, `status:open priority:P1`).
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseViewAttributes {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        name: String,
        query: String,
    ) -> CaseViewAttributes {
        CaseViewAttributes {
            created_at,
            modified_at: None,
            name,
            np_rule_id: None,
            query,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn modified_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified_at = Some(value);
        self
    }

    pub fn np_rule_id(mut self, value: String) -> Self {
        self.np_rule_id = Some(value);
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

impl<'de> Deserialize<'de> for CaseViewAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseViewAttributesVisitor;
        impl<'a> Visitor<'a> for CaseViewAttributesVisitor {
            type Value = CaseViewAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut np_rule_id: Option<String> = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            if v.is_null() {
                                continue;
                            }
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "np_rule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            np_rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = CaseViewAttributes {
                    created_at,
                    modified_at,
                    name,
                    np_rule_id,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseViewAttributesVisitor)
    }
}
