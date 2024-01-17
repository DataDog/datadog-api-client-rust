// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Scorecard outcomes - the result of a rule for a service.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutcomesResponse {
    /// List of rule outcomes.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::OutcomesResponseDataItem>>,
    /// Array of rule details.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::OutcomesResponseIncludedItem>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<Box<crate::datadogV2::model::OutcomesResponseLinks>>,
}

impl OutcomesResponse {
    pub fn new() -> OutcomesResponse {
        OutcomesResponse {
            data: None,
            included: None,
            links: None,
        }
    }
}
impl Default for OutcomesResponse {
    fn default() -> Self {
        Self::new()
    }
}
