// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A tag filter. A rule applies to a resource only when every filter of the rule matches.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationCustomRuleFilter {
    /// How the filter compares the tag. `is`, `is_not`, `contains`, `does_not_contain`, `like` (glob), and `not_like`
    /// compare the tag values with `values`. `all` matches resources that have the tag and `untagged` matches resources
    /// that do not, so both ignore `values`.
    #[serde(rename = "filter_type")]
    pub filter_type: crate::datadogV2::model::CostRecommendationCustomRuleFilterType,
    /// The tag key the filter compares.
    #[serde(rename = "tag")]
    pub tag: String,
    /// Tag values the filter compares. Required unless `filter_type` is `all` or `untagged`.
    #[serde(rename = "values")]
    pub values: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationCustomRuleFilter {
    pub fn new(
        filter_type: crate::datadogV2::model::CostRecommendationCustomRuleFilterType,
        tag: String,
    ) -> CostRecommendationCustomRuleFilter {
        CostRecommendationCustomRuleFilter {
            filter_type,
            tag,
            values: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn values(mut self, value: Vec<String>) -> Self {
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

impl<'de> Deserialize<'de> for CostRecommendationCustomRuleFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationCustomRuleFilterVisitor;
        impl<'a> Visitor<'a> for CostRecommendationCustomRuleFilterVisitor {
            type Value = CostRecommendationCustomRuleFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter_type: Option<
                    crate::datadogV2::model::CostRecommendationCustomRuleFilterType,
                > = None;
                let mut tag: Option<String> = None;
                let mut values: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter_type" => {
                            filter_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filter_type) = filter_type {
                                match _filter_type {
                                    crate::datadogV2::model::CostRecommendationCustomRuleFilterType::UnparsedObject(_filter_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tag" => {
                            tag = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let filter_type =
                    filter_type.ok_or_else(|| M::Error::missing_field("filter_type"))?;
                let tag = tag.ok_or_else(|| M::Error::missing_field("tag"))?;

                let content = CostRecommendationCustomRuleFilter {
                    filter_type,
                    tag,
                    values,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationCustomRuleFilterVisitor)
    }
}
