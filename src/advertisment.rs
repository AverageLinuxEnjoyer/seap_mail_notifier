use self::{
    assigned_user::AssignedUser, cpv_code::CpvCode, currency::Currency,
    sys_advertising_notice::SysAdvertisingNotice, sys_notice_state::SysNoticeState,
};
use crate::subscription::Subscription;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub(crate) mod adv_type;
pub(crate) mod assigned_user;
pub(crate) mod contract_type;
pub(crate) mod cpv_code;
pub(crate) mod currency;
pub(crate) mod sys_advertising_notice;
pub(crate) mod sys_notice_state;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertisment {
    pub value: f32,
    pub contract_authority_name: String,
    pub advertising_notice_id: u32,
    pub contracting_authority_id: u32,
    pub assigned_user: AssignedUser,
    pub notice_no: String,
    pub cpv_code: CpvCode,

    pub estimated_value: Option<f32>,
    pub max_estimated_value: Option<f32>,
    pub min_estimated_value: Option<f32>,
    pub currency_id: u32,

    pub notice_entity_address_id: u32,
    pub contract_object: String,
    pub contract_description: String,
    pub contract_related_conditions: Option<String>,
    pub award_criteria: Option<String>,
    pub parent_advertising_notice_id: u32,
    // document ID, URL, Name, UniqueIdentificationCode?
    pub sys_notice_state_id: u32,
    pub sys_notice_state: SysNoticeState,

    pub sys_advertising_notice_id: u32,
    pub sys_advertising_notice: SysAdvertisingNotice,

    #[serde(with = "time::serde::rfc3339")]
    pub publication_date: OffsetDateTime,

    pub currency: Currency,
    pub participation_conditions: Option<String>,
    pub additional_information: Option<String>,
    pub cpv_code_and_name: String,

    #[serde(with = "time::serde::rfc3339")]
    pub create_date: OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub tender_receipt_deadline: OffsetDateTime,

    #[serde(skip)]
    pub link: String,
}

impl Advertisment {
    pub fn matches_subscription(&mut self, sub: &Subscription) -> bool {
        let title_match = if let Some(title_keywords) = sub.title_keywords.clone() {
            text_matches_keywords(&self.contract_object, title_keywords)
        } else {
            true
        };

        let desc_match = if let Some(desc_keywords) = sub.desc_keywords.clone() {
            text_matches_keywords(&self.contract_description, desc_keywords)
        } else {
            true
        };

        let additional_info_match =
            if let Some(additional_info_keywords) = sub.additional_info_keywords.clone() {
                text_matches_keywords(
                    &self.additional_information.clone().unwrap_or_default(),
                    additional_info_keywords,
                )
            } else {
                true
            };

        let min_value_match = if let Some(min_price) = sub.min_price {
            self.estimated_value
                .unwrap_or_else(|| self.min_estimated_value.unwrap_or(i32::MAX as f32))
                >= min_price as f32
        } else {
            true
        };

        let max_value_match = if let Some(max_price) = sub.max_price {
            self.estimated_value
                .unwrap_or_else(|| self.max_estimated_value.unwrap_or(i32::MIN as f32))
                <= max_price as f32
        } else {
            true
        };

        title_match && desc_match && additional_info_match && min_value_match && max_value_match
    }
}

fn text_matches_keywords(text: &String, keywords: Vec<String>) -> bool {
    text.split(' ')
        .map(|word| word.trim_matches(|c| vec![' ', ',', '.'].contains(&c)))
        .map(|word| word.to_lowercase())
        .any(|word| {
            keywords
                .iter()
                .map(|kw| kw.trim_matches(|c| vec![' ', ',', '.'].contains(&c)))
                .map(|kw| kw.to_lowercase())
                .any(|kw| word == kw)
        })
}
