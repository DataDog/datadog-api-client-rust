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
pub struct ScaRequestDataAttributesVulnerabilitiesItems {
    #[serde(rename = "affects")]
    pub affects: Option<
        Vec<crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItemsAffectsItems>,
    >,
    #[serde(rename = "bom_ref")]
    pub bom_ref: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ScaRequestDataAttributesVulnerabilitiesItems {
    pub fn new() -> ScaRequestDataAttributesVulnerabilitiesItems {
        ScaRequestDataAttributesVulnerabilitiesItems {
            affects: None,
            bom_ref: None,
            id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn affects(
        mut self,
        value: Vec<
            crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItemsAffectsItems,
        >,
    ) -> Self {
        self.affects = Some(value);
        self
    }

    pub fn bom_ref(mut self, value: String) -> Self {
        self.bom_ref = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
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

impl Default for ScaRequestDataAttributesVulnerabilitiesItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ScaRequestDataAttributesVulnerabilitiesItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ScaRequestDataAttributesVulnerabilitiesItemsVisitor;
        impl<'a> Visitor<'a> for ScaRequestDataAttributesVulnerabilitiesItemsVisitor {
            type Value = ScaRequestDataAttributesVulnerabilitiesItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut affects: Option<Vec<crate::datadogV2::model::ScaRequestDataAttributesVulnerabilitiesItemsAffectsItems>> = None;
                let mut bom_ref: Option<String> = None;
                let mut id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "affects" => {
                            if v.is_null() {
                                continue;
                            }
                            affects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bom_ref" => {
                            if v.is_null() {
                                continue;
                            }
                            bom_ref = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ScaRequestDataAttributesVulnerabilitiesItems {
                    affects,
                    bom_ref,
                    id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ScaRequestDataAttributesVulnerabilitiesItemsVisitor)
    }
}
