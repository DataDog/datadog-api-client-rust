// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Information about a live call that has ended.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnCallPageLiveCallOver {
    /// The outcome of a live call that has ended.
    #[serde(rename = "outcome")]
    pub outcome: Option<crate::datadogV2::model::OnCallPageLiveCallOutcome>,
    /// When support hours start, if the call was placed outside support hours.
    #[serde(rename = "support_hours_start_at")]
    pub support_hours_start_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnCallPageLiveCallOver {
    pub fn new() -> OnCallPageLiveCallOver {
        OnCallPageLiveCallOver {
            outcome: None,
            support_hours_start_at: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn outcome(mut self, value: crate::datadogV2::model::OnCallPageLiveCallOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn support_hours_start_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.support_hours_start_at = Some(value);
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

impl Default for OnCallPageLiveCallOver {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for OnCallPageLiveCallOver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnCallPageLiveCallOverVisitor;
        impl<'a> Visitor<'a> for OnCallPageLiveCallOverVisitor {
            type Value = OnCallPageLiveCallOver;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut outcome: Option<crate::datadogV2::model::OnCallPageLiveCallOutcome> = None;
                let mut support_hours_start_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "outcome" => {
                            if v.is_null() {
                                continue;
                            }
                            outcome = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _outcome) = outcome {
                                match _outcome {
                                    crate::datadogV2::model::OnCallPageLiveCallOutcome::UnparsedObject(_outcome) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "support_hours_start_at" => {
                            if v.is_null() {
                                continue;
                            }
                            support_hours_start_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = OnCallPageLiveCallOver {
                    outcome,
                    support_hours_start_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnCallPageLiveCallOverVisitor)
    }
}
