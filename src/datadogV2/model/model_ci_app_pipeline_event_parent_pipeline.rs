// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// If the pipeline is triggered as child of another pipeline, this should contain the details of the parent pipeline.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CIAppPipelineEventParentPipeline {
    /// UUID of a pipeline.
    #[serde(rename = "id")]
    pub id: String,
    /// The URL to look at the pipeline in the CI provider UI.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CIAppPipelineEventParentPipeline {
    pub fn new(id: String) -> CIAppPipelineEventParentPipeline {
        CIAppPipelineEventParentPipeline {
            id,
            url: None,
            _unparsed: false,
        }
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CIAppPipelineEventParentPipeline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CIAppPipelineEventParentPipelineVisitor;
        impl<'a> Visitor<'a> for CIAppPipelineEventParentPipelineVisitor {
            type Value = CIAppPipelineEventParentPipeline;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut url: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = CIAppPipelineEventParentPipeline { id, url, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CIAppPipelineEventParentPipelineVisitor)
    }
}
