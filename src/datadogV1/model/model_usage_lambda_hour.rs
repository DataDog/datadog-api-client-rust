// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Number of lambda functions and sum of the invocations of all lambda functions
/// for each hour for a given organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsageLambdaHour {
    /// Contains the number of different functions for each region and AWS account.
    #[serde(
        rename = "func_count",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub func_count: Option<Option<i64>>,
    /// The hour for the usage.
    #[serde(rename = "hour")]
    pub hour: Option<String>,
    /// Contains the sum of invocations of all functions.
    #[serde(
        rename = "invocations_sum",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub invocations_sum: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
}

impl UsageLambdaHour {
    pub fn new() -> UsageLambdaHour {
        UsageLambdaHour {
            func_count: None,
            hour: None,
            invocations_sum: None,
            org_name: None,
            public_id: None,
        }
    }
}
