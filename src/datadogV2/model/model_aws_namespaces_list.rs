// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Metrics namespace filters
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSNamespacesList {
    /// Exclude all namespaces
    #[serde(rename = "exclude_all")]
    pub exclude_all: Option<bool>,
    /// Exclude only these namespaces
    #[serde(rename = "exclude_only")]
    pub exclude_only: Option<Vec<String>>,
    /// Include all namespaces
    #[serde(rename = "include_all")]
    pub include_all: Option<bool>,
    /// Include only these namespaces
    #[serde(rename = "include_only")]
    pub include_only: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSNamespacesList {
    pub fn new() -> AWSNamespacesList {
        AWSNamespacesList {
            exclude_all: None,
            exclude_only: None,
            include_all: None,
            include_only: None,
            _unparsed: false,
        }
    }

    pub fn exclude_all(mut self, value: bool) -> Self {
        self.exclude_all = Some(value);
        self
    }

    pub fn exclude_only(mut self, value: Vec<String>) -> Self {
        self.exclude_only = Some(value);
        self
    }

    pub fn include_all(mut self, value: bool) -> Self {
        self.include_all = Some(value);
        self
    }

    pub fn include_only(mut self, value: Vec<String>) -> Self {
        self.include_only = Some(value);
        self
    }
}

impl Default for AWSNamespacesList {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSNamespacesList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSNamespacesListVisitor;
        impl<'a> Visitor<'a> for AWSNamespacesListVisitor {
            type Value = AWSNamespacesList;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclude_all: Option<bool> = None;
                let mut exclude_only: Option<Vec<String>> = None;
                let mut include_all: Option<bool> = None;
                let mut include_only: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclude_all" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_all =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclude_only" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_all" => {
                            if v.is_null() {
                                continue;
                            }
                            include_all =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_only" => {
                            if v.is_null() {
                                continue;
                            }
                            include_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = AWSNamespacesList {
                    exclude_all,
                    exclude_only,
                    include_all,
                    include_only,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSNamespacesListVisitor)
    }
}
