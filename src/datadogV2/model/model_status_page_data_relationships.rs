// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The relationships of a status page.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPageDataRelationships {
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::StatusPageDataRelationshipsCreatedByUser>,
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user:
        Option<crate::datadogV2::model::StatusPageDataRelationshipsLastModifiedByUser>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPageDataRelationships {
    pub fn new() -> StatusPageDataRelationships {
        StatusPageDataRelationships {
            created_by_user: None,
            last_modified_by_user: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_user(
        mut self,
        value: crate::datadogV2::model::StatusPageDataRelationshipsCreatedByUser,
    ) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::StatusPageDataRelationshipsLastModifiedByUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
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

impl Default for StatusPageDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StatusPageDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPageDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for StatusPageDataRelationshipsVisitor {
            type Value = StatusPageDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<
                    crate::datadogV2::model::StatusPageDataRelationshipsCreatedByUser,
                > = None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::StatusPageDataRelationshipsLastModifiedByUser,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = StatusPageDataRelationships {
                    created_by_user,
                    last_modified_by_user,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPageDataRelationshipsVisitor)
    }
}
