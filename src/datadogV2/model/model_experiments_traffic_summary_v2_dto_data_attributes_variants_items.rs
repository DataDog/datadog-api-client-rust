// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Exposure count and identity of one experiment variant.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
    /// Number of recorded exposures for this variant.
    #[serde(rename = "exposure_count")]
    pub exposure_count: Option<i64>,
    /// Key that identifies the experiment variant.
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    /// Display name of the experiment variant.
    #[serde(rename = "variant_name")]
    pub variant_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
    pub fn new() -> ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
        ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
            exposure_count: None,
            variant_key: None,
            variant_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn exposure_count(mut self, value: i64) -> Self {
        self.exposure_count = Some(value);
        self
    }

    pub fn variant_key(mut self, value: String) -> Self {
        self.variant_key = Some(value);
        self
    }

    pub fn variant_name(mut self, value: String) -> Self {
        self.variant_name = Some(value);
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

impl Default for ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsTrafficSummaryV2DTODataAttributesVariantsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsTrafficSummaryV2DTODataAttributesVariantsItemsVisitor {
            type Value = ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut exposure_count: Option<i64> = None;
                let mut variant_key: Option<String> = None;
                let mut variant_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "exposure_count" => {
                            if v.is_null() {
                                continue;
                            }
                            exposure_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_key" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ExperimentsTrafficSummaryV2DTODataAttributesVariantsItems {
                    exposure_count,
                    variant_key,
                    variant_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsTrafficSummaryV2DTODataAttributesVariantsItemsVisitor)
    }
}
