// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of Synthetic locations.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsLocations {
    /// List of Synthetic locations.
    #[serde(rename = "locations")]
    pub locations: Option<Vec<crate::datadogV1::model::SyntheticsLocation>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsLocations {
    pub fn new() -> SyntheticsLocations {
        SyntheticsLocations {
            locations: None,
            _unparsed: false,
        }
    }

    pub fn locations(mut self, value: Vec<crate::datadogV1::model::SyntheticsLocation>) -> Self {
        self.locations = Some(value);
        self
    }
}

impl Default for SyntheticsLocations {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsLocations {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsLocationsVisitor;
        impl<'a> Visitor<'a> for SyntheticsLocationsVisitor {
            type Value = SyntheticsLocations;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut locations: Option<Vec<crate::datadogV1::model::SyntheticsLocation>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "locations" => {
                            if v.is_null() {
                                continue;
                            }
                            locations = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = SyntheticsLocations {
                    locations,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsLocationsVisitor)
    }
}
