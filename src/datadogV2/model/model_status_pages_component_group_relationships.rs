// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The relationships of a component group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct StatusPagesComponentGroupRelationships {
    /// The Datadog user who created the component group.
    #[serde(rename = "created_by_user")]
    pub created_by_user:
        Option<crate::datadogV2::model::StatusPagesComponentGroupRelationshipsCreatedByUser>,
    /// The group the component group belongs to.
    #[serde(rename = "group")]
    pub group: Option<crate::datadogV2::model::StatusPagesComponentGroupRelationshipsGroup>,
    /// The Datadog user who last modified the component group.
    #[serde(rename = "last_modified_by_user")]
    pub last_modified_by_user:
        Option<crate::datadogV2::model::StatusPagesComponentGroupRelationshipsLastModifiedByUser>,
    /// The status page the component group belongs to.
    #[serde(rename = "status_page")]
    pub status_page:
        Option<crate::datadogV2::model::StatusPagesComponentGroupRelationshipsStatusPage>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl StatusPagesComponentGroupRelationships {
    pub fn new() -> StatusPagesComponentGroupRelationships {
        StatusPagesComponentGroupRelationships {
            created_by_user: None,
            group: None,
            last_modified_by_user: None,
            status_page: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_by_user(
        mut self,
        value: crate::datadogV2::model::StatusPagesComponentGroupRelationshipsCreatedByUser,
    ) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn group(
        mut self,
        value: crate::datadogV2::model::StatusPagesComponentGroupRelationshipsGroup,
    ) -> Self {
        self.group = Some(value);
        self
    }

    pub fn last_modified_by_user(
        mut self,
        value: crate::datadogV2::model::StatusPagesComponentGroupRelationshipsLastModifiedByUser,
    ) -> Self {
        self.last_modified_by_user = Some(value);
        self
    }

    pub fn status_page(
        mut self,
        value: crate::datadogV2::model::StatusPagesComponentGroupRelationshipsStatusPage,
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

impl Default for StatusPagesComponentGroupRelationships {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for StatusPagesComponentGroupRelationships {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StatusPagesComponentGroupRelationshipsVisitor;
        impl<'a> Visitor<'a> for StatusPagesComponentGroupRelationshipsVisitor {
            type Value = StatusPagesComponentGroupRelationships;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_by_user: Option<
                    crate::datadogV2::model::StatusPagesComponentGroupRelationshipsCreatedByUser,
                > = None;
                let mut group: Option<
                    crate::datadogV2::model::StatusPagesComponentGroupRelationshipsGroup,
                > = None;
                let mut last_modified_by_user: Option<crate::datadogV2::model::StatusPagesComponentGroupRelationshipsLastModifiedByUser> = None;
                let mut status_page: Option<
                    crate::datadogV2::model::StatusPagesComponentGroupRelationshipsStatusPage,
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
                        "group" => {
                            if v.is_null() {
                                continue;
                            }
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = StatusPagesComponentGroupRelationships {
                    created_by_user,
                    group,
                    last_modified_by_user,
                    status_page,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(StatusPagesComponentGroupRelationshipsVisitor)
    }
}
