// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Log source configuration.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AWSLambdaForwarderConfigLogSourceConfig {
    /// List of AWS log source tag filters. Defaults to `[]`.
    #[serde(rename = "tag_filters")]
    pub tag_filters: Option<Vec<crate::datadogV2::model::AWSLogSourceTagFilter>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AWSLambdaForwarderConfigLogSourceConfig {
    pub fn new() -> AWSLambdaForwarderConfigLogSourceConfig {
        AWSLambdaForwarderConfigLogSourceConfig {
            tag_filters: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn tag_filters(
        mut self,
        value: Vec<crate::datadogV2::model::AWSLogSourceTagFilter>,
    ) -> Self {
        self.tag_filters = Some(value);
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

impl Default for AWSLambdaForwarderConfigLogSourceConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for AWSLambdaForwarderConfigLogSourceConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AWSLambdaForwarderConfigLogSourceConfigVisitor;
        impl<'a> Visitor<'a> for AWSLambdaForwarderConfigLogSourceConfigVisitor {
            type Value = AWSLambdaForwarderConfigLogSourceConfig;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut tag_filters: Option<Vec<crate::datadogV2::model::AWSLogSourceTagFilter>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "tag_filters" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_filters =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = AWSLambdaForwarderConfigLogSourceConfig {
                    tag_filters,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AWSLambdaForwarderConfigLogSourceConfigVisitor)
    }
}
