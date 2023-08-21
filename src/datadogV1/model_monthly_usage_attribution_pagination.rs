// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonthlyUsageAttributionPagination {
    /// The cursor to use to get the next results, if any. To make the next request, use the same parameters with the addition of the `next_record_id`.
    #[serde(rename = "next_record_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none", deserialize_with = "Option::deserialize")]
    pub next_record_id: Option<String>,
}

