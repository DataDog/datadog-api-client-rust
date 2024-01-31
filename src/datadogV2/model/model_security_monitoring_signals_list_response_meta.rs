// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Meta attributes.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalsListResponseMeta {
    /// Paging attributes.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponseMetaPage>,
}

impl SecurityMonitoringSignalsListResponseMeta {
    pub fn new() -> SecurityMonitoringSignalsListResponseMeta {
        SecurityMonitoringSignalsListResponseMeta { page: None }
    }

    pub fn with_page(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalsListResponseMetaPage,
    ) -> &mut Self {
        self.page = Some(value);
        self
    }
}
impl Default for SecurityMonitoringSignalsListResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}
