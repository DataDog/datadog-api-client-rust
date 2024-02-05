// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};

/// Object to handle basic authentication when performing the test.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyntheticsBasicAuth {
    SyntheticsBasicAuthWeb(Box<crate::datadogV1::model::SyntheticsBasicAuthWeb>),
    SyntheticsBasicAuthSigv4(Box<crate::datadogV1::model::SyntheticsBasicAuthSigv4>),
    SyntheticsBasicAuthNTLM(Box<crate::datadogV1::model::SyntheticsBasicAuthNTLM>),
    SyntheticsBasicAuthDigest(Box<crate::datadogV1::model::SyntheticsBasicAuthDigest>),
    SyntheticsBasicAuthOauthClient(Box<crate::datadogV1::model::SyntheticsBasicAuthOauthClient>),
    SyntheticsBasicAuthOauthROP(Box<crate::datadogV1::model::SyntheticsBasicAuthOauthROP>),
}
