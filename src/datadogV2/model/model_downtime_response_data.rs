// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Downtime data.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeResponseData {
    /// Downtime details.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::DowntimeResponseAttributes>>,
    /// The downtime ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// All relationships associated with downtime.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::DowntimeRelationships>>,
    /// Downtime resource type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::DowntimeResourceType>,
}

impl DowntimeResponseData {
    pub fn new() -> DowntimeResponseData {
        DowntimeResponseData {
            attributes: None,
            id: None,
            relationships: None,
            type_: None,
        }
    }
}
impl Default for DowntimeResponseData {
    fn default() -> Self {
        Self::new()
    }
}
