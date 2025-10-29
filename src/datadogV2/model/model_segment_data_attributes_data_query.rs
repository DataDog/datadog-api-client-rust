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
pub struct SegmentDataAttributesDataQuery {
    #[serde(rename = "combination")]
    pub combination: Option<String>,
    #[serde(rename = "event_platform")]
    pub event_platform:
        Option<Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryEventPlatformItems>>,
    #[serde(rename = "reference_table")]
    pub reference_table:
        Option<Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItems>>,
    #[serde(rename = "static")]
    pub static_: Option<Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryStaticItems>>,
    #[serde(rename = "user_store")]
    pub user_store:
        Option<Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryUserStoreItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentDataAttributesDataQuery {
    pub fn new() -> SegmentDataAttributesDataQuery {
        SegmentDataAttributesDataQuery {
            combination: None,
            event_platform: None,
            reference_table: None,
            static_: None,
            user_store: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn combination(mut self, value: String) -> Self {
        self.combination = Some(value);
        self
    }

    pub fn event_platform(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryEventPlatformItems>,
    ) -> Self {
        self.event_platform = Some(value);
        self
    }

    pub fn reference_table(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItems>,
    ) -> Self {
        self.reference_table = Some(value);
        self
    }

    pub fn static_(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryStaticItems>,
    ) -> Self {
        self.static_ = Some(value);
        self
    }

    pub fn user_store(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryUserStoreItems>,
    ) -> Self {
        self.user_store = Some(value);
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

impl Default for SegmentDataAttributesDataQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SegmentDataAttributesDataQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentDataAttributesDataQueryVisitor;
        impl<'a> Visitor<'a> for SegmentDataAttributesDataQueryVisitor {
            type Value = SegmentDataAttributesDataQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut combination: Option<String> = None;
                let mut event_platform: Option<
                    Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryEventPlatformItems>,
                > = None;
                let mut reference_table: Option<
                    Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryReferenceTableItems>,
                > = None;
                let mut static_: Option<
                    Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryStaticItems>,
                > = None;
                let mut user_store: Option<
                    Vec<crate::datadogV2::model::SegmentDataAttributesDataQueryUserStoreItems>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "combination" => {
                            if v.is_null() {
                                continue;
                            }
                            combination =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_platform" => {
                            if v.is_null() {
                                continue;
                            }
                            event_platform =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_table" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_table =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "static" => {
                            if v.is_null() {
                                continue;
                            }
                            static_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "user_store" => {
                            if v.is_null() {
                                continue;
                            }
                            user_store = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SegmentDataAttributesDataQuery {
                    combination,
                    event_platform,
                    reference_table,
                    static_,
                    user_store,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentDataAttributesDataQueryVisitor)
    }
}
