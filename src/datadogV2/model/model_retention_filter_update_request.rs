// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The body of the retention filter to be updated.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterUpdateRequest {
    /// The body of the retention filter to be updated.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::RetentionFilterUpdateData>,
}

impl RetentionFilterUpdateRequest {
    pub fn new(
        data: Box<crate::datadogV2::model::RetentionFilterUpdateData>,
    ) -> RetentionFilterUpdateRequest {
        RetentionFilterUpdateRequest { data }
    }
}
