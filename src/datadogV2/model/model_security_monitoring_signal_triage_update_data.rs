// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data containing the updated triage attributes of the signal.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalTriageUpdateData {
    /// Attributes describing a triage state update operation over a security signal.
    #[serde(rename = "attributes")]
    pub attributes: Option<Box<crate::datadogV2::model::SecurityMonitoringSignalTriageAttributes>>,
    /// The unique ID of the security signal.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalMetadataType>,
}

impl SecurityMonitoringSignalTriageUpdateData {
    pub fn new() -> SecurityMonitoringSignalTriageUpdateData {
        SecurityMonitoringSignalTriageUpdateData {
            attributes: None,
            id: None,
            type_: None,
        }
    }
}