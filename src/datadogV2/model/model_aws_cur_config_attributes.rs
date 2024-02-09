// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Attributes for An AWS CUR config.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AwsCURConfigAttributes {
    /// The AWS account ID.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// The AWS bucket name used to store the Cost and Usage Report.
    #[serde(rename = "bucket_name")]
    pub bucket_name: String,
    /// The region the bucket is located in.
    #[serde(rename = "bucket_region")]
    pub bucket_region: String,
    /// The timestamp when the AWS CUR config was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// The error messages for the AWS CUR config.
    #[serde(rename = "error_messages")]
    pub error_messages: Option<Vec<String>>,
    /// The number of months the report has been backfilled.
    #[deprecated]
    #[serde(rename = "months")]
    pub months: Option<i32>,
    /// The name of the Cost and Usage Report.
    #[serde(rename = "report_name")]
    pub report_name: String,
    /// The report prefix used for the Cost and Usage Report.
    #[serde(rename = "report_prefix")]
    pub report_prefix: String,
    /// The status of the AWS CUR.
    #[serde(rename = "status")]
    pub status: String,
    /// The timestamp when the AWS CUR config status was updated.
    #[serde(rename = "status_updated_at")]
    pub status_updated_at: Option<String>,
    /// The timestamp when the AWS CUR config status was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

impl AwsCURConfigAttributes {
    pub fn new(
        account_id: String,
        bucket_name: String,
        bucket_region: String,
        report_name: String,
        report_prefix: String,
        status: String,
    ) -> AwsCURConfigAttributes {
        #[allow(deprecated)]
        AwsCURConfigAttributes {
            account_id,
            bucket_name,
            bucket_region,
            created_at: None,
            error_messages: None,
            months: None,
            report_name,
            report_prefix,
            status,
            status_updated_at: None,
            updated_at: None,
        }
    }

    #[allow(deprecated)]
    pub fn created_at(&mut self, value: String) -> &mut Self {
        self.created_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn error_messages(&mut self, value: Vec<String>) -> &mut Self {
        self.error_messages = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn months(&mut self, value: i32) -> &mut Self {
        self.months = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_updated_at(&mut self, value: String) -> &mut Self {
        self.status_updated_at = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn updated_at(&mut self, value: String) -> &mut Self {
        self.updated_at = Some(value);
        self
    }
}
