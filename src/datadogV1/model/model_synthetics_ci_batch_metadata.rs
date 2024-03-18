// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the Synthetic tests run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsCIBatchMetadata {
    /// Description of the CI provider.
    #[serde(rename = "ci")]
    pub ci: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataCI>,
    /// Git information.
    #[serde(rename = "git")]
    pub git: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataGit>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsCIBatchMetadata {
    pub fn new() -> SyntheticsCIBatchMetadata {
        SyntheticsCIBatchMetadata {
            ci: None,
            git: None,
            _unparsed: false,
        }
    }

    pub fn ci(mut self, value: crate::datadogV1::model::SyntheticsCIBatchMetadataCI) -> Self {
        self.ci = Some(value);
        self
    }

    pub fn git(mut self, value: crate::datadogV1::model::SyntheticsCIBatchMetadataGit) -> Self {
        self.git = Some(value);
        self
    }
}

impl Default for SyntheticsCIBatchMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsCIBatchMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsCIBatchMetadataVisitor;
        impl<'a> Visitor<'a> for SyntheticsCIBatchMetadataVisitor {
            type Value = SyntheticsCIBatchMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ci: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataCI> = None;
                let mut git: Option<crate::datadogV1::model::SyntheticsCIBatchMetadataGit> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ci" => {
                            if v.is_null() {
                                continue;
                            }
                            ci = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git" => {
                            if v.is_null() {
                                continue;
                            }
                            git = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsCIBatchMetadata { ci, git, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsCIBatchMetadataVisitor)
    }
}
