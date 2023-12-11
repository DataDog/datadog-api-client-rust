// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes describing which incident to add the signal to.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AddSignalToIncidentRequest {
    /// Whether to post the signal on the incident timeline.
    #[serde(rename = "add_to_signal_timeline")]
    pub add_to_signal_timeline: Option<bool>,
    /// Public ID attribute of the incident to which the signal will be added.
    #[serde(rename = "incident_id")]
    pub incident_id: i64,
    /// Version of the updated signal. If server side version is higher, update will be rejected.
    #[serde(rename = "version")]
    pub version: Option<i64>,
}

impl AddSignalToIncidentRequest {
    pub fn new(incident_id: i64) -> AddSignalToIncidentRequest {
        AddSignalToIncidentRequest {
            add_to_signal_timeline: None,
            incident_id,
            version: None,
        }
    }
}
