use lotus_script::{content::ContentId, pis::PisSpGroup};

use crate::elements::std::pis_sp_helper::split_into_lines;

pub fn get_announcements_by_station_id(id: u32) -> Vec<ContentId> {
    let mut res = Vec::new();

    let sp_content_id_opt = PisSpGroup::get_content_id("ANNOUNCEMENT");

    if let Some(sp_content_id) = sp_content_id_opt {
        let group_strings = PisSpGroup::get_group_strings(sp_content_id).replace("\t", "\n");

        let lines = split_into_lines(&group_strings);

        for chunk in lines.chunks_exact(2) {
            let user_id = match chunk[0].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };

            let sub_id = match chunk[1].parse::<i32>() {
                Ok(value) => value,
                Err(_) => continue,
            };

            res.push(ContentId { user_id, sub_id });
        }
    }

    res
}

//----------------------------------------

#[derive(Debug, Default)]
pub struct SpecialAnnouncement {
    pub event_id: i32,
    pub text: String,
    pub content_id: ContentId,
    pub code: i32,
    pub target: i32,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: String,
}

impl Property {
    pub fn parse(line: &str) -> Option<Self> {
        let (name, value) = line.split_once('=')?;
        let name = name.trim().to_string();

        let value = value.trim();
        let value = if value.len() >= 2 && value.starts_with('"') && value.ends_with('"') {
            &value[1..value.len() - 1]
        } else {
            value
        };

        Some(Property {
            name,
            value: value.to_string(),
        })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum Section {
    #[default]
    None,
    SpecialAnnouncement,
    SpecialAnnouncementDefaults,
}

pub fn load_special_announcements() -> Vec<SpecialAnnouncement> {
    let mut res = Vec::new();

    let sp_content_id_opt = PisSpGroup::get_content_id("ANNOUNCEMENT");

    if let Some(sp_content_id) = sp_content_id_opt {
        let group_strings = PisSpGroup::get_group_strings(sp_content_id).replace("\t", "\n");

        let lines = split_into_lines(&group_strings);

        let mut section = Section::None;
        let mut user_id: Option<i32> = None;

        let mut pre_event_id = 0;
        let mut pre_text = "".to_string();
        let mut pre_content_user_id = 0;
        let mut pre_content_sub_id = 0;
        let mut pre_code = 0;
        let mut pre_target = 0;

        for raw_line in lines {
            let line = raw_line.trim();

            // Kommentare
            if line.starts_with(';') {
                continue;
            }

            // Sections
            if line == "[SpecialAnnouncement]" {
                if section == Section::SpecialAnnouncement {
                    let ansage = SpecialAnnouncement {
                        event_id: pre_event_id,
                        text: pre_text,
                        content_id: ContentId {
                            user_id: pre_content_user_id,
                            sub_id: pre_content_sub_id,
                        },
                        code: pre_code,
                        target: pre_target,
                    };

                    res.push(ansage);
                }

                pre_event_id = 0;
                pre_text = "".to_string();
                pre_content_user_id = user_id.unwrap_or_default();
                pre_content_sub_id = 0;
                pre_code = 0;
                pre_target = 0;

                section = Section::SpecialAnnouncement;
                continue;
            } else if line == "[SpecialAnnouncementDefaults]" {
                section = Section::SpecialAnnouncementDefaults;
                continue;
            }

            let prop_opt = Property::parse(line);

            // Properties
            if let Some(prop) = prop_opt {
                match section {
                    Section::SpecialAnnouncement => match prop.name.as_str() {
                        "EventID" => pre_event_id = prop.value.parse().unwrap_or_default(),
                        "Text" => pre_text = prop.value.clone(),
                        "ContentUserID" => {
                            pre_content_user_id = prop.value.parse().unwrap_or_default()
                        }
                        "ContentSubID" => {
                            pre_content_sub_id = prop.value.parse().unwrap_or_default()
                        }
                        "Code" => pre_code = prop.value.parse().unwrap_or_default(),
                        "Target" => pre_target = prop.value.parse().unwrap_or_default(),
                        _ => {}
                    },
                    Section::SpecialAnnouncementDefaults => {
                        if prop.name == "ContentUserID" {
                            user_id = prop.value.parse::<i32>().ok();
                        }
                    }
                    Section::None => {}
                }
            }
        }

        if section == Section::SpecialAnnouncement {
            let ansage = SpecialAnnouncement {
                event_id: pre_event_id,
                text: pre_text,
                content_id: ContentId {
                    user_id: pre_content_user_id,
                    sub_id: pre_content_sub_id,
                },
                code: pre_code,
                target: pre_target,
            };

            res.push(ansage);
        }
    }

    res
}
