// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Exclude metric names matching one of these patterns for a single namespace.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSMetricNameFiltersExcludeOnly {
    /// Exclude metric names matching one of these patterns.
    #[serde(rename = "exclude_only")]
    pub exclude_only: Vec<String>,
    /// The AWS CloudWatch namespace to which this metric name filter applies.
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSMetricNameFiltersExcludeOnly {
    pub fn new(exclude_only: Vec<String>, namespace: String) -> AWSMetricNameFiltersExcludeOnly {
        AWSMetricNameFiltersExcludeOnly {
            exclude_only,
            namespace,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for AWSMetricNameFiltersExcludeOnly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSMetricNameFiltersExcludeOnlyVisitor;
        impl<'a> Visitor<'a> for AWSMetricNameFiltersExcludeOnlyVisitor {
            type Value = AWSMetricNameFiltersExcludeOnly;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exclude_only: Option<Vec<String>> = None;
                let mut namespace: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exclude_only" => {
                            exclude_only =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "namespace" => {
                            namespace = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let exclude_only =
                    exclude_only.ok_or_else(|| M::Error::missing_field("exclude_only"))?;
                let namespace = namespace.ok_or_else(|| M::Error::missing_field("namespace"))?;

                let content = AWSMetricNameFiltersExcludeOnly {
                    exclude_only,
                    namespace,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSMetricNameFiltersExcludeOnlyVisitor)
    }
}
