// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Which sections the digest shows. Sent as a whole: sections left out of a request use their defaults, which
/// show everything.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleMessageCustomization {
    /// Links the digest shows. `view_in_ccm` is the link to review the recommendations in Cloud Cost Management.
    #[serde(rename = "actions")]
    pub actions: Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageActions>,
    /// What the digest header shows.
    #[serde(rename = "header")]
    pub header: Option<
        crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationHeader,
    >,
    /// Tag keys whose values are shown on each recommendation in the digest.
    #[serde(rename = "include_tags")]
    pub include_tags: Option<Vec<String>>,
    /// How the message is customized.
    #[serde(rename = "mode")]
    pub mode:
        Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationMode>,
    /// What each recommendation in the digest shows.
    #[serde(rename = "per_rec")]
    pub per_rec: Option<
        crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationPerRec,
    >,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleMessageCustomization {
    pub fn new() -> CostRecommendationNotificationRuleMessageCustomization {
        CostRecommendationNotificationRuleMessageCustomization {
            actions: None,
            header: None,
            include_tags: None,
            mode: None,
            per_rec: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn actions(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageActions,
    ) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn header(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationHeader,
    ) -> Self {
        self.header = Some(value);
        self
    }

    pub fn include_tags(mut self, value: Vec<String>) -> Self {
        self.include_tags = Some(value);
        self
    }

    pub fn mode(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationMode,
    ) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn per_rec(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationPerRec,
    ) -> Self {
        self.per_rec = Some(value);
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

impl Default for CostRecommendationNotificationRuleMessageCustomization {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleMessageCustomization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleMessageCustomizationVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleMessageCustomizationVisitor {
            type Value = CostRecommendationNotificationRuleMessageCustomization;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut actions: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleMessageActions,
                > = None;
                let mut header: Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationHeader> = None;
                let mut include_tags: Option<Vec<String>> = None;
                let mut mode: Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationMode> = None;
                let mut per_rec: Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationPerRec> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "actions" => {
                            if v.is_null() {
                                continue;
                            }
                            actions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header" => {
                            if v.is_null() {
                                continue;
                            }
                            header = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include_tags" => {
                            if v.is_null() {
                                continue;
                            }
                            include_tags =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            if v.is_null() {
                                continue;
                            }
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomizationMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "per_rec" => {
                            if v.is_null() {
                                continue;
                            }
                            per_rec = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleMessageCustomization {
                    actions,
                    header,
                    include_tags,
                    mode,
                    per_rec,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationNotificationRuleMessageCustomizationVisitor)
    }
}
