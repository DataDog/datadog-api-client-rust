// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Language {
    PYTHON,
    JAVASCRIPT,
    TYPESCRIPT,
    JAVA,
    GO,
    YAML,
    RUBY,
    CSHARP,
    PHP,
    KOTLIN,
    SWIFT,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match self {
            Self::PYTHON => String::from("PYTHON"),
            Self::JAVASCRIPT => String::from("JAVASCRIPT"),
            Self::TYPESCRIPT => String::from("TYPESCRIPT"),
            Self::JAVA => String::from("JAVA"),
            Self::GO => String::from("GO"),
            Self::YAML => String::from("YAML"),
            Self::RUBY => String::from("RUBY"),
            Self::CSHARP => String::from("CSHARP"),
            Self::PHP => String::from("PHP"),
            Self::KOTLIN => String::from("KOTLIN"),
            Self::SWIFT => String::from("SWIFT"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnparsedObject(v) => v.serialize(serializer),
            _ => serializer.serialize_str(self.to_string().as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PYTHON" => Self::PYTHON,
            "JAVASCRIPT" => Self::JAVASCRIPT,
            "TYPESCRIPT" => Self::TYPESCRIPT,
            "JAVA" => Self::JAVA,
            "GO" => Self::GO,
            "YAML" => Self::YAML,
            "RUBY" => Self::RUBY,
            "CSHARP" => Self::CSHARP,
            "PHP" => Self::PHP,
            "KOTLIN" => Self::KOTLIN,
            "SWIFT" => Self::SWIFT,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
