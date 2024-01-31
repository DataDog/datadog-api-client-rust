// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object description of a log after being processed and stored by Datadog.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Log {
    /// JSON object containing all log attributes and their associated values.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::LogAttributes>,
    /// Unique ID of the Log.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::LogType>,
}

impl Log {
    pub fn new() -> Log {
        Log {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn with_attributes(&mut self, value: crate::datadogV2::model::LogAttributes) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn with_id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn with_type_(&mut self, value: crate::datadogV2::model::LogType) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}
