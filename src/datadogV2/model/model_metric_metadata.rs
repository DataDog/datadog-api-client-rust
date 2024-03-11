// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the metric.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricMetadata {
    /// Metric origin information.
    #[serde(rename = "origin")]
    pub origin: Option<crate::datadogV2::model::MetricOrigin>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricMetadata {
    pub fn new() -> MetricMetadata {
        MetricMetadata {
            origin: None,
            _unparsed: false,
        }
    }

    pub fn origin(&mut self, value: crate::datadogV2::model::MetricOrigin) -> &mut Self {
        self.origin = Some(value);
        self
    }
}

impl Default for MetricMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MetricMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricMetadataVisitor;
        impl<'a> Visitor<'a> for MetricMetadataVisitor {
            type Value = MetricMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut origin: Option<crate::datadogV2::model::MetricOrigin> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "origin" => {
                            if v.is_null() {
                                continue;
                            }
                            origin = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MetricMetadata { origin, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricMetadataVisitor)
    }
}
