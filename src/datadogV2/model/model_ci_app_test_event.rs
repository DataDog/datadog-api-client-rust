// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object description of test event after being processed and stored by Datadog.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppTestEvent {
    /// JSON object containing all event attributes and their associated values.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::CIAppEventAttributes>,
    /// Unique ID of the event.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Type of the event.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::CIAppTestEventTypeName>,
}

impl CIAppTestEvent {
    pub fn new() -> CIAppTestEvent {
        CIAppTestEvent {
            attributes: None,
            id: None,
            type_: None,
        }
    }

    pub fn attributes(
        &mut self,
        value: crate::datadogV2::model::CIAppEventAttributes,
    ) -> &mut Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(&mut self, value: String) -> &mut Self {
        self.id = Some(value);
        self
    }

    pub fn type_(&mut self, value: crate::datadogV2::model::CIAppTestEventTypeName) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for CIAppTestEvent {
    fn default() -> Self {
        Self::new()
    }
}
