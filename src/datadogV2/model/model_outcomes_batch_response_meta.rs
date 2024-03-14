// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata pertaining to the bulk operation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesBatchResponseMeta {
    /// Total number of scorecard results received during the bulk operation.
    #[serde(rename = "total_received")]
    pub total_received: Option<i64>,
    /// Total number of scorecard results modified during the bulk operation.
    #[serde(rename = "total_updated")]
    pub total_updated: Option<i64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesBatchResponseMeta {
    pub fn new() -> OutcomesBatchResponseMeta {
        OutcomesBatchResponseMeta {
            total_received: None,
            total_updated: None,
            _unparsed: false,
        }
    }

    pub fn total_received(mut self, value: i64) -> Self {
        self.total_received = Some(value);
        self
    }

    pub fn total_updated(mut self, value: i64) -> Self {
        self.total_updated = Some(value);
        self
    }
}

impl Default for OutcomesBatchResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesBatchResponseMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesBatchResponseMetaVisitor;
        impl<'a> Visitor<'a> for OutcomesBatchResponseMetaVisitor {
            type Value = OutcomesBatchResponseMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut total_received: Option<i64> = None;
                let mut total_updated: Option<i64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "total_received" => {
                            if v.is_null() {
                                continue;
                            }
                            total_received =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_updated" => {
                            if v.is_null() {
                                continue;
                            }
                            total_updated =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OutcomesBatchResponseMeta {
                    total_received,
                    total_updated,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesBatchResponseMetaVisitor)
    }
}
