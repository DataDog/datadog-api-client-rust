// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request for creating a downtime.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DowntimeCreateRequest {
    /// Object to create a downtime.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::DowntimeCreateRequestData,
}

impl DowntimeCreateRequest {
    pub fn new(data: crate::datadogV2::model::DowntimeCreateRequestData) -> DowntimeCreateRequest {
        DowntimeCreateRequest { data }
    }
}
