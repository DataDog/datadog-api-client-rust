// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Data containing the patch for changing the state of a signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignalStateUpdateData {
    /// Attributes describing the change of state of a security signal.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes,
    /// The unique ID of the security signal.
    #[serde(rename = "id")]
    pub id: Option<serde_json::Value>,
    /// The type of event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalMetadataType>,
}

impl SecurityMonitoringSignalStateUpdateData {
    pub fn new(
        attributes: crate::datadogV2::model::SecurityMonitoringSignalStateUpdateAttributes,
    ) -> SecurityMonitoringSignalStateUpdateData {
        SecurityMonitoringSignalStateUpdateData {
            attributes,
            id: None,
            type_: None,
        }
    }

    pub fn id(&mut self, value: serde_json::Value) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalMetadataType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
