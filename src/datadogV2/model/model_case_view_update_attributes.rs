// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Attributes that can be updated on a work item view. All fields are optional; only provided fields are changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseViewUpdateAttributes {
    /// The name of the view.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Deprecated. This field is ignored. Use `POST /api/v2/cases/projects/{project_id}/notification_rules` to manage project notifications.
	#[deprecated]
    #[serde(rename = "np_rule_id")]
    pub np_rule_id: Option<String>,
    /// The query used to filter work items in this view.
    #[serde(rename = "query")]
    pub query: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl CaseViewUpdateAttributes {
    pub fn new() -> CaseViewUpdateAttributes {
        #[allow(deprecated)]
        CaseViewUpdateAttributes {
            name: None,
            np_rule_id: None,
            query: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    #[allow(deprecated)]
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    
    #[allow(deprecated)]
    pub fn np_rule_id(mut self, value: String) -> Self {
        self.np_rule_id = Some(value);
        self
    }
    
    #[allow(deprecated)]
    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for CaseViewUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseViewUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseViewUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for CaseViewUpdateAttributesVisitor {
            type Value = CaseViewUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut np_rule_id: Option<String> = None;
                let mut query: Option<String> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "np_rule_id" => {
                            if v.is_null() {
                                continue;
                            }
                            np_rule_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                #[allow(deprecated)]
                let content = CaseViewUpdateAttributes {
                    name,
                    np_rule_id,
                    query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseViewUpdateAttributesVisitor)
    }
}
