// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing the Cloud Workload Security usage for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageCWSResponse {
    /// Get hourly usage for Cloud Workload Security.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageCWSHour>>,
}

impl UsageCWSResponse {
    pub fn new() -> UsageCWSResponse {
        UsageCWSResponse { usage: None }
    }
}
