// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing the Datadog rum-based metric to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumMetricCreateAttributes {
    /// The compute rule to compute the rum-based metric.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::RumMetricCompute,
    /// The type of RUM events to filter on.
    #[serde(rename = "event_type")]
    pub event_type: crate::datadogV2::model::RumMetricEventType,
    /// The rum-based metric filter. Events matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::RumMetricFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::RumMetricGroupBy>>,
    /// The rule to count updatable events. Is only set if "event_type" is "sessions" or "views".
    #[serde(rename = "uniqueness")]
    pub uniqueness: Option<crate::datadogV2::model::RumMetricUniqueness>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumMetricCreateAttributes {
    pub fn new(
        compute: crate::datadogV2::model::RumMetricCompute,
        event_type: crate::datadogV2::model::RumMetricEventType,
    ) -> RumMetricCreateAttributes {
        RumMetricCreateAttributes {
            compute,
            event_type,
            filter: None,
            group_by: None,
            uniqueness: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::RumMetricFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::RumMetricGroupBy>) -> Self {
        self.group_by = Some(value);
        self
    }

    pub fn uniqueness(mut self, value: crate::datadogV2::model::RumMetricUniqueness) -> Self {
        self.uniqueness = Some(value);
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

impl<'de> Deserialize<'de> for RumMetricCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumMetricCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RumMetricCreateAttributesVisitor {
            type Value = RumMetricCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::RumMetricCompute> = None;
                let mut event_type: Option<crate::datadogV2::model::RumMetricEventType> = None;
                let mut filter: Option<crate::datadogV2::model::RumMetricFilter> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::RumMetricGroupBy>> = None;
                let mut uniqueness: Option<crate::datadogV2::model::RumMetricUniqueness> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_type" => {
                            event_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _event_type) = event_type {
                                match _event_type {
                                    crate::datadogV2::model::RumMetricEventType::UnparsedObject(
                                        _event_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "uniqueness" => {
                            if v.is_null() {
                                continue;
                            }
                            uniqueness = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let event_type = event_type.ok_or_else(|| M::Error::missing_field("event_type"))?;

                let content = RumMetricCreateAttributes {
                    compute,
                    event_type,
                    filter,
                    group_by,
                    uniqueness,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumMetricCreateAttributesVisitor)
    }
}
