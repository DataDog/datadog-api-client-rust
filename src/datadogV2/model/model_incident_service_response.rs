// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with an incident service payload.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentServiceResponse {
    /// Incident Service data from responses.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::IncidentServiceResponseData>,
    /// Included objects from relationships.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentServiceIncludedItems>>,
}

impl IncidentServiceResponse {
    pub fn new(
        data: Box<crate::datadogV2::model::IncidentServiceResponseData>,
    ) -> IncidentServiceResponse {
        IncidentServiceResponse {
            data,
            included: None,
        }
    }
}
