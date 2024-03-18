// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Confluent account returned by the API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConfluentAccountsResponse {
    /// The Confluent account.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ConfluentAccountResponseData>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConfluentAccountsResponse {
    pub fn new() -> ConfluentAccountsResponse {
        ConfluentAccountsResponse {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Vec<crate::datadogV2::model::ConfluentAccountResponseData>,
    ) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for ConfluentAccountsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ConfluentAccountsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConfluentAccountsResponseVisitor;
        impl<'a> Visitor<'a> for ConfluentAccountsResponseVisitor {
            type Value = ConfluentAccountsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::ConfluentAccountResponseData>> =
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
                        &_ => {}
                    }
                }

                let content = ConfluentAccountsResponse { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConfluentAccountsResponseVisitor)
    }
}
