// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident data for a create request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentCreateData {
    /// The incident's attributes for a create request.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::IncidentCreateAttributes>,
    /// The relationships the incident will have with other resources once created.
    #[serde(rename = "relationships")]
    pub relationships: Option<Box<crate::datadogV2::model::IncidentCreateRelationships>>,
    /// Incident resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentType,
}

impl IncidentCreateData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::IncidentCreateAttributes>,
        type_: crate::datadogV2::model::IncidentType,
    ) -> IncidentCreateData {
        IncidentCreateData {
            attributes,
            relationships: None,
            type_,
        }
    }
}
