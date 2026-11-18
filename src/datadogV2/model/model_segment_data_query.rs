// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Defines who belongs to the segment. It contains one or more blocks, up to 20 in total.
/// Use `combination` to combine named blocks with a boolean formula such as `A AND NOT B`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentDataQuery {
    /// Account profile blocks.
    #[serde(rename = "account_store")]
    pub account_store: Option<Vec<crate::datadogV2::model::SegmentProfileStoreBlock>>,
    /// Boolean formula that combines blocks by name.
    #[serde(rename = "combination")]
    pub combination: Option<String>,
    /// Event blocks.
    #[serde(rename = "event_platform")]
    pub event_platform: Option<Vec<crate::datadogV2::model::SegmentEventPlatformBlock>>,
    /// Funnel blocks.
    #[serde(rename = "journeys")]
    pub journeys: Option<Vec<crate::datadogV2::model::SegmentJourneyBlock>>,
    /// Reference table blocks.
    #[serde(rename = "reference_table")]
    pub reference_table: Option<Vec<crate::datadogV2::model::SegmentReferenceTableBlock>>,
    /// Retention blocks.
    #[serde(rename = "retentions")]
    pub retentions: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Uploaded list blocks.
    #[serde(rename = "static")]
    pub static_: Option<Vec<crate::datadogV2::model::SegmentStaticBlock>>,
    /// Template blocks.
    #[serde(rename = "templates")]
    pub templates: Option<Vec<crate::datadogV2::model::SegmentTemplateBlock>>,
    /// User profile blocks.
    #[serde(rename = "user_store")]
    pub user_store: Option<Vec<crate::datadogV2::model::SegmentProfileStoreBlock>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentDataQuery {
    pub fn new() -> SegmentDataQuery {
        SegmentDataQuery {
            account_store: None,
            combination: None,
            event_platform: None,
            journeys: None,
            reference_table: None,
            retentions: None,
            static_: None,
            templates: None,
            user_store: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account_store(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentProfileStoreBlock>,
    ) -> Self {
        self.account_store = Some(value);
        self
    }

    pub fn combination(mut self, value: String) -> Self {
        self.combination = Some(value);
        self
    }

    pub fn event_platform(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentEventPlatformBlock>,
    ) -> Self {
        self.event_platform = Some(value);
        self
    }

    pub fn journeys(mut self, value: Vec<crate::datadogV2::model::SegmentJourneyBlock>) -> Self {
        self.journeys = Some(value);
        self
    }

    pub fn reference_table(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentReferenceTableBlock>,
    ) -> Self {
        self.reference_table = Some(value);
        self
    }

    pub fn retentions(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.retentions = Some(value);
        self
    }

    pub fn static_(mut self, value: Vec<crate::datadogV2::model::SegmentStaticBlock>) -> Self {
        self.static_ = Some(value);
        self
    }

    pub fn templates(mut self, value: Vec<crate::datadogV2::model::SegmentTemplateBlock>) -> Self {
        self.templates = Some(value);
        self
    }

    pub fn user_store(
        mut self,
        value: Vec<crate::datadogV2::model::SegmentProfileStoreBlock>,
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

impl Default for SegmentDataQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SegmentDataQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentDataQueryVisitor;
        impl<'a> Visitor<'a> for SegmentDataQueryVisitor {
            type Value = SegmentDataQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account_store: Option<
                    Vec<crate::datadogV2::model::SegmentProfileStoreBlock>,
                > = None;
                let mut combination: Option<String> = None;
                let mut event_platform: Option<
                    Vec<crate::datadogV2::model::SegmentEventPlatformBlock>,
                > = None;
                let mut journeys: Option<Vec<crate::datadogV2::model::SegmentJourneyBlock>> = None;
                let mut reference_table: Option<
                    Vec<crate::datadogV2::model::SegmentReferenceTableBlock>,
                > = None;
                let mut retentions: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut static_: Option<Vec<crate::datadogV2::model::SegmentStaticBlock>> = None;
                let mut templates: Option<Vec<crate::datadogV2::model::SegmentTemplateBlock>> =
                    None;
                let mut user_store: Option<Vec<crate::datadogV2::model::SegmentProfileStoreBlock>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account_store" => {
                            if v.is_null() {
                                continue;
                            }
                            account_store =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "journeys" => {
                            if v.is_null() {
                                continue;
                            }
                            journeys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_table" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_table =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retentions" => {
                            if v.is_null() {
                                continue;
                            }
                            retentions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "static" => {
                            if v.is_null() {
                                continue;
                            }
                            static_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "templates" => {
                            if v.is_null() {
                                continue;
                            }
                            templates = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = SegmentDataQuery {
                    account_store,
                    combination,
                    event_platform,
                    journeys,
                    reference_table,
                    retentions,
                    static_,
                    templates,
                    user_store,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentDataQueryVisitor)
    }
}
