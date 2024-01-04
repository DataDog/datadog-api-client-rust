// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with a list of incidents.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentsResponse {
    /// An array of incidents.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentResponseData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentResponseIncludedItem>>,
    /// The metadata object containing pagination metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::IncidentResponseMeta>>,
}

impl IncidentsResponse {
    pub fn new(data: Vec<crate::datadogV2::model::IncidentResponseData>) -> IncidentsResponse {
        IncidentsResponse {
            data,
            included: None,
            meta: None,
        }
    }
}
