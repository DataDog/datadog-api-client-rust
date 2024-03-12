// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object description of a security signal.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityMonitoringSignal {
    /// The object containing all signal attributes and their
    /// associated values.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::SecurityMonitoringSignalAttributes>,
    /// The unique ID of the security signal.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SecurityMonitoringSignalType>,
}

impl SecurityMonitoringSignal {
    pub fn new() -> SecurityMonitoringSignal {
        SecurityMonitoringSignal {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringSignalAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::SecurityMonitoringSignalType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SecurityMonitoringSignal {
    fn default() -> Self {
        Self::new()
    }
}
