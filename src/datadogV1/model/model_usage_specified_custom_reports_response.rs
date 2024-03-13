// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Returns available specified custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsResponse {
    /// Response containing date and type for specified custom reports.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsData>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsMeta>,
}

impl UsageSpecifiedCustomReportsResponse {
    pub fn new() -> UsageSpecifiedCustomReportsResponse {
        UsageSpecifiedCustomReportsResponse {
            data: None,
            meta: None,
        }
    }

    pub fn data(mut self, value: crate::datadogV1::model::UsageSpecifiedCustomReportsData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV1::model::UsageSpecifiedCustomReportsMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for UsageSpecifiedCustomReportsResponse {
    fn default() -> Self {
        Self::new()
    }
}
