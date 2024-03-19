// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A dashboard within a list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardListItem {
    /// Creator of the object.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV2::model::Creator>,
    /// Date of creation of the dashboard.
    #[serde(rename = "created")]
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    /// URL to the icon of the dashboard.
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option")]
    pub icon: Option<Option<String>>,
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: String,
    /// The short name of the integration.
    #[serde(
        rename = "integration_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub integration_id: Option<Option<String>>,
    /// Whether or not the dashboard is in the favorites.
    #[serde(rename = "is_favorite")]
    pub is_favorite: Option<bool>,
    /// Whether or not the dashboard is read only.
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Whether the dashboard is publicly shared or not.
    #[serde(rename = "is_shared")]
    pub is_shared: Option<bool>,
    /// Date of last edition of the dashboard.
    #[serde(rename = "modified")]
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    /// Popularity of the dashboard.
    #[serde(rename = "popularity")]
    pub popularity: Option<i32>,
    /// List of team names representing ownership of a dashboard.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Title of the dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The type of the dashboard.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::DashboardType,
    /// URL path to the dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardListItem {
    pub fn new(id: String, type_: crate::datadogV2::model::DashboardType) -> DashboardListItem {
        DashboardListItem {
            author: None,
            created: None,
            icon: None,
            id,
            integration_id: None,
            is_favorite: None,
            is_read_only: None,
            is_shared: None,
            modified: None,
            popularity: None,
            tags: None,
            title: None,
            type_,
            url: None,
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV2::model::Creator) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn icon(mut self, value: Option<String>) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn integration_id(mut self, value: Option<String>) -> Self {
        self.integration_id = Some(value);
        self
    }

    pub fn is_favorite(mut self, value: bool) -> Self {
        self.is_favorite = Some(value);
        self
    }

    pub fn is_read_only(mut self, value: bool) -> Self {
        self.is_read_only = Some(value);
        self
    }

    pub fn is_shared(mut self, value: bool) -> Self {
        self.is_shared = Some(value);
        self
    }

    pub fn modified(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn popularity(mut self, value: i32) -> Self {
        self.popularity = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DashboardListItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardListItemVisitor;
        impl<'a> Visitor<'a> for DashboardListItemVisitor {
            type Value = DashboardListItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::Creator> = None;
                let mut created: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut icon: Option<Option<String>> = None;
                let mut id: Option<String> = None;
                let mut integration_id: Option<Option<String>> = None;
                let mut is_favorite: Option<bool> = None;
                let mut is_read_only: Option<bool> = None;
                let mut is_shared: Option<bool> = None;
                let mut modified: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut popularity: Option<i32> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::DashboardType> = None;
                let mut url: Option<String> = None;
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
                        "icon" => {
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "integration_id" => {
                            integration_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorite" => {
                            if v.is_null() {
                                continue;
                            }
                            is_favorite =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_read_only" => {
                            if v.is_null() {
                                continue;
                            }
                            is_read_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_shared" => {
                            if v.is_null() {
                                continue;
                            }
                            is_shared = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
                            if v.is_null() {
                                continue;
                            }
                            modified = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "popularity" => {
                            if v.is_null() {
                                continue;
                            }
                            popularity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::DashboardType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = DashboardListItem {
                    author,
                    created,
                    icon,
                    id,
                    integration_id,
                    is_favorite,
                    is_read_only,
                    is_shared,
                    modified,
                    popularity,
                    tags,
                    title,
                    type_,
                    url,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardListItemVisitor)
    }
}
