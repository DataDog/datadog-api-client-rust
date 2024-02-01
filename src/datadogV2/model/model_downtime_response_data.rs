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
    pub attributes: Option<crate::datadogV2::model::DowntimeResponseAttributes>,
    /// The downtime ID.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// All relationships associated with downtime.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::DowntimeRelationships>,
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

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::DowntimeResponseAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn relationships(
        &mut self,
        value: crate::datadogV2::model::DowntimeRelationships,
    ) -> &mut Self {
        self.relationships = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::DowntimeResourceType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for DowntimeResponseData {
    fn default() -> Self {
        Self::new()
    }
}
