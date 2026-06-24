// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a custom forecast upsert request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomForecastUpsertRequestDataAttributes {
    /// The UUID of the budget that this custom forecast belongs to.
    #[serde(rename = "budget_uid")]
    pub budget_uid: String,
    /// Monthly custom forecast entries. An empty list deletes any existing
    /// custom forecast for the budget.
    #[serde(rename = "entries")]
    pub entries: Vec<crate::datadogV2::model::CustomForecastEntry>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomForecastUpsertRequestDataAttributes {
    pub fn new(
        budget_uid: String,
        entries: Vec<crate::datadogV2::model::CustomForecastEntry>,
    ) -> CustomForecastUpsertRequestDataAttributes {
        CustomForecastUpsertRequestDataAttributes {
            budget_uid,
            entries,
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

impl<'de> Deserialize<'de> for CustomForecastUpsertRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomForecastUpsertRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for CustomForecastUpsertRequestDataAttributesVisitor {
            type Value = CustomForecastUpsertRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut budget_uid: Option<String> = None;
                let mut entries: Option<Vec<crate::datadogV2::model::CustomForecastEntry>> = None;
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
                        "entries" => {
                            entries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let budget_uid = budget_uid.ok_or_else(|| M::Error::missing_field("budget_uid"))?;
                let entries = entries.ok_or_else(|| M::Error::missing_field("entries"))?;

                let content = CustomForecastUpsertRequestDataAttributes {
                    budget_uid,
                    entries,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomForecastUpsertRequestDataAttributesVisitor)
    }
}
