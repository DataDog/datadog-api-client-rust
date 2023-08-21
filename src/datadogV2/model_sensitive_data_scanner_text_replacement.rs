// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensitiveDataScannerTextReplacement {
    /// Required if type == 'partial_replacement_from_beginning'
or 'partial_replacement_from_end'. It must be > 0.
    #[serde(rename = "number_of_chars", skip_serializing_if = "Option::is_none")]
    pub number_of_chars: i64,
    /// Required if type == 'replacement_string'.
    #[serde(rename = "replacement_string", skip_serializing_if = "Option::is_none")]
    pub replacement_string: String,
    /// Type of the replacement text. None means no replacement.
hash means the data will be stubbed. replacement_string means that
one can chose a text to replace the data. partial_replacement_from_beginning
allows a user to partially replace the data from the beginning, and
partial_replacement_from_end on the other hand, allows to replace data from
the end.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: SensitiveDataScannerTextReplacementType,
}

