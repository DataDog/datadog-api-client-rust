// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Test configuration for Synthetics
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTriggerTest {
    /// Metadata for the Synthetic tests run.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata>,
    /// The public ID of the Synthetic test to trigger.
    #[serde(rename = "public_id")]
    pub public_id: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTriggerTest {
    pub fn new(public_id: String) -> SyntheticsTriggerTest {
        SyntheticsTriggerTest {
            metadata: None,
            public_id,
            _unparsed: false,
        }
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::SyntheticsCIBatchMetadata) -> Self {
        self.metadata = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SyntheticsTriggerTest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTriggerTestVisitor;
        impl<'a> Visitor<'a> for SyntheticsTriggerTestVisitor {
            type Value = SyntheticsTriggerTest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata: Option<crate::datadogV1::model::SyntheticsCIBatchMetadata> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let public_id = public_id.ok_or_else(|| M::Error::missing_field("public_id"))?;

                let content = SyntheticsTriggerTest {
                    metadata,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTriggerTestVisitor)
    }
}
