// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A tag filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSTagFilter {
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

impl AWSTagFilter {
    pub fn new() -> AWSTagFilter {
        AWSTagFilter {
            namespace: None,
            tag_filter_str: None,
            _unparsed: false,
        }
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

impl Default for AWSTagFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSTagFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSTagFilterVisitor;
        impl<'a> Visitor<'a> for AWSTagFilterVisitor {
            type Value = AWSTagFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut namespace: Option<crate::datadogV1::model::AWSNamespace> = None;
                let mut tag_filter_str: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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

                let content = AWSTagFilter {
                    namespace,
                    tag_filter_str,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSTagFilterVisitor)
    }
}
