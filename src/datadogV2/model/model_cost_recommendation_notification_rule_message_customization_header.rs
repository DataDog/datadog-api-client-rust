// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// What the digest header shows.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleMessageCustomizationHeader {
    /// Text shown in the header, up to 500 characters. It can use the `{{ total_savings }}`, `{{ rec_count }}`,
    /// and `{{ digest_date }}` variables.
    #[serde(rename = "custom_text")]
    pub custom_text: Option<String>,
    /// Whether the header shows the rule name.
    #[serde(rename = "notification_name")]
    pub notification_name: Option<bool>,
    /// Whether the header shows the total potential savings.
    #[serde(rename = "total_savings")]
    pub total_savings: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleMessageCustomizationHeader {
    pub fn new() -> CostRecommendationNotificationRuleMessageCustomizationHeader {
        CostRecommendationNotificationRuleMessageCustomizationHeader {
            custom_text: None,
            notification_name: None,
            total_savings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_text(mut self, value: String) -> Self {
        self.custom_text = Some(value);
        self
    }

    pub fn notification_name(mut self, value: bool) -> Self {
        self.notification_name = Some(value);
        self
    }

    pub fn total_savings(mut self, value: bool) -> Self {
        self.total_savings = Some(value);
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

impl Default for CostRecommendationNotificationRuleMessageCustomizationHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleMessageCustomizationHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleMessageCustomizationHeaderVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleMessageCustomizationHeaderVisitor {
            type Value = CostRecommendationNotificationRuleMessageCustomizationHeader;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_text: Option<String> = None;
                let mut notification_name: Option<bool> = None;
                let mut total_savings: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_text" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_text =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notification_name" => {
                            if v.is_null() {
                                continue;
                            }
                            notification_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_savings" => {
                            if v.is_null() {
                                continue;
                            }
                            total_savings =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleMessageCustomizationHeader {
                    custom_text,
                    notification_name,
                    total_savings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CostRecommendationNotificationRuleMessageCustomizationHeaderVisitor)
    }
}
