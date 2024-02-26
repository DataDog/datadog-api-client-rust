// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to handle digest authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthDigest {
    /// Password to use for the digest authentication.
    #[serde(rename = "password")]
    pub password: String,
    /// The type of basic authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::SyntheticsBasicAuthDigestType>,
    /// Username to use for the digest authentication.
    #[serde(rename = "username")]
    pub username: String,
}

impl SyntheticsBasicAuthDigest {
    pub fn new(password: String, username: String) -> SyntheticsBasicAuthDigest {
        SyntheticsBasicAuthDigest {
            password,
            type_: None,
            username,
        }
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV1::model::SyntheticsBasicAuthDigestType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}
