// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the Usage Summary by tag(s).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAttributionResponse {
    /// The object containing document metadata.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::UsageAttributionMetadata>,
    /// Get usage summary by tag(s).
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageAttributionBody>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAttributionResponse {
    pub fn new() -> UsageAttributionResponse {
        UsageAttributionResponse {
            metadata: None,
            usage: None,
            _unparsed: false,
        }
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::UsageAttributionMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageAttributionBody>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageAttributionResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAttributionResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAttributionResponseVisitor;
        impl<'a> Visitor<'a> for UsageAttributionResponseVisitor {
            type Value = UsageAttributionResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata: Option<crate::datadogV1::model::UsageAttributionMetadata> = None;
                let mut usage: Option<Vec<crate::datadogV1::model::UsageAttributionBody>> = None;
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

                let content = UsageAttributionResponse {
                    metadata,
                    usage,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAttributionResponseVisitor)
    }
}
