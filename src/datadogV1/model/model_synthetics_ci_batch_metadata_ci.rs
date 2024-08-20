// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Description of the CI provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsCIBatchMetadataCI {
    /// Description of the CI pipeline.
    #[serde(rename = "pipeline")]
    pub pipeline: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataPipeline>,
    /// Description of the CI provider.
    #[serde(rename = "provider")]
    pub provider: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataProvider>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsCIBatchMetadataCI {
    pub fn new() -> SyntheticsCIBatchMetadataCI {
        SyntheticsCIBatchMetadataCI {
            pipeline: None,
            provider: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn pipeline(
        mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadataPipeline,
    ) -> Self {
        self.pipeline = Some(value);
        self
    }

    pub fn provider(
        mut self,
        value: crate::datadogV1::model::SyntheticsCIBatchMetadataProvider,
    ) -> Self {
        self.provider = Some(value);
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

impl Default for SyntheticsCIBatchMetadataCI {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsCIBatchMetadataCI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCIBatchMetadataCIVisitor;
        impl<'a> Visitor<'a> for SyntheticsCIBatchMetadataCIVisitor {
            type Value = SyntheticsCIBatchMetadataCI;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut pipeline: Option<
                    crate::datadogV1::model::SyntheticsCIBatchMetadataPipeline,
                > = None;
                let mut provider: Option<
                    crate::datadogV1::model::SyntheticsCIBatchMetadataProvider,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "pipeline" => {
                            if v.is_null() {
                                continue;
                            }
                            pipeline = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "provider" => {
                            if v.is_null() {
                                continue;
                            }
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsCIBatchMetadataCI {
                    pipeline,
                    provider,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCIBatchMetadataCIVisitor)
    }
}
