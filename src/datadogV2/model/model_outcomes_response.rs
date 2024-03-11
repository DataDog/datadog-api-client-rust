// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Scorecard outcomes - the result of a rule for a service.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OutcomesResponse {
    /// List of rule outcomes.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::OutcomesResponseDataItem>>,
    /// Array of rule details.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::OutcomesResponseIncludedItem>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::OutcomesResponseLinks>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OutcomesResponse {
    pub fn new() -> OutcomesResponse {
        OutcomesResponse {
            data: None,
            included: None,
            links: None,
            _unparsed: false,
        }
    }

    pub fn data(
        &mut self,
        value: Vec<crate::datadogV2::model::OutcomesResponseDataItem>,
    ) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn included(
        &mut self,
        value: Vec<crate::datadogV2::model::OutcomesResponseIncludedItem>,
    ) -> &mut Self {
        self.included = Some(value);
        self
    }

    pub fn links(&mut self, value: crate::datadogV2::model::OutcomesResponseLinks) -> &mut Self {
        self.links = Some(value);
        self
    }
}

impl Default for OutcomesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OutcomesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OutcomesResponseVisitor;
        impl<'a> Visitor<'a> for OutcomesResponseVisitor {
            type Value = OutcomesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::OutcomesResponseDataItem>> = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::OutcomesResponseIncludedItem>,
                > = None;
                let mut links: Option<crate::datadogV2::model::OutcomesResponseLinks> = None;
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
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = OutcomesResponse {
                    data,
                    included,
                    links,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OutcomesResponseVisitor)
    }
}
