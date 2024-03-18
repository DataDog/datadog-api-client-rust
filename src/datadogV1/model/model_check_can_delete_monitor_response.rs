// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response of monitor IDs that can or can't be safely deleted.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CheckCanDeleteMonitorResponse {
    /// Wrapper object with the list of monitor IDs.
    #[serde(rename = "data")]
    pub data: crate::datadogV1::model::CheckCanDeleteMonitorResponseData,
    /// A mapping of Monitor ID to strings denoting where it's used.
    #[serde(rename = "errors", default, with = "::serde_with::rust::double_option")]
    pub errors: Option<Option<std::collections::BTreeMap<String, Option<Vec<String>>>>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CheckCanDeleteMonitorResponse {
    pub fn new(
        data: crate::datadogV1::model::CheckCanDeleteMonitorResponseData,
    ) -> CheckCanDeleteMonitorResponse {
        CheckCanDeleteMonitorResponse {
            data,
            errors: None,
            _unparsed: false,
        }
    }

    pub fn errors(
        mut self,
        value: Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
    ) -> Self {
        self.errors = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for CheckCanDeleteMonitorResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckCanDeleteMonitorResponseVisitor;
        impl<'a> Visitor<'a> for CheckCanDeleteMonitorResponseVisitor {
            type Value = CheckCanDeleteMonitorResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV1::model::CheckCanDeleteMonitorResponseData> =
                    None;
                let mut errors: Option<
                    Option<std::collections::BTreeMap<String, Option<Vec<String>>>>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "errors" => {
                            errors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = CheckCanDeleteMonitorResponse {
                    data,
                    errors,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CheckCanDeleteMonitorResponseVisitor)
    }
}
