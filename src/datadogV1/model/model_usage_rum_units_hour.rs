// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Number of RUM Units used for each hour for a given organization (data available as of November 1, 2021).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageRumUnitsHour {
    /// The number of browser RUM units.
    #[serde(
        rename = "browser_rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub browser_rum_units: Option<Option<i64>>,
    /// The number of mobile RUM units.
    #[serde(
        rename = "mobile_rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub mobile_rum_units: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Total RUM units across mobile and browser RUM.
    #[serde(
        rename = "rum_units",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub rum_units: Option<Option<i64>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageRumUnitsHour {
    pub fn new() -> UsageRumUnitsHour {
        UsageRumUnitsHour {
            browser_rum_units: None,
            mobile_rum_units: None,
            org_name: None,
            public_id: None,
            rum_units: None,
            _unparsed: false,
        }
    }

    pub fn browser_rum_units(mut self, value: Option<i64>) -> Self {
        self.browser_rum_units = Some(value);
        self
    }

    pub fn mobile_rum_units(mut self, value: Option<i64>) -> Self {
        self.mobile_rum_units = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn rum_units(mut self, value: Option<i64>) -> Self {
        self.rum_units = Some(value);
        self
    }
}

impl Default for UsageRumUnitsHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageRumUnitsHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageRumUnitsHourVisitor;
        impl<'a> Visitor<'a> for UsageRumUnitsHourVisitor {
            type Value = UsageRumUnitsHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut browser_rum_units: Option<Option<i64>> = None;
                let mut mobile_rum_units: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut rum_units: Option<Option<i64>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "browser_rum_units" => {
                            browser_rum_units =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mobile_rum_units" => {
                            mobile_rum_units =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_units" => {
                            rum_units = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageRumUnitsHour {
                    browser_rum_units,
                    mobile_rum_units,
                    org_name,
                    public_id,
                    rum_units,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageRumUnitsHourVisitor)
    }
}
