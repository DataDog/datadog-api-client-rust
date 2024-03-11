// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Returns available specified custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSpecifiedCustomReportsResponse {
    /// Response containing date and type for specified custom reports.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsData>,
    /// The object containing document metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSpecifiedCustomReportsResponse {
    pub fn new() -> UsageSpecifiedCustomReportsResponse {
        UsageSpecifiedCustomReportsResponse {
            data: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(
        &mut self,
        value: crate::datadogV1::model::UsageSpecifiedCustomReportsData,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn meta(
        &mut self,
        value: crate::datadogV1::model::UsageSpecifiedCustomReportsMeta,
    ) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for UsageSpecifiedCustomReportsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSpecifiedCustomReportsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSpecifiedCustomReportsResponseVisitor;
        impl<'a> Visitor<'a> for UsageSpecifiedCustomReportsResponseVisitor {
            type Value = UsageSpecifiedCustomReportsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsData> =
                    None;
                let mut meta: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsMeta> =
                    None;
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

                let content = UsageSpecifiedCustomReportsResponse {
                    data,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSpecifiedCustomReportsResponseVisitor)
    }
}
