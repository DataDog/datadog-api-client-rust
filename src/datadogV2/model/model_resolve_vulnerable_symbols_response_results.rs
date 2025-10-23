// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The definition of `ResolveVulnerableSymbolsResponseResults` object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ResolveVulnerableSymbolsResponseResults {
    /// The `items` `purl`.
    #[serde(rename = "purl")]
    pub purl: Option<String>,
    /// The `items` `vulnerable_symbols`.
    #[serde(rename = "vulnerable_symbols")]
    pub vulnerable_symbols: Option<
        Vec<crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbols>,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ResolveVulnerableSymbolsResponseResults {
    pub fn new() -> ResolveVulnerableSymbolsResponseResults {
        ResolveVulnerableSymbolsResponseResults {
            purl: None,
            vulnerable_symbols: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn purl(mut self, value: String) -> Self {
        self.purl = Some(value);
        self
    }

    pub fn vulnerable_symbols(
        mut self,
        value: Vec<
            crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbols,
        >,
    ) -> Self {
        self.vulnerable_symbols = Some(value);
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

impl Default for ResolveVulnerableSymbolsResponseResults {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ResolveVulnerableSymbolsResponseResults {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ResolveVulnerableSymbolsResponseResultsVisitor;
        impl<'a> Visitor<'a> for ResolveVulnerableSymbolsResponseResultsVisitor {
            type Value = ResolveVulnerableSymbolsResponseResults;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut purl: Option<String> = None;
                let mut vulnerable_symbols: Option<Vec<crate::datadogV2::model::ResolveVulnerableSymbolsResponseResultsVulnerableSymbols>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "purl" => {
                            if v.is_null() {
                                continue;
                            }
                            purl = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "vulnerable_symbols" => {
                            if v.is_null() {
                                continue;
                            }
                            vulnerable_symbols =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ResolveVulnerableSymbolsResponseResults {
                    purl,
                    vulnerable_symbols,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ResolveVulnerableSymbolsResponseResultsVisitor)
    }
}
