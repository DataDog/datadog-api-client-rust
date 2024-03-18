// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The objects used to set an AWS tag filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSTagFilterCreateRequest {
    /// Your AWS Account ID without dashes.
    #[serde(rename = "account_id")]
    pub account_id: Option<String>,
    /// The namespace associated with the tag filter entry.
    #[serde(rename = "namespace")]
    pub namespace: Option<crate::datadogV1::model::AWSNamespace>,
    /// The tag filter string.
    #[serde(rename = "tag_filter_str")]
    pub tag_filter_str: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSTagFilterCreateRequest {
    pub fn new() -> AWSTagFilterCreateRequest {
        AWSTagFilterCreateRequest {
            account_id: None,
            namespace: None,
            tag_filter_str: None,
            _unparsed: false,
        }
    }

    pub fn account_id(mut self, value: String) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn namespace(mut self, value: crate::datadogV1::model::AWSNamespace) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn tag_filter_str(mut self, value: String) -> Self {
        self.tag_filter_str = Some(value);
        self
    }
}

impl Default for AWSTagFilterCreateRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSTagFilterCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSTagFilterCreateRequestVisitor;
        impl<'a> Visitor<'a> for AWSTagFilterCreateRequestVisitor {
            type Value = AWSTagFilterCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_id: Option<String> = None;
                let mut namespace: Option<crate::datadogV1::model::AWSNamespace> = None;
                let mut tag_filter_str: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_id" => {
                            if v.is_null() {
                                continue;
                            }
                            account_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _namespace) = namespace {
                                match _namespace {
                                    crate::datadogV1::model::AWSNamespace::UnparsedObject(
                                        _namespace,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tag_filter_str" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filter_str =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSTagFilterCreateRequest {
                    account_id,
                    namespace,
                    tag_filter_str,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSTagFilterCreateRequestVisitor)
    }
}
