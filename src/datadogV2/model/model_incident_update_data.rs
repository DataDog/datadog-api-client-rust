// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident data for an update request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentUpdateData {
    /// The incident's attributes for an update request.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::IncidentUpdateAttributes>>,
    /// The incident's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident's relationships for an update request.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::IncidentUpdateRelationships>>,
    /// Incident resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentType,
}

impl IncidentUpdateData {
    pub fn new(id: String, type_: crate::datadogV2::model::IncidentType) -> IncidentUpdateData {
        IncidentUpdateData {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }
}
