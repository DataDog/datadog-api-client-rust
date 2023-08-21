// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCIVisibilityHour {
    /// The number of spans for pipelines in the queried hour.
    #[serde(rename = "ci_pipeline_indexed_spans", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ci_pipeline_indexed_spans: Option<Int64>,
    /// The number of spans for tests in the queried hour.
    #[serde(rename = "ci_test_indexed_spans", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ci_test_indexed_spans: Option<Int64>,
    /// Shows the total count of all active Git committers for Intelligent Test Runner in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(rename = "ci_visibility_itr_committers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ci_visibility_itr_committers: Option<Int64>,
    /// Shows the total count of all active Git committers for Pipelines in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(rename = "ci_visibility_pipeline_committers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ci_visibility_pipeline_committers: Option<Int64>,
    /// The total count of all active Git committers for tests in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(rename = "ci_visibility_test_committers", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub ci_visibility_test_committers: Option<Int64>,
    /// The organization name.
    #[serde(rename = "org_name", skip_serializing_if = "Option::is_none")]
    pub org_name: String,
    /// The organization public ID.
    #[serde(rename = "public_id", skip_serializing_if = "Option::is_none")]
    pub public_id: String,
}

