// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a security signal list.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalListRequest {
    /// Search filters for listing security signals.
    #[serde(rename = "filter")]
    pub filter: Option<Box<crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter>>,
    /// The paging attributes for listing security signals.
    #[serde(rename = "page")]
    pub page: Option<Box<crate::datadogV2::model::SecurityMonitoringSignalListRequestPage>>,
    /// The sort parameters used for querying security signals.
    #[serde(rename = "sort")]
    pub sort: Option<crate::datadogV2::model::SecurityMonitoringSignalsSort>,
}

impl SecurityMonitoringSignalListRequest {
    pub fn new() -> SecurityMonitoringSignalListRequest {
        SecurityMonitoringSignalListRequest {
            filter: None,
            page: None,
            sort: None,
        }
    }
}