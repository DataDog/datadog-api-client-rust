// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Configuration for Continuous Testing.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyntheticsCITest {
    /// Disable certificate checks in API tests.
    #[serde(rename = "allowInsecureCertificates")]
    pub allow_insecure_certificates: Option<bool>,
    /// Object to handle basic authentication when performing the test.
    #[serde(rename = "basicAuth")]
    pub basic_auth: Option<crate::datadogV1::model::SyntheticsBasicAuth>,
    /// Body to include in the test.
    #[serde(rename = "body")]
    pub body: Option<String>,
    /// Type of the data sent in a Synthetic API test.
    #[serde(rename = "bodyType")]
    pub body_type: Option<String>,
    /// Cookies for the request.
    #[serde(rename = "cookies")]
    pub cookies: Option<String>,
    /// For browser test, array with the different device IDs used to run the test.
    #[serde(rename = "deviceIds")]
    pub device_ids: Option<Vec<crate::datadogV1::model::SyntheticsDeviceID>>,
    /// For API HTTP test, whether or not the test should follow redirects.
    #[serde(rename = "followRedirects")]
    pub follow_redirects: Option<bool>,
    /// Headers to include when performing the test.
    #[serde(rename = "headers")]
    pub headers: Option<std::collections::BTreeMap<String, String>>,
    /// Array of locations used to run the test.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<String>>,
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata>,
    /// The public ID of the Synthetic test to trigger.
    #[serde(rename = "public_id")]
    pub public_id: String,
    /// Object describing the retry strategy to apply to a Synthetic test.
    #[serde(rename = "retry")]
    pub retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry>,
    /// Starting URL for the browser test.
    #[serde(rename = "startUrl")]
    pub start_url: Option<String>,
    /// Variables to replace in the test.
    #[serde(rename = "variables")]
    pub variables: Option<std::collections::BTreeMap<String, String>>,
}

impl SyntheticsCITest {
    pub fn new(public_id: String) -> SyntheticsCITest {
        SyntheticsCITest {
            allow_insecure_certificates: None,
            basic_auth: None,
            body: None,
            body_type: None,
            cookies: None,
            device_ids: None,
            follow_redirects: None,
            headers: None,
            locations: None,
            metadata: None,
            public_id,
            retry: None,
            start_url: None,
            variables: None,
        }
    }

    pub fn allow_insecure_certificates(&mut self, value: bool) -> &mut Self {
        self.allow_insecure_certificates = Some(value);
        self
    }

    pub fn basic_auth(&mut self, value: crate::datadogV1::model::SyntheticsBasicAuth) -> &mut Self {
        self.basic_auth = Some(value);
        self
    }

    pub fn body(&mut self, value: String) -> &mut Self {
        self.body = Some(value);
        self
    }

    pub fn body_type(&mut self, value: String) -> &mut Self {
        self.body_type = Some(value);
        self
    }

    pub fn cookies(&mut self, value: String) -> &mut Self {
        self.cookies = Some(value);
        self
    }

    pub fn device_ids(
        &mut self,
        value: Vec<crate::datadogV1::model::SyntheticsDeviceID>,
    ) -> &mut Self {
        self.device_ids = Some(value);
        self
    }

    pub fn follow_redirects(&mut self, value: bool) -> &mut Self {
        self.follow_redirects = Some(value);
        self
    }

    pub fn headers(&mut self, value: std::collections::BTreeMap<String, String>) -> &mut Self {
        self.headers = Some(value);
        self
    }

    pub fn locations(&mut self, value: Vec<String>) -> &mut Self {
        self.locations = Some(value);
        self
    }

    pub fn metadata(
        &mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn retry(
        &mut self,
        value: crate::datadogV1::model::SyntheticsTestOptionsRetry,
    ) -> &mut Self {
        self.retry = Some(value);
        self
    }

    pub fn start_url(&mut self, value: String) -> &mut Self {
        self.start_url = Some(value);
        self
    }

    pub fn variables(&mut self, value: std::collections::BTreeMap<String, String>) -> &mut Self {
        self.variables = Some(value);
        self
    }
}
