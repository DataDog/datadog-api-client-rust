// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing available custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageCustomReportsResponse {
    /// An array of available custom reports.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV1::model::UsageCustomReportsData>>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::UsageCustomReportsMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageCustomReportsResponse {
    pub fn new() -> UsageCustomReportsResponse {
        UsageCustomReportsResponse {
            data: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<crate::datadogV1::model::UsageCustomReportsData>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV1::model::UsageCustomReportsMeta) -> Self {
        self.meta = Some(value);
        self
    }
}

impl Default for UsageCustomReportsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageCustomReportsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageCustomReportsResponseVisitor;
        impl<'a> Visitor<'a> for UsageCustomReportsResponseVisitor {
            type Value = UsageCustomReportsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV1::model::UsageCustomReportsData>> = None;
                let mut meta: Option<crate::datadogV1::model::UsageCustomReportsMeta> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageCustomReportsResponse {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageCustomReportsResponseVisitor)
    }
}
