// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A service level objective history response.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SLOHistoryResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SLOHistoryResponseData>>,
    /// A list of errors while querying the history data for the service level objective.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option")]
    pub errors: Option<Option<Vec<Option<crate::datadogV1::model::SLOHistoryResponseError>>>>,
}

impl SLOHistoryResponse {
    pub fn new() -> SLOHistoryResponse {
        SLOHistoryResponse {
            data: None,
            errors: None,
        }
    }
}
