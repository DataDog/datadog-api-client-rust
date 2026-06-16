// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for setting an indicator's triage state.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IoCTriageWriteRequestAttributes {
    /// The indicator value to triage (for example, an IP address or domain).
    #[serde(rename = "indicator")]
    pub indicator: String,
    /// Current triage state of the indicator.
    #[serde(rename = "triage_state")]
    pub triage_state: crate::datadogV2::model::IoCTriageState,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IoCTriageWriteRequestAttributes {
    pub fn new(
        indicator: String,
        triage_state: crate::datadogV2::model::IoCTriageState,
    ) -> IoCTriageWriteRequestAttributes {
        IoCTriageWriteRequestAttributes {
            indicator,
            triage_state,
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

impl<'de> Deserialize<'de> for IoCTriageWriteRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IoCTriageWriteRequestAttributesVisitor;
        impl<'a> Visitor<'a> for IoCTriageWriteRequestAttributesVisitor {
            type Value = IoCTriageWriteRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut indicator: Option<String> = None;
                let mut triage_state: Option<crate::datadogV2::model::IoCTriageState> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "indicator" => {
                            indicator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triage_state" => {
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let indicator = indicator.ok_or_else(|| M::Error::missing_field("indicator"))?;
                let triage_state =
                    triage_state.ok_or_else(|| M::Error::missing_field("triage_state"))?;

                let content = IoCTriageWriteRequestAttributes {
                    indicator,
                    triage_state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IoCTriageWriteRequestAttributesVisitor)
    }
}
