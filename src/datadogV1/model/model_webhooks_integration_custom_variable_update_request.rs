// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Update request of a custom variable object.
///
/// *All properties are optional.*
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIntegrationCustomVariableUpdateRequest {
    /// Make custom variable is secret or not.
    /// If the custom variable is secret, the value is not returned in the response payload.
    #[serde(rename = "is_secret")]
    pub is_secret: Option<bool>,
    /// The name of the variable. It corresponds with `<CUSTOM_VARIABLE_NAME>`. It must only contains upper-case characters, integers or underscores.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Value of the custom variable.
    #[serde(rename = "value")]
    pub value: Option<String>,
}

impl WebhooksIntegrationCustomVariableUpdateRequest {
    pub fn new() -> WebhooksIntegrationCustomVariableUpdateRequest {
        WebhooksIntegrationCustomVariableUpdateRequest {
            is_secret: None,
            name: None,
            value: None,
        }
    }

    pub fn with_is_secret(&mut self, value: bool) -> &mut Self {
        self.is_secret = Some(value);
        self
    }

    pub fn with_name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }

    pub fn with_value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
}
impl Default for WebhooksIntegrationCustomVariableUpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}
