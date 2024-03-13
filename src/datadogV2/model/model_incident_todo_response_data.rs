// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident todo response data.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoResponseData {
    /// Incident todo's attributes.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::IncidentTodoAttributes>,
    /// The incident todo's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The incident's relationships from a response.
    #[serde(rename = "relationships")]
    pub relationships: Option<crate::datadogV2::model::IncidentTodoRelationships>,
    /// Todo resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentTodoType,
}

impl IncidentTodoResponseData {
    pub fn new(
        id: String,
        type_: crate::datadogV2::model::IncidentTodoType,
    ) -> IncidentTodoResponseData {
        IncidentTodoResponseData {
            attributes: None,
            id,
            relationships: None,
            type_,
        }
    }

    pub fn attributes(mut self, value: crate::datadogV2::model::IncidentTodoAttributes) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn relationships(
        mut self,
        value: crate::datadogV2::model::IncidentTodoRelationships,
    ) -> Self {
        self.relationships = Some(value);
        self
    }
}
