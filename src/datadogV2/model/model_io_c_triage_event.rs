// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A single entry in an indicator's triage history timeline.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IoCTriageEvent {
    /// Current triage state of the indicator.
    #[serde(rename = "triage_state")]
    pub triage_state: Option<crate::datadogV2::model::IoCTriageState>,
    /// Timestamp when this triage action occurred.
    #[serde(rename = "triaged_at")]
    pub triaged_at: Option<chrono::DateTime<chrono::Utc>>,
    /// UUID of the user who performed this triage action.
    #[serde(rename = "triaged_by")]
    pub triaged_by: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IoCTriageEvent {
    pub fn new() -> IoCTriageEvent {
        IoCTriageEvent {
            triage_state: None,
            triaged_at: None,
            triaged_by: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn triage_state(mut self, value: crate::datadogV2::model::IoCTriageState) -> Self {
        self.triage_state = Some(value);
        self
    }

    pub fn triaged_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.triaged_at = Some(value);
        self
    }

    pub fn triaged_by(mut self, value: String) -> Self {
        self.triaged_by = Some(value);
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

impl Default for IoCTriageEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IoCTriageEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IoCTriageEventVisitor;
        impl<'a> Visitor<'a> for IoCTriageEventVisitor {
            type Value = IoCTriageEvent;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut triage_state: Option<crate::datadogV2::model::IoCTriageState> = None;
                let mut triaged_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut triaged_by: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "triage_state" => {
                            if v.is_null() {
                                continue;
                            }
                            triage_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _triage_state) = triage_state {
                                match _triage_state {
                                    crate::datadogV2::model::IoCTriageState::UnparsedObject(
                                        _triage_state,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "triaged_at" => {
                            if v.is_null() {
                                continue;
                            }
                            triaged_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triaged_by" => {
                            if v.is_null() {
                                continue;
                            }
                            triaged_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = IoCTriageEvent {
                    triage_state,
                    triaged_at,
                    triaged_by,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IoCTriageEventVisitor)
    }
}
