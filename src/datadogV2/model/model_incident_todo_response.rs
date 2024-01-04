// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an incident todo.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTodoResponse {
    /// Incident todo response data.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::IncidentTodoResponseData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentTodoResponseIncludedItem>>,
}

impl IncidentTodoResponse {
    pub fn new(
        data: Box<crate::datadogV2::model::IncidentTodoResponseData>,
    ) -> IncidentTodoResponse {
        IncidentTodoResponse {
            data,
            included: None,
        }
    }
}
