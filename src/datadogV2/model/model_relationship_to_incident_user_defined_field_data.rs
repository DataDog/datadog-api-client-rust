// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Relationship to impact object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationshipToIncidentUserDefinedFieldData {
    /// A unique identifier that represents the responder.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident user defined fields type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentUserDefinedFieldType,
}

impl RelationshipToIncidentUserDefinedFieldData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentUserDefinedFieldType,
    ) -> RelationshipToIncidentUserDefinedFieldData {
        RelationshipToIncidentUserDefinedFieldData { id, type_ }
    }
}
