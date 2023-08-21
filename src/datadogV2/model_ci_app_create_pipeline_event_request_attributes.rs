// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppCreatePipelineEventRequestAttributes {
    /// The Datadog environment.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: String,
    /// Details of the CI pipeline event.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: CIAppCreatePipelineEventRequestAttributesResource,
    /// If the CI provider is SaaS, use this to differentiate between instances.
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: String,
}

