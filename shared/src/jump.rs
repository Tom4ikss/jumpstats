use std::slice::Iter;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, IntoStaticStr};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/DistanceThreshold.ts")]
pub struct DistanceThreshold(pub f64);

#[derive(Serialize, Deserialize, TS, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/BlockThreshold.ts")]
pub struct BlockThreshold(pub i16);

#[derive(Serialize, Deserialize, TS, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/Threshold.ts")]
pub enum Threshold {
    DISTANCE(DistanceThreshold),
    BLOCK(BlockThreshold),
}


#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, TS, IntoStaticStr, EnumString)]
#[ts(repr(enum = name), export_to = "../../../jumpstats_discord_bot/src/types/StatMode.ts")]
pub enum StatMode {
    DISTANCE,
    BLOCKS,
}

impl StatMode {
    pub fn as_str(&self) -> &'static str {
        let _s: &'static str = self.into();
        _s
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS, IntoStaticStr, EnumString)]
#[ts(repr(enum = name), export_to = "../../../jumpstats_discord_bot/src/types/JumpTypes.ts")]
pub enum JumpTypes {
    LJ,
    BH,
    MBH,
    LAJ,
    LAH,
    WJ
}

impl JumpTypes {
    pub fn iterator() -> Iter<'static, JumpTypes> {
        static JUMP_TYPES: [JumpTypes; 6] = [JumpTypes::LJ, JumpTypes::BH, JumpTypes::MBH, JumpTypes::LAJ, JumpTypes::LAH, JumpTypes::WJ];
        JUMP_TYPES.iter()
    }
    pub fn as_str(&self) -> &'static str {
        let _s: &'static str = self.into();
        _s
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS, IntoStaticStr, EnumString)]
#[ts(repr(enum = name), export_to = "../../../jumpstats_discord_bot/src/types/JumpTiers.ts")]
pub enum JumpTier {
    IMPRESSIVE,
    PERFECT,
    GODLIKE,
    OWNAGE,
    WRECKER,
}

impl JumpTier {
    pub fn as_str(&self) -> &'static str {
        let _s: &'static str = self.into();
        _s
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/Crouched.ts")]
pub struct Crouched(pub f64, pub f64);

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/PartialSummary.ts")]
pub struct PartialSummary {
    pub mode: String,
    pub block: Option<i16>,
    pub edge: Option<f64>,
    pub strafes_count: u32,
    pub sync: f64,
    pub pre_speed: f64,
    pub max_speed: f64,
    pub bad_airtime: u8,
    pub overlap: u8,
    pub dead_airtime: u8,
    pub height: f64,
    pub w_timing: Option<String>,
}


impl PartialSummary {
    /// Шаг 2: Принимает вторую строку и превращает PartialSummary в полноценный JumpSummary
    pub fn parse_line2(self, line: &str) -> Option<JumpSummary> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() < 7 {
            println!("parts len < 7 on line: {:#?}", line);
            return None
        }

        Some(JumpSummary {
            mode: self.mode,
            block: self.block,
            edge: self.edge,
            strafes_count: self.strafes_count,
            sync: self.sync,
            pre_speed: self.pre_speed,
            max_speed: self.max_speed,
            bad_airtime: self.bad_airtime,
            overlap: self.overlap,
            dead_airtime: self.dead_airtime,
            height: self.height,
            w_timing: self.w_timing,
            gain_eff: JumpSummary::get_val(parts[0])?.trim_end_matches('%').parse().ok()?,
            airpath: JumpSummary::get_val(parts[1])?.parse().ok()?,
            deviation: JumpSummary::get_val(parts[2])?.parse().ok()?,
            width: JumpSummary::get_val(parts[3])?.parse().ok()?,
            airtime: JumpSummary::get_val(parts[4])?.parse().ok()?,
            offset: JumpSummary::get_val(parts[5])?.parse().ok()?,
            crouched: {
                let crouched_str = JumpSummary::get_val(parts[6])?;
                let c_parts: Vec<&str> = crouched_str.split('/').collect();
                if c_parts.len() != 2 { return None; }
                Crouched(c_parts[0].parse().ok()?, c_parts[1].parse().ok()?)
            },
        })
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/JumpSummary.ts")]
pub struct JumpSummary {
    pub mode: String,
    pub block: Option<i16>,
    pub edge: Option<f64>,
    pub strafes_count: u32,
    pub sync: f64,
    pub pre_speed: f64,
    pub max_speed: f64,
    pub bad_airtime: u8,
    pub overlap: u8,
    pub dead_airtime: u8,
    pub height: f64,
    pub w_timing: Option<String>,

    pub gain_eff: u8,
    pub airpath: f64,
    pub deviation: f64,
    pub width: f64,
    pub airtime: f64,
    pub offset: f64,
    pub crouched: Crouched,
}

impl JumpSummary {

    pub fn get_val(p: &str) -> Option<&str> {
        let w: Vec<&str> = p.split_whitespace().collect();
        println!("{:?} | {:?}", p, w);
        if w.len() >= 2 { Some(w[w.len() - 2]) } else { None }
    }


    pub fn parse_line1(line: &str) -> Option<PartialSummary> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() < 9 {
            println!("parts len < 9 on line: {:#?}", line);
            return None
        }

        let index_offset = if parts.len() < 11 {1} else {0};

        println!("{}\n{:#?}", index_offset, line);
        //CKZ | 252 Block | 9 Strafes | 83.8 Sync | 276.00 Pre | 364.16 Max | 9% BA | 1% OL | 2% DA | 55.83 Height | 0.9 Edge| ✓ W
        Some(PartialSummary {
            mode: parts[0].split_whitespace().last()?.to_string(),
            block: if index_offset > 0 {None} else {Some(Self::get_val(parts[1])?.parse().ok()?)},
            strafes_count: Self::get_val(parts[2 - index_offset])?.parse().ok()?,
            sync: Self::get_val(parts[3 - index_offset])?.parse().ok()?,
            pre_speed: Self::get_val(parts[4 - index_offset])?.parse().ok()?,
            max_speed: Self::get_val(parts[5 - index_offset])?.parse().ok()?,
            bad_airtime: Self::get_val(parts[6 - index_offset])?.trim_end_matches('%').parse().ok()?,
            overlap: Self::get_val(parts[7 - index_offset])?.trim_end_matches('%').parse().ok()?,
            dead_airtime: Self::get_val(parts[8 - index_offset])?.trim_end_matches('%').parse().ok()?,
            height: Self::get_val(parts[9 - index_offset])?.parse().ok()?,
            edge:if index_offset > 0 {None} else {Some(Self::get_val(parts[10])?.parse().ok()?)},
            w_timing: if parts.len() < (11 - (index_offset*2) + 1) {None} else {Some(Self::get_val(parts[11 - (index_offset*2)])?.to_string())},
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/Ratio.ts")]
pub struct Ratio(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/Strafe.ts")]
pub struct Strafe {
    pub number: u32,
    pub sync: u8,
    pub gain: f64,
    pub loss: f64,
    pub max_speed: f64,
    pub airtime: f64,
    pub bad_airtime: u8,
    pub overlap: u8,
    pub dead_airtime: u8,
    pub width: f64,
    pub avg_gain: f64,
    pub gain_eff: u8,
    pub ratio: Ratio,
}

impl Strafe {
    /// Parse Strafe string, return Result
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 14 {
            return None;
        }


        let parse_u32 = |s: &str| -> Option<u32> {
            s.trim_end_matches('.').parse().ok()
        };

        let parse_u8_percent = |s: &str| -> Option<u8> {
            s.trim_end_matches('%').parse().ok()
        };

        let parse_f64 = |s: &str| -> Option<f64> {
            s.parse().ok()
        };

        let parse_ratio = |s: &str| -> Option<Ratio> {
            let r_parts: Vec<&str> = s.split('/').collect();
            if r_parts.len() == 3 {
                if let (Ok(r1), Ok(r2), Ok(r3)) = (r_parts[0].parse(), r_parts[1].parse(), r_parts[2].parse()) {
                    return Some(Ratio(r1, r2, r3));
                }
            }
            None
        };

        Some(Strafe {
            number: parse_u32(parts[2])?,
            sync: parse_u8_percent(parts[3])?,
            gain: parse_f64(parts[4])?,
            loss: parse_f64(parts[5])?,
            max_speed: parse_f64(parts[6])?,
            airtime: parse_f64(parts[7])?,
            bad_airtime: parse_u8_percent(parts[8])?,
            overlap: parse_u8_percent(parts[9])?,
            dead_airtime: parse_u8_percent(parts[10])?,
            width: parse_f64(parts[11])?,
            avg_gain: parse_f64(parts[12])?,
            gain_eff: parse_u8_percent(parts[13])?,
            ratio: parse_ratio(parts[14])?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/JumpMainInfo.ts")]
pub struct JumpMainInfo {
    pub steam_username: String,
    pub amount: f64,
    pub jump_type: JumpTypes,
    pub tier: JumpTier,
}


#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, export_to = "../../../jumpstats_discord_bot/src/types/JumpRecord.ts")]
pub struct JumpRecord {
    pub info: JumpMainInfo,
    pub summary: JumpSummary,
    pub strafes: Vec<Strafe>,
}
