// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
    #[serde(rename = "advisory_id")]
    pub advisory_id: Option<String>,
    #[serde(rename = "symbols")]
    pub symbols: Option<Vec<crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsSymbols>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
    pub fn new() -> ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
        ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
            advisory_id: None,
            symbols: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn advisory_id(mut self, value: String) -> Self {
        self.advisory_id = Some(value);
        self
    }

    pub fn symbols(
        mut self,
        value: Vec<crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsSymbols>,
    ) -> Self {
        self.symbols = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsVisitor;
        impl<'a> Visitor<'a> for ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsVisitor {
            type Value = ResolveVulnerableSymbolsResponseResultsVulnerableSymbols;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut advisory_id: Option<String> = None;
                let mut symbols: Option<Vec<crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsSymbols>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "advisory_id" => {
                            if v.is_null() {
                                continue;
                            }
                            advisory_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "symbols" => {
                            if v.is_null() {
                                continue;
                            }
                            symbols = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ResolveVulnerableSymbolsResponseResultsVulnerableSymbols {
                    advisory_id,
                    symbols,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ResolveVulnerableSymbolsResponseResultsVulnerableSymbolsVisitor)
    }
}
