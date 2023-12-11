// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A list of all Datadog-AWS logs integrations available in your Datadog organization.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AWSLogsListResponse {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// List of ARNs configured in your Datadog account.
    #[serde(rename = "lambdas")]
    pub lambdas: Option<Vec<crate::datadogV1::model::AWSLogsLambda>>,
    /// Array of services IDs.
    #[serde(rename = "services")]
    pub services: Option<Vec<String>>,
}

impl AWSLogsListResponse {
    pub fn new() -> AWSLogsListResponse {
        AWSLogsListResponse {
            account_id: None,
            lambdas: None,
            services: None,
        }
    }
}
