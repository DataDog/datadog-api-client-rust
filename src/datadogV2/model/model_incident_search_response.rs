// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response with incidents and facets.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncidentSearchResponse {
    /// Data returned by an incident search.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::IncidentSearchResponseData>,
    /// Included related resources that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentResponseIncludedItem>>,
    /// The metadata object containing pagination metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV2::model::IncidentSearchResponseMeta>>,
}

impl IncidentSearchResponse {
    pub fn new(
        data: Box<crate::datadogV2::model::IncidentSearchResponseData>,
    ) -> IncidentSearchResponse {
        IncidentSearchResponse {
            data,
            included: None,
            meta: None,
        }
    }
}
