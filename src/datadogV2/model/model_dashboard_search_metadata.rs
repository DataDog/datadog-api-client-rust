// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardSearchMetadata {
    /// User information.
    #[serde(rename = "author")]
    pub author: crate::datadogV2::model::DashboardSearchUser,
    /// Time at which the dashboard was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Time at which the dashboard was deleted, or null if not deleted.
    #[serialize_always]
    #[serde(rename = "deleted_at")]
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// List of domains the dashboard is allowed to be embedded in.
    #[serialize_always]
    #[serde(rename = "embeddable_domains")]
    pub embeddable_domains: Option<Vec<String>>,
    /// Dashboard experience type.
    #[serde(rename = "experience_type")]
    pub experience_type: String,
    /// When the public dashboard link will expire.
    #[serialize_always]
    #[serde(rename = "expiration")]
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether the dashboard has monitors.
    #[serialize_always]
    #[serde(rename = "has_monitors")]
    pub has_monitors: Option<bool>,
    /// Whether the dashboard is favorited by the user.
    #[serde(rename = "is_favorited")]
    pub is_favorited: bool,
    /// Whether the public dashboard owner is deactivated.
    #[serde(rename = "is_public_dashboard_ownerless")]
    pub is_public_dashboard_ownerless: bool,
    /// Whether the dashboard is shared publicly.
    #[serde(rename = "is_shared")]
    pub is_shared: bool,
    /// Last time the dashboard was accessed.
    #[serialize_always]
    #[serde(rename = "last_accessed")]
    pub last_accessed: Option<chrono::DateTime<chrono::Utc>>,
    /// Time at which the dashboard was last updated.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// Relative measure of dashboard popularity.
    #[serde(rename = "popularity")]
    pub popularity: f64,
    /// Published title of the public dashboard.
    #[serde(rename = "public_title")]
    pub public_title: String,
    /// Quality score of the dashboard.
    #[serialize_always]
    #[serde(rename = "quality_score")]
    pub quality_score: Option<f64>,
    /// List of email addresses for invite-only public dashboards.
    #[serialize_always]
    #[serde(rename = "share_list")]
    pub share_list: Option<Vec<String>>,
    /// Share type of the public dashboard.
    #[serde(rename = "share_type")]
    pub share_type: String,
    /// User information.
    #[serde(rename = "shared_by")]
    pub shared_by: crate::datadogV2::model::DashboardSearchUser,
    /// Status of the public dashboard.
    #[serde(rename = "status")]
    pub status: String,
    /// Unique public dashboard token.
    #[serde(rename = "token")]
    pub token: String,
    /// Dashboard type.
    #[serde(rename = "type")]
    pub type_: String,
    /// URL path to the dashboard.
    #[serde(rename = "url")]
    pub url: String,
    /// Number of widgets in the dashboard.
    #[serde(rename = "widget_count")]
    pub widget_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardSearchMetadata {
    pub fn new(
        author: crate::datadogV2::model::DashboardSearchUser,
        created_at: chrono::DateTime<chrono::Utc>,
        deleted_at: Option<chrono::DateTime<chrono::Utc>>,
        embeddable_domains: Option<Vec<String>>,
        experience_type: String,
        expiration: Option<chrono::DateTime<chrono::Utc>>,
        has_monitors: Option<bool>,
        is_favorited: bool,
        is_public_dashboard_ownerless: bool,
        is_shared: bool,
        last_accessed: Option<chrono::DateTime<chrono::Utc>>,
        modified_at: chrono::DateTime<chrono::Utc>,
        popularity: f64,
        public_title: String,
        quality_score: Option<f64>,
        share_list: Option<Vec<String>>,
        share_type: String,
        shared_by: crate::datadogV2::model::DashboardSearchUser,
        status: String,
        token: String,
        type_: String,
        url: String,
        widget_count: i64,
    ) -> DashboardSearchMetadata {
        DashboardSearchMetadata {
            author,
            created_at,
            deleted_at,
            embeddable_domains,
            experience_type,
            expiration,
            has_monitors,
            is_favorited,
            is_public_dashboard_ownerless,
            is_shared,
            last_accessed,
            modified_at,
            popularity,
            public_title,
            quality_score,
            share_list,
            share_type,
            shared_by,
            status,
            token,
            type_,
            url,
            widget_count,
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

impl<'de> Deserialize<'de> for DashboardSearchMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardSearchMetadataVisitor;
        impl<'a> Visitor<'a> for DashboardSearchMetadataVisitor {
            type Value = DashboardSearchMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<crate::datadogV2::model::DashboardSearchUser> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut deleted_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut embeddable_domains: Option<Option<Vec<String>>> = None;
                let mut experience_type: Option<String> = None;
                let mut expiration: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut has_monitors: Option<Option<bool>> = None;
                let mut is_favorited: Option<bool> = None;
                let mut is_public_dashboard_ownerless: Option<bool> = None;
                let mut is_shared: Option<bool> = None;
                let mut last_accessed: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut popularity: Option<f64> = None;
                let mut public_title: Option<String> = None;
                let mut quality_score: Option<Option<f64>> = None;
                let mut share_list: Option<Option<Vec<String>>> = None;
                let mut share_type: Option<String> = None;
                let mut shared_by: Option<crate::datadogV2::model::DashboardSearchUser> = None;
                let mut status: Option<String> = None;
                let mut token: Option<String> = None;
                let mut type_: Option<String> = None;
                let mut url: Option<String> = None;
                let mut widget_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "author" => {
                            author = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deleted_at" => {
                            deleted_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "embeddable_domains" => {
                            embeddable_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experience_type" => {
                            experience_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expiration" => {
                            expiration = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_monitors" => {
                            has_monitors =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_favorited" => {
                            is_favorited =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_public_dashboard_ownerless" => {
                            is_public_dashboard_ownerless =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_shared" => {
                            is_shared = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_accessed" => {
                            last_accessed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "popularity" => {
                            popularity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_title" => {
                            public_title =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quality_score" => {
                            quality_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_list" => {
                            share_list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "share_type" => {
                            share_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "shared_by" => {
                            shared_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "token" => {
                            token = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widget_count" => {
                            widget_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let author = author.ok_or_else(|| M::Error::missing_field("author"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let deleted_at = deleted_at.ok_or_else(|| M::Error::missing_field("deleted_at"))?;
                let embeddable_domains = embeddable_domains
                    .ok_or_else(|| M::Error::missing_field("embeddable_domains"))?;
                let experience_type =
                    experience_type.ok_or_else(|| M::Error::missing_field("experience_type"))?;
                let expiration = expiration.ok_or_else(|| M::Error::missing_field("expiration"))?;
                let has_monitors =
                    has_monitors.ok_or_else(|| M::Error::missing_field("has_monitors"))?;
                let is_favorited =
                    is_favorited.ok_or_else(|| M::Error::missing_field("is_favorited"))?;
                let is_public_dashboard_ownerless = is_public_dashboard_ownerless
                    .ok_or_else(|| M::Error::missing_field("is_public_dashboard_ownerless"))?;
                let is_shared = is_shared.ok_or_else(|| M::Error::missing_field("is_shared"))?;
                let last_accessed =
                    last_accessed.ok_or_else(|| M::Error::missing_field("last_accessed"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let popularity = popularity.ok_or_else(|| M::Error::missing_field("popularity"))?;
                let public_title =
                    public_title.ok_or_else(|| M::Error::missing_field("public_title"))?;
                let quality_score =
                    quality_score.ok_or_else(|| M::Error::missing_field("quality_score"))?;
                let share_list = share_list.ok_or_else(|| M::Error::missing_field("share_list"))?;
                let share_type = share_type.ok_or_else(|| M::Error::missing_field("share_type"))?;
                let shared_by = shared_by.ok_or_else(|| M::Error::missing_field("shared_by"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let token = token.ok_or_else(|| M::Error::missing_field("token"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let url = url.ok_or_else(|| M::Error::missing_field("url"))?;
                let widget_count =
                    widget_count.ok_or_else(|| M::Error::missing_field("widget_count"))?;

                let content = DashboardSearchMetadata {
                    author,
                    created_at,
                    deleted_at,
                    embeddable_domains,
                    experience_type,
                    expiration,
                    has_monitors,
                    is_favorited,
                    is_public_dashboard_ownerless,
                    is_shared,
                    last_accessed,
                    modified_at,
                    popularity,
                    public_title,
                    quality_score,
                    share_list,
                    share_type,
                    shared_by,
                    status,
                    token,
                    type_,
                    url,
                    widget_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardSearchMetadataVisitor)
    }
}
