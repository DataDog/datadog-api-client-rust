// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing the Datadog span-based metric to create.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SpansMetricCreateAttributes {
    /// The compute rule to compute the span-based metric.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::SpansMetricCompute,
    /// The span-based metric filter. Spans matching this filter will be aggregated in this metric.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV2::model::SpansMetricFilter>,
    /// The rules for the group by.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV2::model::SpansMetricGroupBy>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SpansMetricCreateAttributes {
    pub fn new(
        compute: crate::datadogV2::model::SpansMetricCompute,
    ) -> SpansMetricCreateAttributes {
        SpansMetricCreateAttributes {
            compute,
            filter: None,
            group_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV2::model::SpansMetricFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn group_by(mut self, value: Vec<crate::datadogV2::model::SpansMetricGroupBy>) -> Self {
        self.group_by = Some(value);
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

impl<'de> Deserialize<'de> for SpansMetricCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpansMetricCreateAttributesVisitor;
        impl<'a> Visitor<'a> for SpansMetricCreateAttributesVisitor {
            type Value = SpansMetricCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::SpansMetricCompute> = None;
                let mut filter: Option<crate::datadogV2::model::SpansMetricFilter> = None;
                let mut group_by: Option<Vec<crate::datadogV2::model::SpansMetricGroupBy>> = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;

                let content = SpansMetricCreateAttributes {
                    compute,
                    filter,
                    group_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SpansMetricCreateAttributesVisitor)
    }
}
