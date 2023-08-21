// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsPrivateLocationMetadata {
    /// A list of role identifiers that can be pulled from the Roles API, for restricting read and write access.
    #[serde(rename = "restricted_roles", skip_serializing_if = "Option::is_none")]
    pub restricted_roles: Vec<String>,
}

