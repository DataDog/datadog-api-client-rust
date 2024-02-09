// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Incident todo data for a patch request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoPatchData {
    /// Incident todo's attributes.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::IncidentTodoAttributes,
    /// Todo resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::IncidentTodoType,
}

impl IncidentTodoPatchData {
    pub fn new(
        attributes: crate::datadogV2::model::IncidentTodoAttributes,
        type_: crate::datadogV2::model::IncidentTodoType,
    ) -> IncidentTodoPatchData {
        IncidentTodoPatchData { attributes, type_ }
    }
}
