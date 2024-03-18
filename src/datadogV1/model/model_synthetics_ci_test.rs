// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Configuration for Continuous Testing.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            _unparsed: false,
        }
    }

    pub fn allow_insecure_certificates(mut self, value: bool) -> Self {
        self.allow_insecure_certificates = Some(value);
        self
    }

    pub fn basic_auth(mut self, value: crate::datadogV1::model::SyntheticsBasicAuth) -> Self {
        self.basic_auth = Some(value);
        self
    }

    pub fn body(mut self, value: String) -> Self {
        self.body = Some(value);
        self
    }

    pub fn body_type(mut self, value: String) -> Self {
        self.body_type = Some(value);
        self
    }

    pub fn cookies(mut self, value: String) -> Self {
        self.cookies = Some(value);
        self
    }

    pub fn device_ids(mut self, value: Vec<crate::datadogV1::model::SyntheticsDeviceID>) -> Self {
        self.device_ids = Some(value);
        self
    }

    pub fn follow_redirects(mut self, value: bool) -> Self {
        self.follow_redirects = Some(value);
        self
    }

    pub fn headers(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn locations(mut self, value: Vec<String>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::SyntheticsCIBatchMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn retry(mut self, value: crate::datadogV1::model::SyntheticsTestOptionsRetry) -> Self {
        self.retry = Some(value);
        self
    }

    pub fn start_url(mut self, value: String) -> Self {
        self.start_url = Some(value);
        self
    }

    pub fn variables(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.variables = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsCITest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCITestVisitor;
        impl<'a> Visitor<'a> for SyntheticsCITestVisitor {
            type Value = SyntheticsCITest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_insecure_certificates: Option<bool> = None;
                let mut basic_auth: Option<crate::datadogV1::model::SyntheticsBasicAuth> = None;
                let mut body: Option<String> = None;
                let mut body_type: Option<String> = None;
                let mut cookies: Option<String> = None;
                let mut device_ids: Option<Vec<crate::datadogV1::model::SyntheticsDeviceID>> = None;
                let mut follow_redirects: Option<bool> = None;
                let mut headers: Option<std::collections::BTreeMap<String, String>> = None;
                let mut locations: Option<Vec<String>> = None;
                let mut metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata> = None;
                let mut public_id: Option<String> = None;
                let mut retry: Option<crate::datadogV1::model::SyntheticsTestOptionsRetry> = None;
                let mut start_url: Option<String> = None;
                let mut variables: Option<std::collections::BTreeMap<String, String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowInsecureCertificates" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_insecure_certificates =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "basicAuth" => {
                            if v.is_null() {
                                continue;
                            }
                            basic_auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _basic_auth) = basic_auth {
                                match _basic_auth {
                                    crate::datadogV1::model::SyntheticsBasicAuth::UnparsedObject(_basic_auth) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "body" => {
                            if v.is_null() {
                                continue;
                            }
                            body = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bodyType" => {
                            if v.is_null() {
                                continue;
                            }
                            body_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cookies" => {
                            if v.is_null() {
                                continue;
                            }
                            cookies = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "deviceIds" => {
                            if v.is_null() {
                                continue;
                            }
                            device_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "followRedirects" => {
                            if v.is_null() {
                                continue;
                            }
                            follow_redirects =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "headers" => {
                            if v.is_null() {
                                continue;
                            }
                            headers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "locations" => {
                            if v.is_null() {
                                continue;
                            }
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retry" => {
                            if v.is_null() {
                                continue;
                            }
                            retry = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "startUrl" => {
                            if v.is_null() {
                                continue;
                            }
                            start_url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variables" => {
                            if v.is_null() {
                                continue;
                            }
                            variables = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let public_id = public_id.ok_or_else(|| M::Error::missing_field("public_id"))?;

                let content = SyntheticsCITest {
                    allow_insecure_certificates,
                    basic_auth,
                    body,
                    body_type,
                    cookies,
                    device_ids,
                    follow_redirects,
                    headers,
                    locations,
                    metadata,
                    public_id,
                    retry,
                    start_url,
                    variables,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCITestVisitor)
    }
}
