// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Usage statistics for a dashboard.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DashboardUsageAttributes {
    /// A user referenced from a dashboard usage record (author or viewer).
    #[serde(rename = "author", default, with = "::serde_with::rust::double_option")]
    pub author: Option<Option<crate::datadogV2::model::DashboardUsageUser>>,
    /// When the dashboard was created.
    #[serde(
        rename = "created_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub created_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The dashboard quality score, or `null` when no score is available.
    #[serde(
        rename = "dashboard_quality_score",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub dashboard_quality_score: Option<Option<f64>>,
    /// When the dashboard was most recently edited.
    #[serde(
        rename = "edited_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub edited_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The Datadog organization that owns the dashboard.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// Teams the dashboard is tagged with.
    #[serde(rename = "teams", default, with = "::serde_with::rust::double_option")]
    pub teams: Option<Option<Vec<String>>>,
    /// The dashboard title.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The total number of times the dashboard has been viewed.
    #[serde(rename = "total_views")]
    pub total_views: Option<i64>,
    /// View counts keyed by view type. Possible keys are `in_app`, `embed`, `public`, `shared`, `api`, and `unknown`.
    #[serde(
        rename = "total_views_by_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub total_views_by_type: Option<Option<std::collections::BTreeMap<String, i64>>>,
    /// When the dashboard was most recently viewed.
    #[serde(
        rename = "viewed_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub viewed_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// A user referenced from a dashboard usage record (author or viewer).
    #[serde(rename = "viewer", default, with = "::serde_with::rust::double_option")]
    pub viewer: Option<Option<crate::datadogV2::model::DashboardUsageUser>>,
    /// The total number of widgets on the dashboard.
    #[serde(
        rename = "widget_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub widget_count: Option<Option<i64>>,
    /// Widget counts keyed by widget type. The map includes group widgets and widgets without requests.
    #[serde(
        rename = "widget_count_by_type",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub widget_count_by_type: Option<Option<std::collections::BTreeMap<String, i64>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DashboardUsageAttributes {
    pub fn new(org_id: i64) -> DashboardUsageAttributes {
        DashboardUsageAttributes {
            author: None,
            created_at: None,
            dashboard_quality_score: None,
            edited_at: None,
            org_id,
            teams: None,
            title: None,
            total_views: None,
            total_views_by_type: None,
            viewed_at: None,
            viewer: None,
            widget_count: None,
            widget_count_by_type: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn author(mut self, value: Option<crate::datadogV2::model::DashboardUsageUser>) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dashboard_quality_score(mut self, value: Option<f64>) -> Self {
        self.dashboard_quality_score = Some(value);
        self
    }

    pub fn edited_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.edited_at = Some(value);
        self
    }

    pub fn teams(mut self, value: Option<Vec<String>>) -> Self {
        self.teams = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn total_views(mut self, value: i64) -> Self {
        self.total_views = Some(value);
        self
    }

    pub fn total_views_by_type(
        mut self,
        value: Option<std::collections::BTreeMap<String, i64>>,
    ) -> Self {
        self.total_views_by_type = Some(value);
        self
    }

    pub fn viewed_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.viewed_at = Some(value);
        self
    }

    pub fn viewer(mut self, value: Option<crate::datadogV2::model::DashboardUsageUser>) -> Self {
        self.viewer = Some(value);
        self
    }

    pub fn widget_count(mut self, value: Option<i64>) -> Self {
        self.widget_count = Some(value);
        self
    }

    pub fn widget_count_by_type(
        mut self,
        value: Option<std::collections::BTreeMap<String, i64>>,
    ) -> Self {
        self.widget_count_by_type = Some(value);
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

impl<'de> Deserialize<'de> for DashboardUsageAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DashboardUsageAttributesVisitor;
        impl<'a> Visitor<'a> for DashboardUsageAttributesVisitor {
            type Value = DashboardUsageAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut author: Option<Option<crate::datadogV2::model::DashboardUsageUser>> = None;
                let mut created_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut dashboard_quality_score: Option<Option<f64>> = None;
                let mut edited_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut org_id: Option<i64> = None;
                let mut teams: Option<Option<Vec<String>>> = None;
                let mut title: Option<String> = None;
                let mut total_views: Option<i64> = None;
                let mut total_views_by_type: Option<
                    Option<std::collections::BTreeMap<String, i64>>,
                > = None;
                let mut viewed_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut viewer: Option<Option<crate::datadogV2::model::DashboardUsageUser>> = None;
                let mut widget_count: Option<Option<i64>> = None;
                let mut widget_count_by_type: Option<
                    Option<std::collections::BTreeMap<String, i64>>,
                > = None;
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
                        "dashboard_quality_score" => {
                            if v.as_str() == Some("") {
                                continue;
                            }
                            dashboard_quality_score =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "edited_at" => {
                            edited_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams" => {
                            teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_views" => {
                            if v.is_null() {
                                continue;
                            }
                            total_views =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_views_by_type" => {
                            total_views_by_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewed_at" => {
                            viewed_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "viewer" => {
                            viewer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widget_count" => {
                            widget_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "widget_count_by_type" => {
                            widget_count_by_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;

                let content = DashboardUsageAttributes {
                    author,
                    created_at,
                    dashboard_quality_score,
                    edited_at,
                    org_id,
                    teams,
                    title,
                    total_views,
                    total_views_by_type,
                    viewed_at,
                    viewer,
                    widget_count,
                    widget_count_by_type,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DashboardUsageAttributesVisitor)
    }
}
