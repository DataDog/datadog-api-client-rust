// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The response object with all security signals matching the request
/// and pagination information.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalsListResponse {
    /// An array of security signals matching the request.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::SecurityMonitoringSignal>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponseLinks>,
    /// Meta attributes.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::SecurityMonitoringSignalsListResponseMeta>,
}

impl SecurityMonitoringSignalsListResponse {
    pub fn new() -> SecurityMonitoringSignalsListResponse {
        SecurityMonitoringSignalsListResponse {
            data: None,
            links: None,
            meta: None,
        }
    }

    pub fn with_data(
        &mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringSignal>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn with_links(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalsListResponseLinks,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn with_meta(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalsListResponseMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}
impl Default for SecurityMonitoringSignalsListResponse {
    fn default() -> Self {
        Self::new()
    }
}
