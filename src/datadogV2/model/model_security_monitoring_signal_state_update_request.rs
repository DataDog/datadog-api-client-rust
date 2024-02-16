// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Request body for changing the state of a given security monitoring signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalStateUpdateRequest {
    /// Data containing the patch for changing the state of a signal.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateData,
}

impl SecurityMonitoringSignalStateUpdateRequest {
    pub fn new(
        data: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateData,
    ) -> SecurityMonitoringSignalStateUpdateRequest {
        SecurityMonitoringSignalStateUpdateRequest { data }
    }
}
