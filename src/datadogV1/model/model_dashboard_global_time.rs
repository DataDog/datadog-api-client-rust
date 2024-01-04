// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object containing the live span selection for the dashboard.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardGlobalTime {
    /// Dashboard global time live_span selection
    #[serde(rename = "live_span")]
    pub live_span: Option<crate::datadogV1::model::DashboardGlobalTimeLiveSpan>,
}

impl DashboardGlobalTime {
    pub fn new() -> DashboardGlobalTime {
        DashboardGlobalTime { live_span: None }
    }
}
