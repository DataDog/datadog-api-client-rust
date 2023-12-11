// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Returns available specified custom reports.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsResponse {
    /// Response containing date and type for specified custom reports.
    #[serde(rename = "data")]
    pub data: Option<Box<crate::datadogV1::model::UsageSpecifiedCustomReportsData>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<Box<crate::datadogV1::model::UsageSpecifiedCustomReportsMeta>>,
}

impl UsageSpecifiedCustomReportsResponse {
    pub fn new() -> UsageSpecifiedCustomReportsResponse {
        UsageSpecifiedCustomReportsResponse {
            data: None,
            meta: None,
        }
    }
}
