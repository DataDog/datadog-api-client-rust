// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Query definition for the segment. Contains one or more query blocks and an optional combination formula.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumSegmentDataQuery {
    /// Boolean expression combining multiple query blocks.
    #[serde(rename = "combination")]
    pub combination: Option<String>,
    /// List of event platform query blocks.
    #[serde(rename = "event_platforms")]
    pub event_platforms: Option<Vec<crate::datadogV2::model::RumSegmentEventPlatform>>,
    /// List of journey-based query blocks.
    #[serde(rename = "journeys")]
    pub journeys: Option<Vec<crate::datadogV2::model::RumSegmentJourney>>,
    /// List of reference table query blocks.
    #[serde(rename = "reference_tables")]
    pub reference_tables: Option<Vec<crate::datadogV2::model::RumSegmentReferenceTable>>,
    /// List of static user list blocks.
    #[serde(rename = "static")]
    pub static_: Option<Vec<crate::datadogV2::model::RumSegmentStaticEntry>>,
    /// List of template-based query blocks.
    #[serde(rename = "templates")]
    pub templates: Option<Vec<crate::datadogV2::model::RumSegmentTemplateInstance>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumSegmentDataQuery {
    pub fn new() -> RumSegmentDataQuery {
        RumSegmentDataQuery {
            combination: None,
            event_platforms: None,
            journeys: None,
            reference_tables: None,
            static_: None,
            templates: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn combination(mut self, value: String) -> Self {
        self.combination = Some(value);
        self
    }

    pub fn event_platforms(
        mut self,
        value: Vec<crate::datadogV2::model::RumSegmentEventPlatform>,
    ) -> Self {
        self.event_platforms = Some(value);
        self
    }

    pub fn journeys(mut self, value: Vec<crate::datadogV2::model::RumSegmentJourney>) -> Self {
        self.journeys = Some(value);
        self
    }

    pub fn reference_tables(
        mut self,
        value: Vec<crate::datadogV2::model::RumSegmentReferenceTable>,
    ) -> Self {
        self.reference_tables = Some(value);
        self
    }

    pub fn static_(mut self, value: Vec<crate::datadogV2::model::RumSegmentStaticEntry>) -> Self {
        self.static_ = Some(value);
        self
    }

    pub fn templates(
        mut self,
        value: Vec<crate::datadogV2::model::RumSegmentTemplateInstance>,
    ) -> Self {
        self.templates = Some(value);
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

impl Default for RumSegmentDataQuery {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumSegmentDataQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumSegmentDataQueryVisitor;
        impl<'a> Visitor<'a> for RumSegmentDataQueryVisitor {
            type Value = RumSegmentDataQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut combination: Option<String> = None;
                let mut event_platforms: Option<
                    Vec<crate::datadogV2::model::RumSegmentEventPlatform>,
                > = None;
                let mut journeys: Option<Vec<crate::datadogV2::model::RumSegmentJourney>> = None;
                let mut reference_tables: Option<
                    Vec<crate::datadogV2::model::RumSegmentReferenceTable>,
                > = None;
                let mut static_: Option<Vec<crate::datadogV2::model::RumSegmentStaticEntry>> = None;
                let mut templates: Option<
                    Vec<crate::datadogV2::model::RumSegmentTemplateInstance>,
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
                        "event_platforms" => {
                            if v.is_null() {
                                continue;
                            }
                            event_platforms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "journeys" => {
                            if v.is_null() {
                                continue;
                            }
                            journeys = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reference_tables" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_tables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumSegmentDataQuery {
                    combination,
                    event_platforms,
                    journeys,
                    reference_tables,
                    static_,
                    templates,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumSegmentDataQueryVisitor)
    }
}
