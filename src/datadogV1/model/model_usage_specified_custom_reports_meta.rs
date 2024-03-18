// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object containing document metadata.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSpecifiedCustomReportsMeta {
    /// The object containing page total count for specified ID.
    #[serde(rename = "page")]
    pub page: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsPage>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSpecifiedCustomReportsMeta {
    pub fn new() -> UsageSpecifiedCustomReportsMeta {
        UsageSpecifiedCustomReportsMeta {
            page: None,
            _unparsed: false,
        }
    }

    pub fn page(mut self, value: crate::datadogV1::model::UsageSpecifiedCustomReportsPage) -> Self {
        self.page = Some(value);
        self
    }
}

impl Default for UsageSpecifiedCustomReportsMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSpecifiedCustomReportsMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSpecifiedCustomReportsMetaVisitor;
        impl<'a> Visitor<'a> for UsageSpecifiedCustomReportsMetaVisitor {
            type Value = UsageSpecifiedCustomReportsMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut page: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsPage> =
                    None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "page" => {
                            if v.is_null() {
                                continue;
                            }
                            page = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageSpecifiedCustomReportsMeta { page, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSpecifiedCustomReportsMetaVisitor)
    }
}
