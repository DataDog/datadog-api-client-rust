// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response of a monitor group search.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorGroupSearchResponse {
    /// The counts of monitor groups per different criteria.
    #[serde(rename = "counts")]
    pub counts: Option<crate::datadogV1::model::MonitorGroupSearchResponseCounts>,
    /// The list of found monitor groups.
    #[serde(rename = "groups")]
    pub groups: Option<Vec<crate::datadogV1::model::MonitorGroupSearchResult>>,
    /// Metadata about the response.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::MonitorSearchResponseMetadata>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorGroupSearchResponse {
    pub fn new() -> MonitorGroupSearchResponse {
        MonitorGroupSearchResponse {
            counts: None,
            groups: None,
            metadata: None,
            _unparsed: false,
        }
    }

    pub fn counts(
        mut self,
        value: crate::datadogV1::model::MonitorGroupSearchResponseCounts,
    ) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn groups(mut self, value: Vec<crate::datadogV1::model::MonitorGroupSearchResult>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV1::model::MonitorSearchResponseMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }
}

impl Default for MonitorGroupSearchResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorGroupSearchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorGroupSearchResponseVisitor;
        impl<'a> Visitor<'a> for MonitorGroupSearchResponseVisitor {
            type Value = MonitorGroupSearchResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut counts: Option<crate::datadogV1::model::MonitorGroupSearchResponseCounts> =
                    None;
                let mut groups: Option<Vec<crate::datadogV1::model::MonitorGroupSearchResult>> =
                    None;
                let mut metadata: Option<crate::datadogV1::model::MonitorSearchResponseMetadata> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "counts" => {
                            if v.is_null() {
                                continue;
                            }
                            counts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groups" => {
                            if v.is_null() {
                                continue;
                            }
                            groups = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = MonitorGroupSearchResponse {
                    counts,
                    groups,
                    metadata,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorGroupSearchResponseVisitor)
    }
}
