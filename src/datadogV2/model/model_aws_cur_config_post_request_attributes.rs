// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for AWS CUR config Post Request.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigPostRequestAttributes {
    /// The AWS account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The AWS bucket name used to store the Cost and Usage Report.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// The region the bucket is located in.
    #[serde(rename = "bucket_region")]
    pub bucket_region: Option<String>,
    /// Whether or not the Cloud Cost Management account is enabled.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// The month of the report.
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// The report prefix used for the Cost and Usage Report.
    #[serde(rename = "report_prefix")]
    pub report_prefix: String,
}

impl AwsCURConfigPostRequestAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        report_name: String,
        report_prefix: String,
    ) -> AwsCURConfigPostRequestAttributes {
        AwsCURConfigPostRequestAttributes {
            account_id,
            bucket_name,
            bucket_region: None,
            is_enabled: None,
            months: None,
            report_name,
            report_prefix,
        }
    }
}
