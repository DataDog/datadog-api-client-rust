// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An endpoint's keys mapped to the billing_dimension.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillingDimensionsMappingBodyItemAttributesEndpointsItems {
    /// The URL for the endpoint.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The billing dimension.
    #[serde(rename = "keys")]
    pub keys: Option<Vec<String>>,
    /// Denotes whether mapping keys were available for this endpoint.
    #[serde(rename = "status")]
    pub status: Option<
        crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItemsStatus,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BillingDimensionsMappingBodyItemAttributesEndpointsItems {
    pub fn new() -> BillingDimensionsMappingBodyItemAttributesEndpointsItems {
        BillingDimensionsMappingBodyItemAttributesEndpointsItems {
            id: None,
            keys: None,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn keys(mut self, value: Vec<String>) -> Self {
        self.keys = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItemsStatus,
    ) -> Self {
        self.status = Some(value);
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

impl Default for BillingDimensionsMappingBodyItemAttributesEndpointsItems {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BillingDimensionsMappingBodyItemAttributesEndpointsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BillingDimensionsMappingBodyItemAttributesEndpointsItemsVisitor;
        impl<'a> Visitor<'a> for BillingDimensionsMappingBodyItemAttributesEndpointsItemsVisitor {
            type Value = BillingDimensionsMappingBodyItemAttributesEndpointsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut keys: Option<Vec<String>> = None;
                let mut status: Option<crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItemsStatus> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "keys" => {
                            if v.is_null() {
                                continue;
                            }
                            keys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItemsStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BillingDimensionsMappingBodyItemAttributesEndpointsItems {
                    id,
                    keys,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(BillingDimensionsMappingBodyItemAttributesEndpointsItemsVisitor)
    }
}
