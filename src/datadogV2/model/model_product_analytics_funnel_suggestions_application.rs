// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Suggested features of one application.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsFunnelSuggestionsApplication {
    /// ID of the RUM application.
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    /// Feature areas of the application.
    #[serde(rename = "domains")]
    pub domains: Option<Vec<crate::datadogV2::model::ProductAnalyticsFunnelSuggestionsDomain>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsFunnelSuggestionsApplication {
    pub fn new() -> ProductAnalyticsFunnelSuggestionsApplication {
        ProductAnalyticsFunnelSuggestionsApplication {
            application_id: None,
            domains: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn domains(
        mut self,
        value: Vec<crate::datadogV2::model::ProductAnalyticsFunnelSuggestionsDomain>,
    ) -> Self {
        self.domains = Some(value);
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

impl Default for ProductAnalyticsFunnelSuggestionsApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsFunnelSuggestionsApplication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsFunnelSuggestionsApplicationVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsFunnelSuggestionsApplicationVisitor {
            type Value = ProductAnalyticsFunnelSuggestionsApplication;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut domains: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsFunnelSuggestionsDomain>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "domains" => {
                            if v.is_null() {
                                continue;
                            }
                            domains = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ProductAnalyticsFunnelSuggestionsApplication {
                    application_id,
                    domains,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsFunnelSuggestionsApplicationVisitor)
    }
}
