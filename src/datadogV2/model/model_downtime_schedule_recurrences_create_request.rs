// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A recurring downtime schedule definition.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeScheduleRecurrencesCreateRequest {
    /// A list of downtime recurrences.
    #[serde(rename = "recurrences")]
    pub recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
    /// The timezone in which to schedule the downtime.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeScheduleRecurrencesCreateRequest {
    pub fn new(
        recurrences: Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
    ) -> DowntimeScheduleRecurrencesCreateRequest {
        DowntimeScheduleRecurrencesCreateRequest {
            recurrences,
            timezone: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
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

impl<'de> Deserialize<'de> for DowntimeScheduleRecurrencesCreateRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeScheduleRecurrencesCreateRequestVisitor;
        impl<'a> Visitor<'a> for DowntimeScheduleRecurrencesCreateRequestVisitor {
            type Value = DowntimeScheduleRecurrencesCreateRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut recurrences: Option<
                    Vec<crate::datadogV2::model::DowntimeScheduleRecurrenceCreateUpdateRequest>,
                > = None;
                let mut timezone: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "recurrences" => {
                            recurrences =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let recurrences =
                    recurrences.ok_or_else(|| M::Error::missing_field("recurrences"))?;

                let content = DowntimeScheduleRecurrencesCreateRequest {
                    recurrences,
                    timezone,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeScheduleRecurrencesCreateRequestVisitor)
    }
}
