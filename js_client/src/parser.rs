use regex::Regex;
use std::collections::HashMap;
use serde::Serialize;
use shared::jump::{DistanceThreshold, JumpMainInfo, JumpRecord, JumpSummary, JumpTier, JumpTypes, PartialSummary, Strafe};

#[derive(Debug, Clone, Serialize)]
pub struct TierConfig(pub f64, pub f64, pub f64, pub f64, pub f64);

impl TierConfig {
    pub fn get_tier(&self, amount: f64) -> Option<JumpTier> {
        if amount >= self.4 {
            Some(JumpTier::WRECKER)
        } else if amount >= self.3 {
            Some(JumpTier::OWNAGE)
        } else if amount >= self.2 {
            Some(JumpTier::GODLIKE)
        } else if amount >= self.1 {
            Some(JumpTier::PERFECT)
        } else if amount >= self.0 {
            Some(JumpTier::IMPRESSIVE)
        } else {
            None
        }
    }
}





enum ParserState {
    WaitJumpStart,
    WaitSummary1 {
        info: JumpMainInfo
    },
    WaitSummary2 {
        info: JumpMainInfo,
        partial: PartialSummary,
    },
    WaitGarbage {
        info: JumpMainInfo,
        summary: JumpSummary,
    },
    WaitStrafes {
        info: JumpMainInfo,
        summary: JumpSummary,
        strafes: Vec<Strafe>,
        remaining: u32,
    },
}

impl Default for ParserState {
    fn default() -> Self {
        Self::WaitJumpStart
    }
}

pub struct JumpParser {
    state: ParserState,
    tiers: HashMap<JumpTypes, TierConfig>,
    user_min: HashMap<JumpTypes, DistanceThreshold>,
    js_always_thresholds: HashMap<JumpTypes, DistanceThreshold>,
    start_re: Regex,
    valid_username: Option<String>,
}

pub enum ParserError {
    ParseError,
    JSAlwaysError,
    InvalidUsernameError,
}
impl JumpParser {
    pub fn new(tiers: HashMap<JumpTypes, TierConfig>, user_min: HashMap<JumpTypes, DistanceThreshold>, js_always_thresholds: HashMap<JumpTypes, DistanceThreshold>) -> Self {


        Self {
            state: ParserState::default(),
            tiers,
            user_min,
            js_always_thresholds,
            start_re: Regex::new(r"^(\d{2}/\d{2} \d{2}:\d{2}:\d{2})\s+(.+?) jumped ([\d\.]+) units with a (.*)$").unwrap(),
            valid_username: None,
        }
    }

    pub fn set_valid_username(&mut self, username: Option<String>) {
        self.valid_username = username;
    }

    fn reset(&mut self) {
        self.state = ParserState::default();
    }

    fn parse_jump_type(jt: &str) -> Result<JumpTypes, ParserError> {
        println!("{}", jt);
        match jt.to_uppercase().as_str() {
            "LONG JUMP" => Ok(JumpTypes::LJ),
            "BUNNYHOP" => Ok(JumpTypes::BH),
            "MULTI BUNNYHOP" => Ok(JumpTypes::MBH),
            "LADDER JUMP" => Ok(JumpTypes::LAJ),
            "LADDERHOP" => Ok(JumpTypes::LAH),
            "WEIRD JUMP" => Ok(JumpTypes::WJ),

            "FALL" => Err(ParserError::JSAlwaysError),
            "INVALID JUMP" => Err(ParserError::JSAlwaysError),
            jt if jt.contains("(INVALID COLLISION)")
                || jt.contains("(JUST NOCLIPPED)")
                || jt.contains("(EXTERNALLY MODIFIED)")
                || jt.contains("(TELEPORTED)") => {
                Err(ParserError::JSAlwaysError)
            }

            _ => Err(ParserError::ParseError),
        }
    }

    pub fn process_line(&mut self, line: &str) -> Result<JumpRecord, ParserError> {
        let line = line.trim();
        if line.is_empty() {
            return Err(ParserError::ParseError);
        }

        let state = std::mem::take(&mut self.state);

        match state {
            ParserState::WaitJumpStart => {
                if let Some(caps) = self.start_re.captures(line) {
                    let steam_username = caps[2].to_string();
                    if let Some(valid_username) = self.valid_username.clone() {
                        if(steam_username != valid_username) { return Err(ParserError::InvalidUsernameError); }
                    }
                    let amount: f64 = caps[3].parse().unwrap_or(0.0);
                    match JumpParser::parse_jump_type(&caps[4]) {
                        Ok(jump_type) => {
                            if let Some(tier) = self.tiers.get(&jump_type).expect("Tiers must be set for each jump type").get_tier(amount) {
                                println!("Step 1 succeed");
                                self.state = ParserState::WaitSummary1 {
                                    info: JumpMainInfo { steam_username, amount, tier, jump_type }
                                };
                            } else {
                                if DistanceThreshold(amount) < self.js_always_thresholds.get(&jump_type).expect("JS always thresholds must be set for each jump type").clone() {
                                    return Err(ParserError::JSAlwaysError)
                                }
                            }
                        }
                        Err(e) => return Err(e)
                    }
                }
                Err(ParserError::ParseError)
            }

            ParserState::WaitSummary1 { info } => {
                if let Some(summary) = JumpSummary::parse_line1(line) {

                    if DistanceThreshold(info.amount) < self.user_min.get(&info.jump_type).expect("User min must be set for each jump type").clone() {
                        return Err(ParserError::ParseError)
                    }

                    println!("Step 2 succeed");
                    self.state = ParserState::WaitSummary2 { info, partial: summary };
                } else {
                    self.reset();
                }
                Err(ParserError::ParseError)
            }

            ParserState::WaitSummary2 { info, partial } => {
                if let Some(summary) = partial.clone().parse_line2(line) {
                    println!("Step 3 succeed");
                    self.state = ParserState::WaitGarbage {
                        info, summary
                    };
                } else {
                    self.reset();
                }
                Err(ParserError::ParseError)
            }

            ParserState::WaitGarbage { info, summary, } => {
                println!("Step 4 succeed");
                let strafes_count = summary.strafes_count.clone();
                self.state = ParserState::WaitStrafes { info, summary, strafes: Vec::new(), remaining: strafes_count };
                Err(ParserError::ParseError)
            }

            ParserState::WaitStrafes { info, summary, mut strafes, mut remaining } => {
                if let Some(strafe) = Strafe::parse(line) {
                    println!("Step 5 succeed");
                    strafes.push(strafe);

                    if remaining == 1 {
                        let finished_record = JumpRecord {
                            info: info.clone(),
                            summary: summary.clone(),
                            strafes: strafes.clone(),
                        };
                        self.reset();

                        Ok(finished_record)
                    } else {
                        remaining-=1;
                        self.state = ParserState::WaitStrafes { info, summary, strafes, remaining };
                        Err(ParserError::ParseError)
                    }
                } else {
                    self.reset();
                    Err(ParserError::ParseError)
                }
            }
        }
    }
}