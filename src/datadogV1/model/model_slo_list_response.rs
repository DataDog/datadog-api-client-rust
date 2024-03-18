// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A response with one or more service level objective.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOListResponse {
    /// An array of service level objective objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::ServiceLevelObjective>>,
    /// An array of error messages. Each endpoint documents how/whether this field is
    /// used.
    #[serde(rename = "errors")]
    pub errors: Option<Vec<String>>,
    /// The metadata object containing additional information about the list of SLOs.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV1::model::SLOListResponseMetadata>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOListResponse {
    pub fn new() -> SLOListResponse {
        SLOListResponse {
            data: None,
            errors: None,
            metadata: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV1::model::ServiceLevelObjective>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn metadata(mut self, value: crate::datadogV1::model::SLOListResponseMetadata) -> Self {
        self.metadata = Some(value);
        self
    }
}

impl Default for SLOListResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SLOListResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOListResponseVisitor;
        impl<'a> Visitor<'a> for SLOListResponseVisitor {
            type Value = SLOListResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV1::model::ServiceLevelObjective>> = None;
                let mut errors: Option<Vec<String>> = None;
                let mut metadata: Option<crate::datadogV1::model::SLOListResponseMetadata> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            if v.is_null() {
                                continue;
                            }
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SLOListResponse {
                    data,
                    errors,
                    metadata,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOListResponseVisitor)
    }
}
