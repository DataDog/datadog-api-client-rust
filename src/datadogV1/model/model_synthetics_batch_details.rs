// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Details about a batch response.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBatchDetails {
    /// Wrapper object that contains the details of a batch.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::SyntheticsBatchDetailsData>>,
}

impl SyntheticsBatchDetails {
    pub fn new() -> SyntheticsBatchDetails {
        SyntheticsBatchDetails { data: None }
    }
}
