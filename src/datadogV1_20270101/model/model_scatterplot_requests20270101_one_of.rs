// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ScatterplotRequests20270101OneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "x")]
    pub x: crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "y")]
    pub y: crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScatterplotRequests20270101OneOf {
    pub fn new(x: crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest, y: crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest) -> ScatterplotRequests20270101OneOf {
        ScatterplotRequests20270101OneOf {
            x,
            y,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for ScatterplotRequests20270101OneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotRequests20270101OneOfVisitor;
        impl<'a> Visitor<'a> for ScatterplotRequests20270101OneOfVisitor {
            type Value = ScatterplotRequests20270101OneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut x: Option<crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest> = None;
                let mut y: Option<crate::datadogV1_20270101::model::ScatterplotBackendAxisRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "x" => {
                            x = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "y" => {
                            y = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let x = x.ok_or_else(|| M::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| M::Error::missing_field("y"))?;

                let content = ScatterplotRequests20270101OneOf {
                    x,
                    y,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotRequests20270101OneOfVisitor)
    }
}
