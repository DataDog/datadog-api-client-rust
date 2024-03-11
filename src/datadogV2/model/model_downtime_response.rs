// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Downtiming gives you greater control over monitor notifications by
/// allowing you to globally exclude scopes from alerting.
/// Downtime settings, which can be scheduled with start and end times,
/// prevent all alerting related to specified Datadog tags.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeResponse {
    /// Downtime data.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::DowntimeResponseData>,
    /// Array of objects related to the downtime that the user requested.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::DowntimeResponseIncludedItem>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeResponse {
    pub fn new() -> DowntimeResponse {
        DowntimeResponse {
            data: None,
            included: None,
            _unparsed: false,
        }
    }

    pub fn data(&mut self, value: crate::datadogV2::model::DowntimeResponseData) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::DowntimeResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }
}

impl Default for DowntimeResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeResponseVisitor;
        impl<'a> Visitor<'a> for DowntimeResponseVisitor {
            type Value = DowntimeResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::DowntimeResponseData> = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::DowntimeResponseIncludedItem>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeResponse {
                    data,
                    included,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeResponseVisitor)
    }
}
