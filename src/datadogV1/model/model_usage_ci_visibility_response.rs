// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI visibility usage response
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageCIVisibilityResponse {
    /// Response containing CI visibility usage.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageCIVisibilityHour>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageCIVisibilityResponse {
    pub fn new() -> UsageCIVisibilityResponse {
        UsageCIVisibilityResponse {
            usage: None,
            _unparsed: false,
        }
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageCIVisibilityHour>) -> Self {
        self.usage = Some(value);
        self
    }
}

impl Default for UsageCIVisibilityResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageCIVisibilityResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageCIVisibilityResponseVisitor;
        impl<'a> Visitor<'a> for UsageCIVisibilityResponseVisitor {
            type Value = UsageCIVisibilityResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut usage: Option<Vec<crate::datadogV1::model::UsageCIVisibilityHour>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageCIVisibilityResponse { usage, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageCIVisibilityResponseVisitor)
    }
}
