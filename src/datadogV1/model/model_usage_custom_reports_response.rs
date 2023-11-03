// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Response containing available custom reports.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageCustomReportsResponse {
    /// An array of available custom reports.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::UsageCustomReportsData>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV1::model::UsageCustomReportsMeta>>,
}

impl UsageCustomReportsResponse {
    pub fn new() -> UsageCustomReportsResponse {
        UsageCustomReportsResponse {
            data: None,
            meta: None,
        }
    }
}
