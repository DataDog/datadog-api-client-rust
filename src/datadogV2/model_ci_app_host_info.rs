// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CIAppHostInfo {
    /// FQDN of the host.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: String,
    /// A list of labels used to select or identify the node.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Vec<String>,
    /// Name for the host.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: String,
    /// The path where the code is checked out.
    #[serde(rename = "workspace", skip_serializing_if = "Option::is_none")]
    pub workspace: String,
}

