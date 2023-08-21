// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AWSAccount {
    /// Your AWS access key ID. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "access_key_id", skip_serializing_if = "Option::is_none")]
    pub access_key_id: String,
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: String,
    /// An object, (in the form `{"namespace1":true/false, "namespace2":true/false}`),
that enables or disables metric collection for specific AWS namespaces for this
AWS account only.
    #[serde(rename = "account_specific_namespace_rules", skip_serializing_if = "Option::is_none")]
    pub account_specific_namespace_rules: map[string]bool,
    /// Whether Datadog collects cloud security posture management resources from your AWS account. This includes additional resources not covered under the general `resource_collection`.
    #[serde(rename = "cspm_resource_collection_enabled", skip_serializing_if = "Option::is_none")]
    pub cspm_resource_collection_enabled: bool,
    /// An array of AWS regions to exclude from metrics collection.
    #[serde(rename = "excluded_regions", skip_serializing_if = "Option::is_none")]
    pub excluded_regions: Vec<String>,
    /// The array of EC2 tags (in the form `key:value`) defines a filter that Datadog uses when collecting metrics from EC2.
Wildcards, such as `?` (for single characters) and `*` (for multiple characters) can also be used.
Only hosts that match one of the defined tags
will be imported into Datadog. The rest will be ignored.
Host matching a given tag can also be excluded by adding `!` before the tag.
For example, `env:production,instance-type:c1.*,!region:us-east-1`
    #[serde(rename = "filter_tags", skip_serializing_if = "Option::is_none")]
    pub filter_tags: Vec<String>,
    /// Array of tags (in the form `key:value`) to add to all hosts
and metrics reporting through this integration.
    #[serde(rename = "host_tags", skip_serializing_if = "Option::is_none")]
    pub host_tags: Vec<String>,
    /// Whether Datadog collects metrics for this AWS account.
    #[serde(rename = "metrics_collection_enabled", skip_serializing_if = "Option::is_none")]
    pub metrics_collection_enabled: bool,
    /// Whether Datadog collects a standard set of resources from your AWS account.
    #[serde(rename = "resource_collection_enabled", skip_serializing_if = "Option::is_none")]
    pub resource_collection_enabled: bool,
    /// Your Datadog role delegation name.
    #[serde(rename = "role_name", skip_serializing_if = "Option::is_none")]
    pub role_name: String,
    /// Your AWS secret access key. Only required if your AWS account is a GovCloud or China account.
    #[serde(rename = "secret_access_key", skip_serializing_if = "Option::is_none")]
    pub secret_access_key: String,
}

