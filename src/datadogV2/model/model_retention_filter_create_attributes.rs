// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object describing the configuration of the retention filter to create/update.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RetentionFilterCreateAttributes {
    /// Enable/Disable the retention filter.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The spans filter. Spans matching this filter will be indexed and stored.
    #[serde(rename = "filter")]
    pub filter: crate::datadogV2::model::SpansFilterCreate,
    /// The type of retention filter. The value should always be spans-sampling-processor.
    #[serde(rename = "filter_type")]
    pub filter_type: crate::datadogV2::model::RetentionFilterType,
    /// The name of the retention filter.
    #[serde(rename = "name")]
    pub name: String,
    /// Sample rate to apply to spans going through this retention filter,
    /// a value of 1.0 keeps all spans matching the query.
    #[serde(rename = "rate")]
    pub rate: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RetentionFilterCreateAttributes {
    pub fn new(
        enabled: bool,
        filter: crate::datadogV2::model::SpansFilterCreate,
        filter_type: crate::datadogV2::model::RetentionFilterType,
        name: String,
        rate: f64,
    ) -> RetentionFilterCreateAttributes {
        RetentionFilterCreateAttributes {
            enabled,
            filter,
            filter_type,
            name,
            rate,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RetentionFilterCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RetentionFilterCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RetentionFilterCreateAttributesVisitor {
            type Value = RetentionFilterCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut filter: Option<crate::datadogV2::model::SpansFilterCreate> = None;
                let mut filter_type: Option<crate::datadogV2::model::RetentionFilterType> = None;
                let mut name: Option<String> = None;
                let mut rate: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter" => {
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_type" => {
                            filter_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _filter_type) = filter_type {
                                match _filter_type {
                                    crate::datadogV2::model::RetentionFilterType::UnparsedObject(_filter_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate" => {
                            rate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let filter = filter.ok_or_else(|| M::Error::missing_field("filter"))?;
                let filter_type =
                    filter_type.ok_or_else(|| M::Error::missing_field("filter_type"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let rate = rate.ok_or_else(|| M::Error::missing_field("rate"))?;

                let content = RetentionFilterCreateAttributes {
                    enabled,
                    filter,
                    filter_type,
                    name,
                    rate,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RetentionFilterCreateAttributesVisitor)
    }
}
