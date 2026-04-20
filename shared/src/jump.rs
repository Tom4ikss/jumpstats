use std::slice::Iter;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "DistanceThreshold.ts"))]
pub struct DistanceThreshold(pub f64);

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "BlockThreshold.ts"))]
pub struct BlockThreshold(pub i16);

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "Threshold.ts"))]
pub enum Threshold {
    DISTANCE(DistanceThreshold),
    BLOCK(BlockThreshold),
}


#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, IntoStaticStr, EnumString)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(repr(enum = name), export_to = "StatMode.ts"))]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, IntoStaticStr, EnumString)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(repr(enum = name), export_to = "JumpTypes.ts"))]
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

#[derive(Debug, Clone, Deserialize, Serialize, IntoStaticStr, EnumString)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(repr(enum = name), export_to = "JumpTiers.ts"))]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "Crouched.ts"))]
pub struct Crouched(pub f64, pub f64);


#[derive(Debug, Clone, Deserialize, Serialize, IntoStaticStr, EnumString)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(repr(enum = name), export_to = "JumpDirection.ts"))]
pub enum JumpDirection {
    FORWARDS,
    BACKWARDS,
    SIDEWAYS,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "PartialSummary.ts"))]
pub struct PartialSummary {
    pub mode: String,
    pub block: Option<i16>,
    //pub edge: Option<f64>,
    pub strafes_count: u32,
    pub sync: f64,
    pub pre_speed: f64,
    pub max_speed: f64,
    pub bad_angles: f64,
    pub overlap: f64,
    pub dead_air: f64,
    pub jump_direction: JumpDirection,
    //pub height: f64,
    //pub w_timing: Option<String>,
}


impl PartialSummary {
    /// Шаг 2: Принимает вторую строку и превращает PartialSummary в полноценный JumpSummary
    pub fn parse_line2(self, line: &str) -> Option<JumpSummary> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() < 8 {
            println!("parts len < 8 on line: {:#?}", line);
            return None
        }


        //CKZ | 9 Strafes | 67.68% AvgSync | 267.08 Pre | 331.83 Max | 30.30% AvgBadAngles | 1.01% AvgOverlap | 0.00% AvgDeadAir | JumpDirection: Forwards
        //20.61° Deviation | 1.013 Airpath | 44.33% AvgGainEff | 0.00 AvgLoss | 27.81° AvgWidth | 0.00 Offset | 0.15/0.15 Crouched | 55.83 Height | -0.50 W

        let index_offset = if let Some(_) = self.block {0} else {1};
        let have_w_timing = if let Some(_) = self.block {
            parts.len() == 10
        } else {
            parts.len() == 9
        };

        Some(JumpSummary {
            mode: self.mode,
            block: self.block,
            strafes_count: self.strafes_count,
            sync: self.sync,
            pre_speed: self.pre_speed,
            max_speed: self.max_speed,
            bad_angles: self.bad_angles,
            overlap: self.overlap,
            dead_air: self.dead_air,
            jump_direction: self.jump_direction,
            deviation: JumpSummary::get_val(parts[0])?.trim_end_matches('°').parse().ok()?,
            airpath: JumpSummary::get_val(parts[1])?.parse().ok()?,
            gain_eff: JumpSummary::get_val(parts[2])?.trim_end_matches('%').parse().ok()?,
            loss: JumpSummary::get_val(parts[3])?.parse().ok()?,
            width: JumpSummary::get_val(parts[4])?.trim_end_matches('°').parse().ok()?,
            offset: JumpSummary::get_val(parts[5])?.parse().ok()?,
            crouched: {
                let crouched_str = JumpSummary::get_val(parts[6])?;
                let c_parts: Vec<&str> = crouched_str.split('/').collect();
                if c_parts.len() != 2 { return None; }
                Crouched(c_parts[0].parse().ok()?, c_parts[1].parse().ok()?)
            },
            height: JumpSummary::get_val(parts[7])?.parse().ok()?,
            edge: if let Some(_) = self.block {Some(JumpSummary::get_val(parts[8])?.parse().ok()?)} else {None},
            w_timing: if have_w_timing {Some(JumpSummary::get_val(parts[9-index_offset])?.to_string())} else {None},
            //airtime: JumpSummary::get_val(parts[4])?.parse().ok()?,


        })
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "JumpSummary.ts"))]
pub struct JumpSummary {
    pub mode: String,
    pub block: Option<i16>,
    pub edge: Option<f64>,
    pub strafes_count: u32,
    pub sync: f64,
    pub pre_speed: f64,
    pub max_speed: f64,
    pub bad_angles: f64,
    pub overlap: f64,
    pub dead_air: f64,
    pub jump_direction: JumpDirection,

    pub gain_eff: f64,
    pub loss: f64,
    pub airpath: f64,
    pub deviation: f64,
    pub width: f64,
    pub height: f64,
    pub offset: f64,
    pub crouched: Crouched,
    pub w_timing: Option<String>,
}

impl JumpSummary {

    pub fn get_val(p: &str) -> Option<&str> {
        let w: Vec<&str> = p.split_whitespace().collect();
        println!("{:?} | {:?}", p, w);
        if w.len() >= 2 { Some(w[w.len() - 2]) } else { None }
    }

    pub fn get_direction(d_str: &str) -> Option<String> {
        let raw_direction = d_str.split(':').last()?.trim();
        Some(raw_direction.to_uppercase())
    }


    pub fn parse_line1(line: &str) -> Option<PartialSummary> {
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() < 9 {
            println!("parts len < 9 on line: {:#?}", line);
            return None
        }

        let index_offset = if parts.len() < 10 {1} else {0};

        println!("{}\n{:#?}", index_offset, line);
        Some(PartialSummary {
            mode: parts[0].split_whitespace().last()?.to_string(),
            block: if index_offset > 0 {None} else {Some(Self::get_val(parts[1])?.parse().ok()?)},
            strafes_count: Self::get_val(parts[2 - index_offset])?.parse().ok()?,
            sync: Self::get_val(parts[3 - index_offset])?.trim_end_matches('%').parse().ok()?,
            pre_speed: Self::get_val(parts[4 - index_offset])?.parse().ok()?,
            max_speed: Self::get_val(parts[5 - index_offset])?.parse().ok()?,
            bad_angles: Self::get_val(parts[6 - index_offset])?.trim_end_matches('%').parse().ok()?,
            overlap: Self::get_val(parts[7 - index_offset])?.trim_end_matches('%').parse().ok()?,
            dead_air: Self::get_val(parts[8 - index_offset])?.trim_end_matches('%').parse().ok()?,
            jump_direction: Self::get_direction(parts[9 - index_offset])?.parse().ok()?,
            //height: Self::get_val(parts[9 - index_offset])?.parse().ok()?,
            //edge:if index_offset > 0 {None} else {Some(Self::get_val(parts[10])?.parse().ok()?)},
            //w_timing: if parts.len() < (11 - (index_offset*2) + 1) {None} else {Some(Self::get_val(parts[11 - (index_offset*2)])?.to_string())},
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "Ratio.ts"))]
pub struct Ratio(pub f64, pub f64, pub f64);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "Strafe.ts"))]
pub struct Strafe {
    pub number: u32,
    pub sync: f64,
    pub gain: f64,
    pub loss: f64,
    pub max_speed: f64,
    pub airtime: f64,
    pub bad_angles: f64,
    pub overlap: f64,
    pub dead_air: f64,
    pub width: f64,
    pub avg_gain: f64,
    pub gain_eff: f64,
    pub ratio: Ratio,
}

impl Strafe {
    /// Parse Strafe string, return Result
    pub fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 19 {
            return None;
        }


        let parse_u32 = |s: &str| -> Option<u32> {
            s.trim_end_matches('.').parse().ok()
        };

        let parse_f64_percent = |s: &str| -> Option<f64> {
            println!("parse_f64_percent {:#?}", s);
            s.trim_end_matches('%').parse().ok()
        };

        let parse_f64 = |s: &str| -> Option<f64> {
            println!("parse_f64 {:#?}", s);
            s.parse().ok()
        };

        // let parse_ratio = |s: &str| -> Option<Ratio> {
        //     let r_parts: Vec<&str> = s.split(" | ").collect();
        //     println!("parse_ratio {:#?}", r_parts);
        //     if r_parts.len() == 3 {
        //         if let (Ok(r1), Ok(r2), Ok(r3)) = (r_parts[0].parse(), r_parts[1].parse(), r_parts[2].parse()) {
        //             return Some(Ratio(r1, r2, r3));
        //         }
        //     }
        //     None
        // };
        //#.     Sync        Gain       Loss      Max        Airtime     BadAngles     Overlap     DeadAir     Width      AvgGain     GainEff     AngRatio(Avg/Med/Max)
        //9.     0.00%       +0.00      -0.00     343.56     3.02%       100.00%       0.00%       0.00%       1.19°       0.00        0.00%       -1.00 | -0.50 | -0.50

        Some(Strafe {
            number: parse_u32(parts[2])?,
            sync: parse_f64_percent(parts[3])?,
            gain: parse_f64(parts[4])?,
            loss: parse_f64(parts[5])?,
            max_speed: parse_f64(parts[6])?,
            airtime: parse_f64_percent(parts[7])?,
            bad_angles: parse_f64_percent(parts[8])?,
            overlap: parse_f64_percent(parts[9])?,
            dead_air: parse_f64_percent(parts[10])?,
            width: parse_f64(parts[11].trim_end_matches('°'))?,
            avg_gain: parse_f64(parts[12])?,
            gain_eff: parse_f64_percent(parts[13])?,
            ratio: Ratio(parse_f64(parts[14])?, parse_f64(parts[16])?, parse_f64(parts[18])?),
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "JumpMainInfo.ts"))]
pub struct JumpMainInfo {
    pub steam_username: String,
    pub amount: f64,
    pub jump_type: JumpTypes,
    pub tier: JumpTier,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "JumpRecord.ts"))]
pub struct JumpRecord {
    pub info: JumpMainInfo,
    pub summary: JumpSummary,
    pub strafes: Vec<Strafe>,
}
