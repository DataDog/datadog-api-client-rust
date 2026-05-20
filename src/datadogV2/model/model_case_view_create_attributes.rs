// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes required to create a case view.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseViewCreateAttributes {
    /// The name of the view.
    #[serde(rename = "name")]
    pub name: String,
    /// The identifier of a notification rule linked to this view. When set, users subscribed to the view receive alerts for matching cases.
    #[serde(rename = "np_rule_id")]
    pub np_rule_id: Option<String>,
    /// The UUID of the project this view belongs to. Views are scoped to a single project.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The query used to filter cases in this view.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseViewCreateAttributes {
    pub fn new(name: String, project_id: String, query: String) -> CaseViewCreateAttributes {
        CaseViewCreateAttributes {
            name,
            np_rule_id: None,
            project_id,
            query,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
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

impl<'de> Deserialize<'de> for CaseViewCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseViewCreateAttributesVisitor;
        impl<'a> Visitor<'a> for CaseViewCreateAttributesVisitor {
            type Value = CaseViewCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut np_rule_id: Option<String> = None;
                let mut project_id: Option<String> = None;
                let mut query: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "np_rule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            np_rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "project_id" => {
                            project_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let project_id = project_id.ok_or_else(|| M::Error::missing_field("project_id"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content = CaseViewCreateAttributes {
                    name,
                    np_rule_id,
                    project_id,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseViewCreateAttributesVisitor)
    }
}
