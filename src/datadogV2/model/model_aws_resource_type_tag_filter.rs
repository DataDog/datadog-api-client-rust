// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// AWS Metrics Collection resource type tag filter.
/// Each filter targets either a specific AWS service's resource types (`service` and
/// `resource_types`) or custom metrics (`custom`). These two modes are mutually exclusive.
/// The `expression` defines a tag filter that Datadog uses when collecting metrics from the
/// resource types of a specified service. Wildcards, such as `?` (match a single character)
/// and `*` (match multiple characters), and exclusion using `!` before the tag are supported.
/// For example, `env:production AND instance-type:c?.* AND !region:us-east-1`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSResourceTypeTagFilter {
    /// Whether the tag filter applies to custom metrics. When `true`, `service` and `resource_types` must be omitted.
    #[serde(rename = "custom")]
    pub custom: Option<bool>,
    /// The tag filter expression to apply when collecting metrics for the matching resource types.
    #[serde(rename = "expression")]
    pub expression: String,
    /// The AWS resource types of the service to which the tag filter applies. Required when `service` is set. Use `*` as the only entry to match all resource types of the service; `*` cannot be combined with specific resource types.
    #[serde(rename = "resource_types")]
    pub resource_types: Option<Vec<String>>,
    /// The AWS service for which the tag filter defined in `expression` will be applied. Required unless `custom` is `true`, and must be set together with `resource_types`.
    #[serde(rename = "service")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSResourceTypeTagFilter {
    pub fn new(expression: String) -> AWSResourceTypeTagFilter {
        AWSResourceTypeTagFilter {
            custom: None,
            expression,
            resource_types: None,
            service: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom(mut self, value: bool) -> Self {
        self.custom = Some(value);
        self
    }

    pub fn resource_types(mut self, value: Vec<String>) -> Self {
        self.resource_types = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
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

impl<'de> Deserialize<'de> for AWSResourceTypeTagFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSResourceTypeTagFilterVisitor;
        impl<'a> Visitor<'a> for AWSResourceTypeTagFilterVisitor {
            type Value = AWSResourceTypeTagFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom: Option<bool> = None;
                let mut expression: Option<String> = None;
                let mut resource_types: Option<Vec<String>> = None;
                let mut service: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom" => {
                            if v.is_null() {
                                continue;
                            }
                            custom = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "expression" => {
                            expression = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_types" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_types =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let expression = expression.ok_or_else(|| M::Error::missing_field("expression"))?;

                let content = AWSResourceTypeTagFilter {
                    custom,
                    expression,
                    resource_types,
                    service,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSResourceTypeTagFilterVisitor)
    }
}
