// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterResponse {
    /// The definition of the retention filter.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV2::model::RetentionFilter>>,
}

impl RetentionFilterResponse {
    /// The retention filters definition.
    pub fn new() -> RetentionFilterResponse {
        RetentionFilterResponse { data: None }
    }
}
