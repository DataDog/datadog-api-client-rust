// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object to handle `SIGV4` authentication when performing the test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn region(mut self, value: String) -> Self {
        self.region = Some(value);
        self
    }

    pub fn service_name(mut self, value: String) -> Self {
        self.service_name = Some(value);
        self
    }

    pub fn session_token(mut self, value: String) -> Self {
        self.session_token = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsBasicAuthSigv4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsBasicAuthSigv4Visitor;
        impl<'a> Visitor<'a> for SyntheticsBasicAuthSigv4Visitor {
            type Value = SyntheticsBasicAuthSigv4;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut access_key: Option<String> = None;
                let mut region: Option<String> = None;
                let mut secret_key: Option<String> = None;
                let mut service_name: Option<String> = None;
                let mut session_token: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::SyntheticsBasicAuthSigv4Type> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accessKey" => {
                            access_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "region" => {
                            if v.is_null() {
                                continue;
                            }
                            region = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "secretKey" => {
                            secret_key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "serviceName" => {
                            if v.is_null() {
                                continue;
                            }
                            service_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sessionToken" => {
                            if v.is_null() {
                                continue;
                            }
                            session_token =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::SyntheticsBasicAuthSigv4Type::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let access_key = access_key.ok_or_else(|| M::Error::missing_field("access_key"))?;
                let secret_key = secret_key.ok_or_else(|| M::Error::missing_field("secret_key"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SyntheticsBasicAuthSigv4 {
                    access_key,
                    region,
                    secret_key,
                    service_name,
                    session_token,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsBasicAuthSigv4Visitor)
    }
}
