// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RetentionFilterCreateRequest {
    /// The body of the retention filter to be created.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::RetentionFilterCreateData>,
}

impl RetentionFilterCreateRequest {
    /// The body of the retention filter to be created.
    pub fn new(data: crate::datadogV2::model::RetentionFilterCreateData) -> RetentionFilterCreateRequest {
        RetentionFilterCreateRequest { data: Box::new(data) }
    }
}
