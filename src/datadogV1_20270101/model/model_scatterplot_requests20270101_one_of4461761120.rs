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
pub struct ScatterplotRequests20270101OneOf4461761120 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "table")]
    pub table: crate::datadogV1_20270101::model::ScatterplotBackendTableRequest,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ScatterplotRequests20270101OneOf4461761120 {
    pub fn new(table: crate::datadogV1_20270101::model::ScatterplotBackendTableRequest) -> ScatterplotRequests20270101OneOf4461761120 {
        ScatterplotRequests20270101OneOf4461761120 {
            table,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for ScatterplotRequests20270101OneOf4461761120 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScatterplotRequests20270101OneOf4461761120Visitor;
        impl<'a> Visitor<'a> for ScatterplotRequests20270101OneOf4461761120Visitor {
            type Value = ScatterplotRequests20270101OneOf4461761120;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut table: Option<crate::datadogV1_20270101::model::ScatterplotBackendTableRequest> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "table" => {
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _table) = table {
                                match _table {
                                    crate::datadogV1_20270101::model::ScatterplotBackendTableRequest::UnparsedObject(_table) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let table = table.ok_or_else(|| M::Error::missing_field("table"))?;

                let content = ScatterplotRequests20270101OneOf4461761120 {
                    table,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScatterplotRequests20270101OneOf4461761120Visitor)
    }
}
