// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an incident team payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentTeamResponse {
    /// Incident Team data from a response.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::IncidentTeamResponseData,
    /// Included objects from relationships.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentTeamIncludedItems>>,
}

impl IncidentTeamResponse {
    pub fn new(data: crate::datadogV2::model::IncidentTeamResponseData) -> IncidentTeamResponse {
        IncidentTeamResponse {
            data,
            included: None,
        }
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::IncidentTeamIncludedItems>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}
