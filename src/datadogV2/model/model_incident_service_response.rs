// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with an incident service payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentServiceResponse {
    /// Incident Service data from responses.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::IncidentServiceResponseData,
    /// Included objects from relationships.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentServiceIncludedItems>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentServiceResponse {
    pub fn new(
        data: crate::datadogV2::model::IncidentServiceResponseData,
    ) -> IncidentServiceResponse {
        IncidentServiceResponse {
            data,
            included: None,
            _unparsed: false,
        }
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentServiceIncludedItems>,
    ) -> Self {
        self.included = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for IncidentServiceResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentServiceResponseVisitor;
        impl<'a> Visitor<'a> for IncidentServiceResponseVisitor {
            type Value = IncidentServiceResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::IncidentServiceResponseData> = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::IncidentServiceIncludedItems>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
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
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = IncidentServiceResponse {
                    data,
                    included,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentServiceResponseVisitor)
    }
}
