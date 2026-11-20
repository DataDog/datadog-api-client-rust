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
pub struct SplitConfigSort {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1_20270101::model::SplitSortCompute>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "order")]
    pub order: crate::datadogV1_20270101::model::SplitConfigSortOrder,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl SplitConfigSort {
    pub fn new(order: crate::datadogV1_20270101::model::SplitConfigSortOrder) -> SplitConfigSort {
        SplitConfigSort {
            compute: None,
            order,
            _unparsed: false,
        }
    }
    
    pub fn compute(mut self, value: crate::datadogV1_20270101::model::SplitSortCompute) -> Self {
        self.compute = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for SplitConfigSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitConfigSortVisitor;
        impl<'a> Visitor<'a> for SplitConfigSortVisitor {
            type Value = SplitConfigSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1_20270101::model::SplitSortCompute> = None;
                let mut order: Option<crate::datadogV1_20270101::model::SplitConfigSortOrder> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1_20270101::model::SplitConfigSortOrder::UnparsedObject(_order) => {
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
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;

                let content = SplitConfigSort {
                    compute,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitConfigSortVisitor)
    }
}
