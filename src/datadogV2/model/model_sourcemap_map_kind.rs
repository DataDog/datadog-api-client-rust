// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourcemapMapKind {
    JS,
    JVM,
    IOS,
    REACT,
    FLUTTER,
    ELF,
    NDK,
    IL2CPP,
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl ToString for SourcemapMapKind {
    fn to_string(&self) -> String {
        match self {
            Self::JS => String::from("js"),
            Self::JVM => String::from("jvm"),
            Self::IOS => String::from("ios"),
            Self::REACT => String::from("react"),
            Self::FLUTTER => String::from("flutter"),
            Self::ELF => String::from("elf"),
            Self::NDK => String::from("ndk"),
            Self::IL2CPP => String::from("il2cpp"),
            Self::UnparsedObject(v) => v.value.to_string(),
        }
    }
}

impl Serialize for SourcemapMapKind {
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

impl<'de> Deserialize<'de> for SourcemapMapKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "js" => Self::JS,
            "jvm" => Self::JVM,
            "ios" => Self::IOS,
            "react" => Self::REACT,
            "flutter" => Self::FLUTTER,
            "elf" => Self::ELF,
            "ndk" => Self::NDK,
            "il2cpp" => Self::IL2CPP,
            _ => Self::UnparsedObject(crate::datadog::UnparsedObject {
                value: serde_json::Value::String(s.into()),
            }),
        })
    }
}
