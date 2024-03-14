// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the hourly usage attribution by tag(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct HourlyUsageAttributionResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::HourlyUsageAttributionMetadata>,
    /// Get the hourly usage attribution by tag(s).
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::HourlyUsageAttributionBody>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl HourlyUsageAttributionResponse {
    pub fn new() -> HourlyUsageAttributionResponse {
        HourlyUsageAttributionResponse {
            metadata: None,
            usage: None,
            _unparsed: false,
        }
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV1::model::HourlyUsageAttributionMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn usage(
        mut self,
        value: Vec<crate::datadogV1::model::HourlyUsageAttributionBody>,
    ) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for HourlyUsageAttributionResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for HourlyUsageAttributionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HourlyUsageAttributionResponseVisitor;
        impl<'a> Visitor<'a> for HourlyUsageAttributionResponseVisitor {
            type Value = HourlyUsageAttributionResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata: Option<crate::datadogV1::model::HourlyUsageAttributionMetadata> =
                    None;
                let mut usage: Option<Vec<crate::datadogV1::model::HourlyUsageAttributionBody>> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = HourlyUsageAttributionResponse {
                    metadata,
                    usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(HourlyUsageAttributionResponseVisitor)
    }
}
