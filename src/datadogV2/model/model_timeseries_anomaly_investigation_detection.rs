// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Anomaly detection configuration used for the result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationDetection {
    /// Source of the anomaly detection configuration.
    #[serde(rename = "configuration_source")]
    pub configuration_source:
        crate::datadogV2::model::TimeseriesAnomalyInvestigationConfigurationSource,
    /// Applied Watchdog Explains profile, or null when the request supplied an explicit `anomalies()` formula. The current Watchdog profile is `watchdog_explains_v1`.
    #[serialize_always]
    #[serde(rename = "profile")]
    pub profile: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationDetection {
    pub fn new(
        configuration_source: crate::datadogV2::model::TimeseriesAnomalyInvestigationConfigurationSource,
        profile: Option<String>,
    ) -> TimeseriesAnomalyInvestigationDetection {
        TimeseriesAnomalyInvestigationDetection {
            configuration_source,
            profile,
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationDetection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationDetectionVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationDetectionVisitor {
            type Value = TimeseriesAnomalyInvestigationDetection;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration_source: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationConfigurationSource,
                > = None;
                let mut profile: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration_source" => {
                            configuration_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _configuration_source) = configuration_source {
                                match _configuration_source {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationConfigurationSource::UnparsedObject(_configuration_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "profile" => {
                            profile = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let configuration_source = configuration_source
                    .ok_or_else(|| M::Error::missing_field("configuration_source"))?;
                let profile = profile.ok_or_else(|| M::Error::missing_field("profile"))?;

                let content = TimeseriesAnomalyInvestigationDetection {
                    configuration_source,
                    profile,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationDetectionVisitor)
    }
}
