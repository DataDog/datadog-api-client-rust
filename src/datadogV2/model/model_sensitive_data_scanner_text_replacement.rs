// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing how the scanned event will be replaced.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SensitiveDataScannerTextReplacement {
    /// Required if type == 'partial_replacement_from_beginning'
    /// or 'partial_replacement_from_end'. It must be > 0.
    #[serde(rename = "number_of_chars")]
    pub number_of_chars: Option<i64>,
    /// Required if type == 'replacement_string'.
    #[serde(rename = "replacement_string")]
    pub replacement_string: Option<String>,
    /// Type of the replacement text. None means no replacement.
    /// hash means the data will be stubbed. replacement_string means that
    /// one can chose a text to replace the data. partial_replacement_from_beginning
    /// allows a user to partially replace the data from the beginning, and
    /// partial_replacement_from_end on the other hand, allows to replace data from
    /// the end.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::SensitiveDataScannerTextReplacementType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SensitiveDataScannerTextReplacement {
    pub fn new() -> SensitiveDataScannerTextReplacement {
        SensitiveDataScannerTextReplacement {
            number_of_chars: None,
            replacement_string: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn number_of_chars(mut self, value: i64) -> Self {
        self.number_of_chars = Some(value);
        self
    }

    pub fn replacement_string(mut self, value: String) -> Self {
        self.replacement_string = Some(value);
        self
    }

    pub fn type_(
        mut self,
        value: crate::datadogV2::model::SensitiveDataScannerTextReplacementType,
    ) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerTextReplacement {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SensitiveDataScannerTextReplacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SensitiveDataScannerTextReplacementVisitor;
        impl<'a> Visitor<'a> for SensitiveDataScannerTextReplacementVisitor {
            type Value = SensitiveDataScannerTextReplacement;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut number_of_chars: Option<i64> = None;
                let mut replacement_string: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::SensitiveDataScannerTextReplacementType,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "number_of_chars" => {
                            if v.is_null() {
                                continue;
                            }
                            number_of_chars =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "replacement_string" => {
                            if v.is_null() {
                                continue;
                            }
                            replacement_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SensitiveDataScannerTextReplacementType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = SensitiveDataScannerTextReplacement {
                    number_of_chars,
                    replacement_string,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SensitiveDataScannerTextReplacementVisitor)
    }
}
