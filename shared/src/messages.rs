use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumString, IntoStaticStr};
use crate::jump::{BlockThreshold, DistanceThreshold, JumpTier, JumpTypes, StatMode, Threshold};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResponse {
    pub thresholds: HashMap<JumpTypes, (DistanceThreshold, BlockThreshold)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitJumpResponse {
    pub new_threshold: (DistanceThreshold, BlockThreshold),
    pub valid_username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitJumpRequest<T> {
    pub user_token: String,
    pub steam_username: String,
    pub record: T,
    pub tier: JumpTier,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub block: Option<i16>,
    pub is_js_always: bool,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "TopRecord.ts"))]
pub struct TopRecord {
    pub jump_id: i32,
    pub discord_id: Option<String>,
    pub steam_id: String,
    pub amount: f64,
    pub block: Option<i16>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "TopRecordResponse.ts"))]
pub struct  TopRecordResponse(pub Vec<TopRecord>);

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "DailyRecord.ts"))]
pub struct DailyRecord {
    pub jump_id: i32,
    pub discord_id: Option<String>,
    pub steam_id: String,
    pub amount: f64,
    pub block: Option<i16>,
    pub created_at: i64, // Для вывода времени рекорда
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "DailyRecordResponse.ts"))]
pub struct  DailyRecordResponse(pub Option<DailyRecord>);

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "DiscordUserTopRecord.ts"))]
pub struct DiscordUserTopRecord<T> {
    pub jump_id: i32,
    pub amount: f64,
    pub block: Option<i16>,
    pub steam_id: String,
    pub created_at: i64,
    pub record: T,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "DiscordUserTopRecordResponse.ts"))]
pub struct DiscordUserTopRecordResponse<T>(pub Vec<DiscordUserTopRecord<T>>);


#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "ServerRecordNotification.ts"))]
pub struct ServerRecordNotification {
    pub jump_id: i32,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub block: Option<i16>,
    pub discord_id: Option<String>,
    pub previous_amount: Threshold,
    pub tier: JumpTier,
    pub stat_mode: StatMode,
    pub steam_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "PersonalBestNotification.ts"))]
pub struct PersonalBestNotification {
    pub jump_id: i32,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub block: Option<i16>,
    pub discord_id: String,
    pub previous_amount: Threshold,
    pub new_min_value: Threshold,
    pub tier: JumpTier,
    pub stat_mode: StatMode,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "PersonalTopNotification.ts"))]
pub struct PersonalTopNotification {
    pub jump_id: i32,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub discord_id: String,
    pub new_min_value: DistanceThreshold,
    pub tier: JumpTier,
}


#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "JumpDetails.ts"))]
pub struct JumpDetails<T> {
    pub record: T,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "BotSessionRequest.ts"))]
pub struct BotSessionRequest {
    pub discord_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "BotSessionResponse.ts"))]
pub struct BotSessionResponse {
    pub auth_url: String,
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, IntoStaticStr, EnumString, Display)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(repr(enum = name), export_to = "StatMode.ts"))]
pub enum AuthType {
    PORT,
    SESSION
}