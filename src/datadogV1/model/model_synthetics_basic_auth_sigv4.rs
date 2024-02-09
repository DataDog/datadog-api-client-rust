// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object to handle `SIGV4` authentication when performing the test.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsBasicAuthSigv4 {
    /// Access key for the `SIGV4` authentication.
    #[serde(rename = "accessKey")]
    pub access_key: String,
    /// Region for the `SIGV4` authentication.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// Secret key for the `SIGV4` authentication.
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    /// Service name for the `SIGV4` authentication.
    #[serde(rename = "serviceName")]
    pub service_name: Option<String>,
    /// Session token for the `SIGV4` authentication.
    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,
    /// The type of authentication to use when performing the test.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::SyntheticsBasicAuthSigv4Type,
}

impl SyntheticsBasicAuthSigv4 {
    pub fn new(
        access_key: String,
        secret_key: String,
        type_: crate::datadogV1::model::SyntheticsBasicAuthSigv4Type,
    ) -> SyntheticsBasicAuthSigv4 {
        SyntheticsBasicAuthSigv4 {
            access_key,
            region: None,
            secret_key,
            service_name: None,
            session_token: None,
            type_,
        }
    }

    pub fn region(&mut self, value: String) -> &mut Self {
        self.region = Some(value);
        self
    }

    pub fn service_name(&mut self, value: String) -> &mut Self {
        self.service_name = Some(value);
        self
    }

    pub fn session_token(&mut self, value: String) -> &mut Self {
        self.session_token = Some(value);
        self
    }
}
