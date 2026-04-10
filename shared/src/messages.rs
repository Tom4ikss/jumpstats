
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use crate::jump::{BlockThreshold, DistanceThreshold, JumpTier, JumpTypes, StatMode, Threshold};



#[derive(Serialize, Deserialize, TS, Debug)]
pub struct InitResponse {
    pub thresholds: HashMap<JumpTypes, (DistanceThreshold, BlockThreshold)>,
}

#[derive(Serialize, Deserialize, TS, Debug)]
pub struct SubmitJumpResponse {
    pub new_threshold: (DistanceThreshold, BlockThreshold),
    pub valid_username: Option<String>,
}

#[derive(Serialize, Deserialize, TS, Debug)]
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

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/TopRecord.ts")]
pub struct TopRecord {
    pub jump_id: i32,
    pub discord_id: Option<String>,
    pub amount: f64,
    pub block: Option<i16>,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/TopRecordResponse.ts")]
pub struct  TopRecordResponse(pub Vec<TopRecord>);

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/DailyRecord.ts")]
pub struct DailyRecord {
    pub jump_id: i32,
    pub discord_id: Option<String>,
    pub amount: f64,
    pub block: Option<i16>,
    pub created_at: i64, // Для вывода времени рекорда
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/DailyRecordResponse.ts")]
pub struct  DailyRecordResponse(pub Option<DailyRecord>);

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/DiscordUserTopRecord.ts")]
pub struct DiscordUserTopRecord<T> {
    pub jump_id: i32,
    pub amount: f64,
    pub block: Option<i16>,
    pub created_at: i64,
    pub record: T,
}

#[derive(Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/DiscordUserTopRecordResponse.ts")]
pub struct DiscordUserTopRecordResponse<T>(pub Vec<DiscordUserTopRecord<T>>);


#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/ServerRecordNotification.ts")]
pub struct ServerRecordNotification {
    pub jump_id: i32,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub block: Option<i16>,
    pub discord_id: Option<String>,
    pub previous_amount: Threshold,
    pub tier: JumpTier,
    pub stat_mode: StatMode,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/PersonalBestNotification.ts")]
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

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/PersonalTopNotification.ts")]
pub struct PersonalTopNotification {
    pub jump_id: i32,
    pub jump_type: JumpTypes,
    pub amount: f64,
    pub discord_id: String,
    pub new_min_value: DistanceThreshold,
    pub tier: JumpTier,
}


#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/JumpDetails.ts")]
pub struct JumpDetails<T> {
    pub record: T,
    pub created_at: i64,
}