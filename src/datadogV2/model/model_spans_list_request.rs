// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a spans list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpansListRequest {
    /// The object containing the query content.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::SpansListRequestData>>,
}

impl SpansListRequest {
    pub fn new() -> SpansListRequest {
        SpansListRequest { data: None }
    }
}
