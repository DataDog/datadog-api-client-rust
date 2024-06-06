// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Metrics tag filters
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSNamespaceTagFilter {
    /// The AWS Namespace to apply the tag filters against
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    /// The tags to filter based on
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSNamespaceTagFilter {
    pub fn new() -> AWSNamespaceTagFilter {
        AWSNamespaceTagFilter {
            namespace: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for AWSNamespaceTagFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSNamespaceTagFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSNamespaceTagFilterVisitor;
        impl<'a> Visitor<'a> for AWSNamespaceTagFilterVisitor {
            type Value = AWSNamespaceTagFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut namespace: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "namespace" => {
                            if v.is_null() {
                                continue;
                            }
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSNamespaceTagFilter {
                    namespace,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSNamespaceTagFilterVisitor)
    }
}
