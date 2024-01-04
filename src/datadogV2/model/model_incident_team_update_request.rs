// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update request with an incident team payload.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamUpdateRequest {
    /// Incident Team data for an update request.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::IncidentTeamUpdateData>,
}

impl IncidentTeamUpdateRequest {
    pub fn new(
        data: Box<crate::datadogV2::model::IncidentTeamUpdateData>,
    ) -> IncidentTeamUpdateRequest {
        IncidentTeamUpdateRequest { data }
    }
}