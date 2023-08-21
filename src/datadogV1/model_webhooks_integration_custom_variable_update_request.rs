// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhooksIntegrationCustomVariableUpdateRequest {
    /// Make custom variable is secret or not.
If the custom variable is secret, the value is not returned in the response payload.
    #[serde(rename = "is_secret", skip_serializing_if = "Option::is_none")]
    pub is_secret: bool,
    /// The name of the variable. It corresponds with `<CUSTOM_VARIABLE_NAME>`. It must only contains upper-case characters, integers or underscores.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// Value of the custom variable.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: String,
}

