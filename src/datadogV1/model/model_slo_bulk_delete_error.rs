// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the error.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SLOBulkDeleteError {
    /// The ID of the service level objective object associated with
    /// this error.
    #[serde(rename = "id")]
    pub id: String,
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
    /// The timeframe of the threshold associated with this error
    /// or "all" if all thresholds are affected.
    #[serde(rename = "timeframe")]
    pub timeframe: crate::datadogV1::model::SLOErrorTimeframe,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SLOBulkDeleteError {
    pub fn new(
        id: String,
        message: String,
        timeframe: crate::datadogV1::model::SLOErrorTimeframe,
    ) -> SLOBulkDeleteError {
        SLOBulkDeleteError {
            id,
            message,
            timeframe,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for SLOBulkDeleteError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SLOBulkDeleteErrorVisitor;
        impl<'a> Visitor<'a> for SLOBulkDeleteErrorVisitor {
            type Value = SLOBulkDeleteError;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut message: Option<String> = None;
                let mut timeframe: Option<crate::datadogV1::model::SLOErrorTimeframe> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _timeframe) = timeframe {
                                match _timeframe {
                                    crate::datadogV1::model::SLOErrorTimeframe::UnparsedObject(
                                        _timeframe,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let timeframe = timeframe.ok_or_else(|| M::Error::missing_field("timeframe"))?;

                let content = SLOBulkDeleteError {
                    id,
                    message,
                    timeframe,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SLOBulkDeleteErrorVisitor)
    }
}
