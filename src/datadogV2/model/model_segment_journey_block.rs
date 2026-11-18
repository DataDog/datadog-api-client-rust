// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Selects the users who converted or dropped off in a funnel.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SegmentJourneyBlock {
    /// Whether to return the entities that converted at the target step, or those that dropped off.
    #[serde(rename = "conversion_type")]
    pub conversion_type: Option<crate::datadogV2::model::ProductAnalyticsJourneyConversionType>,
    /// Whether to leave out anonymous users.
    #[serde(rename = "exclude_anonymous_traffic")]
    pub exclude_anonymous_traffic: Option<bool>,
    /// Start of the time range, in epoch milliseconds.
    #[serde(rename = "from")]
    pub from: Option<i64>,
    /// Group-by definitions of the funnel query.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<std::collections::BTreeMap<String, serde_json::Value>>>,
    /// Name of the block, used in `combination`.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Funnel search, with the same shape as the `search` of a Product Analytics journey query.
    #[serde(rename = "search")]
    pub search: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Funnel step or step range to evaluate.
    #[serde(rename = "target")]
    pub target: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// End of the time range, in epoch milliseconds.
    #[serde(rename = "to")]
    pub to: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SegmentJourneyBlock {
    pub fn new() -> SegmentJourneyBlock {
        SegmentJourneyBlock {
            conversion_type: None,
            exclude_anonymous_traffic: None,
            from: None,
            group_by: None,
            name: None,
            search: None,
            target: None,
            to: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn conversion_type(
        mut self,
        value: crate::datadogV2::model::ProductAnalyticsJourneyConversionType,
    ) -> Self {
        self.conversion_type = Some(value);
        self
    }

    pub fn exclude_anonymous_traffic(mut self, value: bool) -> Self {
        self.exclude_anonymous_traffic = Some(value);
        self
    }

    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn group_by(
        mut self,
        value: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn search(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.search = Some(value);
        self
    }

    pub fn target(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.target = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
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

impl Default for SegmentJourneyBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SegmentJourneyBlock {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SegmentJourneyBlockVisitor;
        impl<'a> Visitor<'a> for SegmentJourneyBlockVisitor {
            type Value = SegmentJourneyBlock;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut conversion_type: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneyConversionType,
                > = None;
                let mut exclude_anonymous_traffic: Option<bool> = None;
                let mut from: Option<i64> = None;
                let mut group_by: Option<
                    Vec<std::collections::BTreeMap<String, serde_json::Value>>,
                > = None;
                let mut name: Option<String> = None;
                let mut search: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut target: Option<std::collections::BTreeMap<String, serde_json::Value>> =
                    None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "conversion_type" => {
                            if v.is_null() {
                                continue;
                            }
                            conversion_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _conversion_type) = conversion_type {
                                match _conversion_type {
                                    crate::datadogV2::model::ProductAnalyticsJourneyConversionType::UnparsedObject(_conversion_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "exclude_anonymous_traffic" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude_anonymous_traffic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            if v.is_null() {
                                continue;
                            }
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            if v.is_null() {
                                continue;
                            }
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            if v.is_null() {
                                continue;
                            }
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SegmentJourneyBlock {
                    conversion_type,
                    exclude_anonymous_traffic,
                    from,
                    group_by,
                    name,
                    search,
                    target,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SegmentJourneyBlockVisitor)
    }
}
