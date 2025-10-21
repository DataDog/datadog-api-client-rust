// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Metrics Collection tag filters list. Defaults to `[]`.
/// The array of custom AWS resource tags (in the form `key:value`) defines a filter that Datadog uses
/// when collecting metrics from a specified service.
/// Wildcards, such as `?` (match a single character) and `*` (match multiple characters),
/// and exclusion using `!` before the tag are supported.
/// For EC2, only hosts that match one of the defined tags are imported into Datadog.
/// The rest are ignored. For example, `env:production,instance-type:c?.*,!region:us-east-1`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSNamespaceTagFilter {
    /// The AWS service for which the tag filters defined in `tags` will be applied.
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    /// The AWS resource tags to filter on for the service specified by `namespace`.
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option")]
    pub tags: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSNamespaceTagFilter {
    pub fn new() -> AWSNamespaceTagFilter {
        AWSNamespaceTagFilter {
            namespace: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn namespace(mut self, value: String) -> Self {
        self.namespace = Some(value);
        self
    }

    pub fn tags(mut self, value: Option<Vec<String>>) -> Self {
        self.tags = Some(value);
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
                let mut tags: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSNamespaceTagFilter {
                    namespace,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSNamespaceTagFilterVisitor)
    }
}
