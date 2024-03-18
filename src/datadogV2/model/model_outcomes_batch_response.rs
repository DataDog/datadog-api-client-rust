// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scorecard outcomes batch response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesBatchResponse {
    /// List of rule outcomes which were affected during the bulk operation.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::OutcomesResponseDataItem>,
    /// Metadata pertaining to the bulk operation.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::OutcomesBatchResponseMeta,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesBatchResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::OutcomesResponseDataItem>,
        meta: crate::datadogV2::model::OutcomesBatchResponseMeta,
    ) -> OutcomesBatchResponse {
        OutcomesBatchResponse {
            data,
            meta,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for OutcomesBatchResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesBatchResponseVisitor;
        impl<'a> Visitor<'a> for OutcomesBatchResponseVisitor {
            type Value = OutcomesBatchResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::OutcomesResponseDataItem>> = None;
                let mut meta: Option<crate::datadogV2::model::OutcomesBatchResponseMeta> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;

                let content = OutcomesBatchResponse {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesBatchResponseVisitor)
    }
}
