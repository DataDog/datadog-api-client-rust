// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsServicesRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: String,
    /// Array of services IDs set to enable automatic log collection. Discover the list of available services with the get list of AWS log ready services API endpoint.
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Vec<String>,
}

