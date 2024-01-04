// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing the new list of related signals for a security signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalIncidentsUpdateAttributes {
    /// Array of incidents that are associated with this signal.
    #[serde(rename = "incident_ids")]
    pub incident_ids: Vec<i64>,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl SecurityMonitoringSignalIncidentsUpdateAttributes {
    pub fn new(incident_ids: Vec<i64>) -> SecurityMonitoringSignalIncidentsUpdateAttributes {
        SecurityMonitoringSignalIncidentsUpdateAttributes {
            incident_ids,
            version: None,
        }
    }
}
