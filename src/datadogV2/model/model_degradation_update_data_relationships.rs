// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships of a degradation update resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DegradationUpdateDataRelationships {
    /// A user relationship of a degradation update.
    #[serde(rename = "created_by_user")]
    pub created_by_user: Option<crate::datadogV2::model::DegradationUpdateDataRelationshipsUser>,
    /// The degradation relationship of a degradation update.
    #[serde(rename = "degradation")]
    pub degradation: Option<crate::datadogV2::model::DegradationUpdateDataRelationshipsDegradation>,
    /// A user relationship of a degradation update.
    #[serde(rename = "deleted_by_user")]
    pub deleted_by_user: Option<crate::datadogV2::model::DegradationUpdateDataRelationshipsUser>,
    /// A user relationship of a degradation update.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user:
        Option<crate::datadogV2::model::DegradationUpdateDataRelationshipsUser>,
    /// The status page relationship of a degradation update.
    #[serde(rename = "status_page")]
    pub status_page: Option<crate::datadogV2::model::DegradationUpdateDataRelationshipsStatusPage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DegradationUpdateDataRelationships {
    pub fn new() -> DegradationUpdateDataRelationships {
        DegradationUpdateDataRelationships {
            created_by_user: None,
            degradation: None,
            deleted_by_user: None,
            last_modified_by_user: None,
            status_page: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_user(
        mut self,
        value: crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
    ) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn degradation(
        mut self,
        value: crate::datadogV2::model::DegradationUpdateDataRelationshipsDegradation,
    ) -> Self {
        self.degradation = Some(value);
        self
    }

    pub fn deleted_by_user(
        mut self,
        value: crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
    ) -> Self {
        self.deleted_by_user = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn status_page(
        mut self,
        value: crate::datadogV2::model::DegradationUpdateDataRelationshipsStatusPage,
    ) -> Self {
        self.status_page = Some(value);
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

impl Default for DegradationUpdateDataRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DegradationUpdateDataRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DegradationUpdateDataRelationshipsVisitor;
        impl<'a> Visitor<'a> for DegradationUpdateDataRelationshipsVisitor {
            type Value = DegradationUpdateDataRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<
                    crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
                > = None;
                let mut degradation: Option<
                    crate::datadogV2::model::DegradationUpdateDataRelationshipsDegradation,
                > = None;
                let mut deleted_by_user: Option<
                    crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
                > = None;
                let mut last_modified_by_user: Option<
                    crate::datadogV2::model::DegradationUpdateDataRelationshipsUser,
                > = None;
                let mut status_page: Option<
                    crate::datadogV2::model::DegradationUpdateDataRelationshipsStatusPage,
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
                        "degradation" => {
                            if v.is_null() {
                                continue;
                            }
                            degradation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            deleted_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_modified_by_user" => {
                            if v.is_null() {
                                continue;
                            }
                            last_modified_by_user =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status_page" => {
                            if v.is_null() {
                                continue;
                            }
                            status_page =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DegradationUpdateDataRelationships {
                    created_by_user,
                    degradation,
                    deleted_by_user,
                    last_modified_by_user,
                    status_page,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DegradationUpdateDataRelationshipsVisitor)
    }
}
