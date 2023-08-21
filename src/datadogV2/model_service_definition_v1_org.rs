// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinitionV1Org {
    /// App feature this service supports.
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: String,
    /// Team that owns the service.
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: String,
}

