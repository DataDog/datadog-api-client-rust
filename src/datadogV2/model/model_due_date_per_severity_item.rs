// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A mapping of a severity level to the number of days until a finding is due.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DueDatePerSeverityItem {
    /// The number of days from the reference point until the finding is due.
    #[serde(rename = "due_in_days")]
    pub due_in_days: i64,
    /// A severity level used to configure due date thresholds.
    #[serde(rename = "severity")]
    pub severity: crate::datadogV2::model::DueDateSeverity,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DueDatePerSeverityItem {
    pub fn new(
        due_in_days: i64,
        severity: crate::datadogV2::model::DueDateSeverity,
    ) -> DueDatePerSeverityItem {
        DueDatePerSeverityItem {
            due_in_days,
            severity,
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

impl<'de> Deserialize<'de> for DueDatePerSeverityItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DueDatePerSeverityItemVisitor;
        impl<'a> Visitor<'a> for DueDatePerSeverityItemVisitor {
            type Value = DueDatePerSeverityItem;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut due_in_days: Option<i64> = None;
                let mut severity: Option<crate::datadogV2::model::DueDateSeverity> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "due_in_days" => {
                            due_in_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "severity" => {
                            severity = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _severity) = severity {
                                match _severity {
                                    crate::datadogV2::model::DueDateSeverity::UnparsedObject(
                                        _severity,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let due_in_days =
                    due_in_days.ok_or_else(|| M::Error::missing_field("due_in_days"))?;
                let severity = severity.ok_or_else(|| M::Error::missing_field("severity"))?;

                let content = DueDatePerSeverityItem {
                    due_in_days,
                    severity,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DueDatePerSeverityItemVisitor)
    }
}
