// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The request for a security signal list.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalListRequest {
    /// Search filters for listing security signals.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter>,
    /// The paging attributes for listing security signals.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SecurityMonitoringSignalListRequestPage>,
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

    pub fn filter(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequestFilter,
    ) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn page(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalListRequestPage,
    ) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort(mut self, value: crate::datadogV2::model::SecurityMonitoringSignalsSort) -> Self {
        self.sort = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignalListRequest {
    fn default() -> Self {
        Self::new()
    }
}
