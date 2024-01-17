// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The object containing document metadata.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageSpecifiedCustomReportsMeta {
    /// The object containing page total count for specified ID.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV1::model::UsageSpecifiedCustomReportsPage>>,
}

impl UsageSpecifiedCustomReportsMeta {
    pub fn new() -> UsageSpecifiedCustomReportsMeta {
        UsageSpecifiedCustomReportsMeta { page: None }
    }
}
impl Default for UsageSpecifiedCustomReportsMeta {
    fn default() -> Self {
        Self::new()
    }
}
