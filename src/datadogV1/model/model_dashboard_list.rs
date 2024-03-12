// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Your Datadog Dashboards.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardList {
    /// Object describing the creator of the shared element.
    #[serde(rename = "author")]
    pub author: Option<crate::datadogV1::model::Creator>,
    /// Date of creation of the dashboard list.
    #[serde(rename = "created")]
    pub created: Option<String>,
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
    pub modified: Option<String>,
    /// The name of the dashboard list.
    #[serde(rename = "name")]
    pub name: String,
    /// The type of dashboard list.
    #[serde(rename = "type")]
    pub type_: Option<String>,
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
        }
    }

    pub fn author(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.author = Some(value);
        self
    }

    pub fn created(mut self, value: String) -> Self {
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

    pub fn modified(mut self, value: String) -> Self {
        self.modified = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
        self
    }
}
