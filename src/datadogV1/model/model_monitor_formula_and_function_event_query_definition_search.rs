// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Search options.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionEventQueryDefinitionSearch {
    /// Events search string.
    #[serde(rename = "query")]
    pub query: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionEventQueryDefinitionSearch {
    pub fn new(query: String) -> MonitorFormulaAndFunctionEventQueryDefinitionSearch {
        MonitorFormulaAndFunctionEventQueryDefinitionSearch {
            query,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionEventQueryDefinitionSearch {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionEventQueryDefinitionSearchVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionEventQueryDefinitionSearchVisitor {
            type Value = MonitorFormulaAndFunctionEventQueryDefinitionSearch;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut query: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;

                let content =
                    MonitorFormulaAndFunctionEventQueryDefinitionSearch { query, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionEventQueryDefinitionSearchVisitor)
    }
}
