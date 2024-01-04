// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data containing the patch for changing the related incidents of a signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateData {
    /// Attributes describing the new list of related signals for a security signal.
    #[serde(rename = "attributes")]
    pub attributes: Box<crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateAttributes>,
}

impl SecurityMonitoringSignalIncidentsUpdateData {
    pub fn new(
        attributes: Box<crate::datadogV2::model::SecurityMonitoringSignalIncidentsUpdateAttributes>,
    ) -> SecurityMonitoringSignalIncidentsUpdateData {
        SecurityMonitoringSignalIncidentsUpdateData { attributes }
    }
}