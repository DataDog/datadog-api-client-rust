// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// AWS account ID and Lambda ARN.
#[skip_serializing_none]
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AWSAccountAndLambdaRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// ARN of the Datadog Lambda created during the Datadog-Amazon Web services Log collection setup.
    #[serde(rename = "lambda_arn")]
    pub lambda_arn: String,
}

impl AWSAccountAndLambdaRequest {
    pub fn new(account_id: String, lambda_arn: String) -> AWSAccountAndLambdaRequest {
        AWSAccountAndLambdaRequest { account_id, lambda_arn }
    }
}
