// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to handle `NTLM` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthNTLM {
    /// Domain for the authentication to use when performing the test.
    #[serde(rename = "domain")]
    pub domain: Option<String>,
    /// Password for the authentication to use when performing the test.
    #[serde(rename = "password")]
    pub password: Option<String>,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBasicAuthNTLMType,
    /// Username for the authentication to use when performing the test.
    #[serde(rename = "username")]
    pub username: Option<String>,
    /// Workstation for the authentication to use when performing the test.
    #[serde(rename = "workstation")]
    pub workstation: Option<String>,
}

impl SyntheticsBasicAuthNTLM {
    pub fn new(
        type_: crate::datadogV1::model::SyntheticsBasicAuthNTLMType,
    ) -> SyntheticsBasicAuthNTLM {
        SyntheticsBasicAuthNTLM {
            domain: None,
            password: None,
            type_,
            username: None,
            workstation: None,
        }
    }

    pub fn domain(&mut self, value: String) -> &mut Self {
        self.domain = Some(value);
        self
    }

    pub fn password(&mut self, value: String) -> &mut Self {
        self.password = Some(value);
        self
    }

    pub fn username(&mut self, value: String) -> &mut Self {
        self.username = Some(value);
        self
    }

    pub fn workstation(&mut self, value: String) -> &mut Self {
        self.workstation = Some(value);
        self
    }
}
