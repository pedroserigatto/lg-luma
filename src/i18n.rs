//! Bundled translations: English and Portuguese (Brazil), the languages this fork keeps.
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::BTreeMap, sync::OnceLock};
use time::OffsetDateTime;

type Messages = BTreeMap<String, BTreeMap<String, String>>;
fn messages() -> &'static Messages {
    static CATALOG: OnceLock<Messages> = OnceLock::new();
    CATALOG.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/messages.json"))
            .expect("Bundled translations must be valid")
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    En,
    Pt,
}
impl Language {
    pub const ALL: [Self; 2] = [Self::En, Self::Pt];
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Pt => "pt",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Pt => "Português (Brasil)",
        }
    }
    pub fn index(self) -> usize {
        Self::ALL
            .iter()
            .position(|language| *language == self)
            .unwrap()
    }
    pub fn geocoding_code(self) -> &'static str {
        self.code()
    }
    pub fn t(self, key: &str) -> &str {
        messages()
            .get(key)
            .and_then(|entry| entry.get(self.code()).or_else(|| entry.get("en")))
            .map(String::as_str)
            .unwrap_or(key)
    }
    pub fn format(self, key: &str, values: &[(&str, &str)]) -> String {
        let mut value = self.t(key).to_owned();
        for (name, replacement) in values {
            value = value.replace(&format!("{{{name}}}"), replacement);
        }
        value
    }
    pub fn category(self, value: &str) -> &str {
        match value {
            "Streaming" => self.t("categoryStreaming"),
            "Music" => self.t("categoryMusic"),
            "Gaming" => self.t("categoryGaming"),
            "Utilities" => self.t("categoryUtilities"),
            _ => self.t("categoryUtilities"),
        }
    }
    pub fn greeting(self, hour: u8, name: &str) -> String {
        let key = match hour {
            0..=4 => "greetingNight",
            5..=11 => "morning",
            12..=17 => "afternoon",
            _ => "evening",
        };
        let name = name.trim();
        if name.is_empty() {
            self.t(key).to_owned()
        } else {
            self.format(&format!("{key}Named"), &[("name", name)])
        }
    }
    pub fn date(self, now: OffsetDateTime) -> String {
        let weekday = now.weekday().number_days_from_monday() as usize;
        let month = now.month() as usize - 1;
        match self {
            Self::En => format!(
                "{}, {} {}",
                [
                    "Monday",
                    "Tuesday",
                    "Wednesday",
                    "Thursday",
                    "Friday",
                    "Saturday",
                    "Sunday"
                ][weekday],
                [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December"
                ][month],
                now.day()
            )
            .to_uppercase(),
            Self::Pt => format!(
                "{}, {} de {}",
                [
                    "segunda-feira",
                    "terça-feira",
                    "quarta-feira",
                    "quinta-feira",
                    "sexta-feira",
                    "sábado",
                    "domingo"
                ][weekday],
                now.day(),
                [
                    "janeiro",
                    "fevereiro",
                    "março",
                    "abril",
                    "maio",
                    "junho",
                    "julho",
                    "agosto",
                    "setembro",
                    "outubro",
                    "novembro",
                    "dezembro"
                ][month]
            )
            .to_uppercase(),
        }
    }
    pub fn clock(self, now: OffsetDateTime, hour24: bool, seconds: bool) -> String {
        let base = self.clock_base(now, hour24);
        if !seconds {
            return base;
        }
        let tail = format!(":{:02}", now.second());
        match base.find(' ') {
            Some(space) => format!("{}{tail}{}", &base[..space], &base[space..]),
            None => format!("{base}{tail}"),
        }
    }
    fn clock_base(self, now: OffsetDateTime, hour24: bool) -> String {
        if hour24 {
            return if self == Self::En {
                format!("{:02}:{:02}", now.hour(), now.minute())
            } else {
                format!("{}:{:02}", now.hour(), now.minute())
            };
        }
        let hour = match now.hour() % 12 {
            0 => 12,
            value => value,
        };
        format!(
            "{hour}:{:02} {}",
            now.minute(),
            if now.hour() >= 12 { "PM" } else { "AM" }
        )
    }
}
pub fn deserialize_language<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Language, D::Error> {
    Ok(
        match serde_json::Value::deserialize(deserializer)?.as_str() {
            Some("pt") => Language::Pt,
            _ => Language::En,
        },
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_saved_language_this_fork_removed_loads_as_english() {
        for code in ["de", "zh", "zh-tw", "fr", "vi", "unknown"] {
            let preferences: crate::model::Preferences =
                serde_json::from_value(serde_json::json!({"language": code})).unwrap();
            assert_eq!(preferences.language, super::Language::En, "{code}");
        }
        let preferences: crate::model::Preferences =
            serde_json::from_value(serde_json::json!({"language": "pt"})).unwrap();
        assert_eq!(preferences.language, super::Language::Pt);
    }

    #[test]
    fn homebrew_description_lists_every_supported_language() {
        let description = include_str!("../assets/homebrew-description.html");
        for language in super::Language::ALL {
            let name = match language {
                super::Language::En => "English",
                super::Language::Pt => "Portuguese (Brazil)",
            };
            assert!(
                description.contains(name),
                "Homebrew description is missing {name}"
            );
        }
    }
    use super::*;
    #[test]
    fn every_message_has_all_languages_and_no_empty_values() {
        for (key, entry) in messages() {
            for language in Language::ALL {
                assert!(
                    !entry
                        .get(language.code())
                        .unwrap_or_else(|| panic!("Missing {key} / {}", language.code()))
                        .is_empty()
                );
            }
        }
    }
    #[test]
    fn every_translation_preserves_the_english_message_placeholders() {
        fn placeholders(message: &str) -> std::collections::BTreeSet<&str> {
            message
                .split('{')
                .skip(1)
                .map(|part| {
                    part.split_once('}')
                        .expect("Unclosed message placeholder")
                        .0
                })
                .collect()
        }

        for (key, entry) in messages() {
            let expected = placeholders(&entry["en"]);
            for language in Language::ALL {
                assert_eq!(
                    placeholders(&entry[language.code()]),
                    expected,
                    "Mismatched placeholders in {key} / {}",
                    language.code()
                );
            }
        }
    }
    #[test]
    fn localization_keeps_user_provided_names_intact() {
        assert_eq!(
            Language::Pt.format("favoriteAdded", &[("app", "Netflix")]),
            "Netflix adicionado aos favoritos."
        );
        assert_eq!(Language::En.greeting(15, ""), "Good afternoon.");
        assert_eq!(Language::En.greeting(15, "  Ana "), "Good afternoon, Ana.");
    }
    #[test]
    fn clock_and_date_follow_the_selected_locale() {
        let now = OffsetDateTime::parse(
            "2026-10-21T20:41:00+02:00",
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap();
        assert_eq!(Language::Pt.date(now), "QUARTA-FEIRA, 21 DE OUTUBRO");
        assert_eq!(Language::En.clock(now, true, false), "20:41");
        assert_eq!(Language::En.clock(now, true, true), "20:41:00");
        assert_eq!(Language::En.clock(now, false, true), "8:41:00 PM");
    }
    #[test]
    fn portuguese_dates_include_accented_names_and_handle_leap_days() {
        for (date, expected) in [
            ("2024-02-29T00:05:00-03:00", "QUINTA-FEIRA, 29 DE FEVEREIRO"),
            ("2026-03-03T12:00:00-03:00", "TERÇA-FEIRA, 3 DE MARÇO"),
            ("2026-10-24T20:41:00-03:00", "SÁBADO, 24 DE OUTUBRO"),
        ] {
            let now = OffsetDateTime::parse(date, &time::format_description::well_known::Rfc3339)
                .unwrap();
            assert_eq!(Language::Pt.date(now), expected);
        }
    }
}
