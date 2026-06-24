// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a custom forecast.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomForecastResponseDataAttributes {
    /// The UUID of the budget that this custom forecast belongs to.
    #[serde(rename = "budget_uid")]
    pub budget_uid: String,
    /// Timestamp the custom forecast was created, in Unix milliseconds.
    #[serde(rename = "created_at")]
    pub created_at: i64,
    /// The id of the user that created the custom forecast.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Monthly custom forecast entries.
    #[serde(rename = "entries")]
    pub entries: Vec<crate::datadogV2::model::CustomForecastEntry>,
    /// Timestamp the custom forecast was last updated, in Unix milliseconds.
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
    /// The id of the user that last updated the custom forecast.
    #[serde(rename = "updated_by")]
    pub updated_by: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomForecastResponseDataAttributes {
    pub fn new(
        budget_uid: String,
        created_at: i64,
        created_by: String,
        entries: Vec<crate::datadogV2::model::CustomForecastEntry>,
        updated_at: i64,
        updated_by: String,
    ) -> CustomForecastResponseDataAttributes {
        CustomForecastResponseDataAttributes {
            budget_uid,
            created_at,
            created_by,
            entries,
            updated_at,
            updated_by,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CustomForecastResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomForecastResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for CustomForecastResponseDataAttributesVisitor {
            type Value = CustomForecastResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_uid: Option<String> = None;
                let mut created_at: Option<i64> = None;
                let mut created_by: Option<String> = None;
                let mut entries: Option<Vec<crate::datadogV2::model::CustomForecastEntry>> = None;
                let mut updated_at: Option<i64> = None;
                let mut updated_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "budget_uid" => {
                            budget_uid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entries" => {
                            entries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_by" => {
                            updated_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let budget_uid = budget_uid.ok_or_else(|| M::Error::missing_field("budget_uid"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let entries = entries.ok_or_else(|| M::Error::missing_field("entries"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let updated_by = updated_by.ok_or_else(|| M::Error::missing_field("updated_by"))?;

                let content = CustomForecastResponseDataAttributes {
                    budget_uid,
                    created_at,
                    created_by,
                    entries,
                    updated_at,
                    updated_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomForecastResponseDataAttributesVisitor)
    }
}
