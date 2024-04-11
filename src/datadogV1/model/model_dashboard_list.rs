// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Your Datadog Dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardList {
    /// Object describing the creator of the shared element.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::Creator>,
    /// Date of creation of the dashboard list.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// The number of dashboards in the list.
    #[serde(rename = "dashboard_count")]
    pub dashboard_count: Option<i64>,
    /// The ID of the dashboard list.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Whether or not the list is in the favorites.
    #[serde(rename = "is_favorite")]
    pub is_favorite: Option<bool>,
    /// Date of last edition of the dashboard list.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// The name of the dashboard list.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of dashboard list.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardList {
    pub fn new(name: String) -> DashboardList {
        DashboardList {
            author: None,
            created: None,
            dashboard_count: None,
            id: None,
            is_favorite: None,
            modified: None,
            name,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn dashboard_count(mut self, value: i64) -> Self {
        self.dashboard_count = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DashboardList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListVisitor;
        impl<'a> Visitor<'a> for DashboardListVisitor {
            type Value = DashboardList;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV1::model::Creator> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut dashboard_count: Option<i64> = None;
                let mut id: Option<i64> = None;
                let mut is_favorite: Option<bool> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut name: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            if v.is_null() {
                                continue;
                            }
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created" => {
                            if v.is_null() {
                                continue;
                            }
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dashboard_count" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboard_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorite" => {
                            if v.is_null() {
                                continue;
                            }
                            is_favorite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = DashboardList {
                    author,
                    created,
                    dashboard_count,
                    id,
                    is_favorite,
                    modified,
                    name,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListVisitor)
    }
}
