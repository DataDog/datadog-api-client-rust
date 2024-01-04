// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request body for changing the related incidents of a given security monitoring signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateRequest {
    /// Data containing the patch for changing the related incidents of a signal.
    #[serde(rename = "data")]
    pub data: Box<crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateData>,
}

impl SecurityMonitoringSignalIncidentsUpdateRequest {
    pub fn new(
        data: Box<crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateData>,
    ) -> SecurityMonitoringSignalIncidentsUpdateRequest {
        SecurityMonitoringSignalIncidentsUpdateRequest { data }
    }
}