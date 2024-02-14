// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The postmortem relationship data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentPostmortemData {
    /// A unique identifier that represents the postmortem.
    #[serde(rename = "id")]
    pub id: String,
    /// Incident postmortem resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentPostmortemType,
}

impl RelationshipToIncidentPostmortemData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentPostmortemType,
    ) -> RelationshipToIncidentPostmortemData {
        RelationshipToIncidentPostmortemData { id, type_ }
    }
}
