// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Description of the CI pipeline.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsCIBatchMetadataPipeline {
    /// URL of the pipeline.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsCIBatchMetadataPipeline {
    pub fn new() -> SyntheticsCIBatchMetadataPipeline {
        SyntheticsCIBatchMetadataPipeline {
            url: None,
            _unparsed: false,
        }
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadataPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsCIBatchMetadataPipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCIBatchMetadataPipelineVisitor;
        impl<'a> Visitor<'a> for SyntheticsCIBatchMetadataPipelineVisitor {
            type Value = SyntheticsCIBatchMetadataPipeline;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsCIBatchMetadataPipeline { url, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCIBatchMetadataPipelineVisitor)
    }
}
