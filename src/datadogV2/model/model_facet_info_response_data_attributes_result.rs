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
pub struct FacetInfoResponseDataAttributesResult {
    #[serde(rename = "range")]
    pub range: Option<crate::datadogV2::model::FacetInfoResponseDataAttributesResultRange>,
    #[serde(rename = "values")]
    pub values:
        Option<Vec<crate::datadogV2::model::FacetInfoResponseDataAttributesResultValuesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FacetInfoResponseDataAttributesResult {
    pub fn new() -> FacetInfoResponseDataAttributesResult {
        FacetInfoResponseDataAttributesResult {
            range: None,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn range(
        mut self,
        value: crate::datadogV2::model::FacetInfoResponseDataAttributesResultRange,
    ) -> Self {
        self.range = Some(value);
        self
    }

    pub fn values(
        mut self,
        value: Vec<crate::datadogV2::model::FacetInfoResponseDataAttributesResultValuesItems>,
    ) -> Self {
        self.values = Some(value);
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

impl Default for FacetInfoResponseDataAttributesResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FacetInfoResponseDataAttributesResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FacetInfoResponseDataAttributesResultVisitor;
        impl<'a> Visitor<'a> for FacetInfoResponseDataAttributesResultVisitor {
            type Value = FacetInfoResponseDataAttributesResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut range: Option<
                    crate::datadogV2::model::FacetInfoResponseDataAttributesResultRange,
                > = None;
                let mut values: Option<
                    Vec<crate::datadogV2::model::FacetInfoResponseDataAttributesResultValuesItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "range" => {
                            if v.is_null() {
                                continue;
                            }
                            range = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "values" => {
                            if v.is_null() {
                                continue;
                            }
                            values = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FacetInfoResponseDataAttributesResult {
                    range,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FacetInfoResponseDataAttributesResultVisitor)
    }
}
