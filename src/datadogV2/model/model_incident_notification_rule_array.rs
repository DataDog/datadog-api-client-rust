// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response with notification rules.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentNotificationRuleArray {
    /// The `NotificationRuleArray` `data`.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::IncidentNotificationRuleResponseData>,
    /// Related objects that are included in the response.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::IncidentNotificationRuleIncludedItems>>,
    /// Response metadata.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::IncidentNotificationRuleArrayMeta>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentNotificationRuleArray {
    pub fn new(
        data: Vec<crate::datadogV2::model::IncidentNotificationRuleResponseData>,
    ) -> IncidentNotificationRuleArray {
        IncidentNotificationRuleArray {
            data,
            included: None,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn included(
        mut self,
        value: Vec<crate::datadogV2::model::IncidentNotificationRuleIncludedItems>,
    ) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::IncidentNotificationRuleArrayMeta,
    ) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for IncidentNotificationRuleArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentNotificationRuleArrayVisitor;
        impl<'a> Visitor<'a> for IncidentNotificationRuleArrayVisitor {
            type Value = IncidentNotificationRuleArray;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Vec<crate::datadogV2::model::IncidentNotificationRuleResponseData>,
                > = None;
                let mut included: Option<
                    Vec<crate::datadogV2::model::IncidentNotificationRuleIncludedItems>,
                > = None;
                let mut meta: Option<crate::datadogV2::model::IncidentNotificationRuleArrayMeta> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = IncidentNotificationRuleArray {
                    data,
                    included,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentNotificationRuleArrayVisitor)
    }
}
