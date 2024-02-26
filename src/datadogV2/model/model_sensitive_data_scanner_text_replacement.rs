// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Object describing how the scanned event will be replaced.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

impl SensitiveDataScannerTextReplacement {
    pub fn new() -> SensitiveDataScannerTextReplacement {
        SensitiveDataScannerTextReplacement {
            number_of_chars: None,
            replacement_string: None,
            type_: None,
        }
    }

    pub fn number_of_chars(&mut self, value: i64) -> &mut Self {
        self.number_of_chars = Some(value);
        self
    }

    pub fn replacement_string(&mut self, value: String) -> &mut Self {
        self.replacement_string = Some(value);
        self
    }

    pub fn type_(
        &mut self,
        value: crate::datadogV2::model::SensitiveDataScannerTextReplacementType,
    ) -> &mut Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for SensitiveDataScannerTextReplacement {
    fn default() -> Self {
        Self::new()
    }
}
