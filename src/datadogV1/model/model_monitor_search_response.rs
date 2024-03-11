// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response form a monitor search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorSearchResponse {
    /// The counts of monitors per different criteria.
    #[serde(rename = "counts")]
    pub counts: Option<crate::datadogV1::model::MonitorSearchResponseCounts>,
    /// Metadata about the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::MonitorSearchResponseMetadata>,
    /// The list of found monitors.
    #[serde(rename = "monitors")]
    pub monitors: Option<Vec<crate::datadogV1::model::MonitorSearchResult>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorSearchResponse {
    pub fn new() -> MonitorSearchResponse {
        MonitorSearchResponse {
            counts: None,
            metadata: None,
            monitors: None,
            _unparsed: false,
        }
    }

    pub fn counts(
        &mut self,
        value: crate::datadogV1::model::MonitorSearchResponseCounts,
    ) -> &mut Self {
        self.counts = Some(value);
        self
    }

    pub fn metadata(
        &mut self,
        value: crate::datadogV1::model::MonitorSearchResponseMetadata,
    ) -> &mut Self {
        self.metadata = Some(value);
        self
    }

    pub fn monitors(
        &mut self,
        value: Vec<crate::datadogV1::model::MonitorSearchResult>,
    ) -> &mut Self {
        self.monitors = Some(value);
        self
    }
}

impl Default for MonitorSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorSearchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSearchResponseVisitor;
        impl<'a> Visitor<'a> for MonitorSearchResponseVisitor {
            type Value = MonitorSearchResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut counts: Option<crate::datadogV1::model::MonitorSearchResponseCounts> = None;
                let mut metadata: Option<crate::datadogV1::model::MonitorSearchResponseMetadata> =
                    None;
                let mut monitors: Option<Vec<crate::datadogV1::model::MonitorSearchResult>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "counts" => {
                            if v.is_null() {
                                continue;
                            }
                            counts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "monitors" => {
                            if v.is_null() {
                                continue;
                            }
                            monitors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorSearchResponse {
                    counts,
                    metadata,
                    monitors,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSearchResponseVisitor)
    }
}
