// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Links the digest shows. `view_in_ccm` is the link to review the recommendations in Cloud Cost Management.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleMessageActions {
    /// Where a message action appears.
    #[serde(rename = "view_in_ccm")]
    pub view_in_ccm:
        Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageAction>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleMessageActions {
    pub fn new() -> CostRecommendationNotificationRuleMessageActions {
        CostRecommendationNotificationRuleMessageActions {
            view_in_ccm: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn view_in_ccm(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageAction,
    ) -> Self {
        self.view_in_ccm = Some(value);
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

impl Default for CostRecommendationNotificationRuleMessageActions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleMessageActions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleMessageActionsVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleMessageActionsVisitor {
            type Value = CostRecommendationNotificationRuleMessageActions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut view_in_ccm: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleMessageAction,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "view_in_ccm" => {
                            if v.is_null() {
                                continue;
                            }
                            view_in_ccm =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleMessageActions {
                    view_in_ccm,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationNotificationRuleMessageActionsVisitor)
    }
}
