use maud::{Markup, html};
use serde::de::Visitor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum CmsPageStatus {
    Draft,
    Published,
}

impl Default for CmsPageStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl TryFrom<&str> for CmsPageStatus {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "draft" => Ok(Self::Draft),
            "published" => Ok(Self::Published),
            _ => Err(()),
        }
    }
}

impl TryFrom<String> for CmsPageStatus {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl From<&CmsPageStatus> for String {
    fn from(s: &CmsPageStatus) -> Self {
        match s {
            CmsPageStatus::Draft => "draft".to_string(),
            CmsPageStatus::Published => "published".to_string(),
        }
    }
}

impl CmsPageStatus {
    pub fn all_statuses() -> Vec<Self> {
        vec![Self::Draft, Self::Published]
    }

    pub fn as_stringed(&self) -> String {
        String::from(self)
    }

    pub fn html_option(&self) -> Markup {
        html! {
            @for status in Self::all_statuses() {
                @if self == &status {
                    option value=(status.as_stringed()) selected {
                        (status.as_stringed())
                    }
                } @else {
                    option value=(status.as_stringed()) {
                        (status.as_stringed())
                    }
                }
            }
        }
    }
}

impl Serialize for CmsPageStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(String::from(self).as_str())
    }
}

impl<'de> Deserialize<'de> for CmsPageStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CmsPageStatusVisitor;

        impl<'de> Visitor<'de> for CmsPageStatusVisitor {
            type Value = CmsPageStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a cms page status")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                CmsPageStatus::try_from(v).map_err(|_| E::custom("invalid cms page status"))
            }
        }

        deserializer.deserialize_str(CmsPageStatusVisitor)
    }
}
