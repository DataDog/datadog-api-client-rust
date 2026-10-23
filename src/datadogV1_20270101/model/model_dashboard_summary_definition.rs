// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Dashboard definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardSummaryDefinition {
    /// Object describing the creator of the shared element.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::Creator>,
    /// Date of creation of the dashboard.
    #[serde(
        rename = "created",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// URL to the icon of the dashboard.
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option")]
    pub icon: Option<Option<String>>,
    /// ID of the dashboard.
    #[serde(rename = "id")]
    pub id: Option<crate::datadogV1_20270101::model::DashboardSummaryID>,
    /// The short name of the integration.
    #[serde(
        rename = "integration_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub integration_id: Option<Option<String>>,
    /// Whether the dashboard is in the favorites.
    #[serde(rename = "is_favorite")]
    pub is_favorite: Option<bool>,
    /// Whether the dashboard is read only.
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    /// Whether the dashboard is publicly shared.
    #[serde(rename = "is_shared")]
    pub is_shared: Option<bool>,
    /// Date when the dashboard was last viewed.
    #[serde(
        rename = "last_view_date",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_view_date: Option<Option<String>>,
    /// Date of last edition of the dashboard.
    #[serde(
        rename = "modified",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub modified: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// Popularity of the dashboard.
    #[serde(rename = "popularity")]
    pub popularity: Option<i32>,
    /// List of team names representing ownership of the dashboard.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    /// Title of the dashboard.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The type of the dashboard.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL path to the dashboard.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardSummaryDefinition {
    pub fn new() -> DashboardSummaryDefinition {
        DashboardSummaryDefinition {
            author: None,
            created: None,
            icon: None,
            id: None,
            integration_id: None,
            is_favorite: None,
            is_read_only: None,
            is_shared: None,
            last_view_date: None,
            modified: None,
            popularity: None,
            tags: None,
            title: None,
            type_: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn icon(mut self, value: Option<String>) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn id(mut self, value: crate::datadogV1_20270101::model::DashboardSummaryID) -> Self {
        self.id = Some(value);
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

    pub fn last_view_date(mut self, value: Option<String>) -> Self {
        self.last_view_date = Some(value);
        self
    }

    pub fn modified(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
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

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for DashboardSummaryDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DashboardSummaryDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardSummaryDefinitionVisitor;
        impl<'a> Visitor<'a> for DashboardSummaryDefinitionVisitor {
            type Value = DashboardSummaryDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV1::model::Creator> = None;
                let mut created: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut icon: Option<Option<String>> = None;
                let mut id: Option<crate::datadogV1_20270101::model::DashboardSummaryID> = None;
                let mut integration_id: Option<Option<String>> = None;
                let mut is_favorite: Option<bool> = None;
                let mut is_read_only: Option<bool> = None;
                let mut is_shared: Option<bool> = None;
                let mut last_view_date: Option<Option<String>> = None;
                let mut modified: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut popularity: Option<i32> = None;
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut title: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            created = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "icon" => {
                            icon = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _id) = id {
                                match _id {
                                    crate::datadogV1_20270101::model::DashboardSummaryID::UnparsedObject(_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
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
                        "last_view_date" => {
                            last_view_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified" => {
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
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = DashboardSummaryDefinition {
                    author,
                    created,
                    icon,
                    id,
                    integration_id,
                    is_favorite,
                    is_read_only,
                    is_shared,
                    last_view_date,
                    modified,
                    popularity,
                    tags,
                    title,
                    type_,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardSummaryDefinitionVisitor)
    }
}
