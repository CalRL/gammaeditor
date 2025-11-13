use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Stats {
    CurrentHp,
    MaxHp,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED,
}

impl Stats {
    pub fn iter() -> impl Iterator<Item = Stats> {
        [
            Stats::CurrentHp,
            Stats::MaxHp,
            Stats::ATK,
            Stats::DEF,
            Stats::SATK,
            Stats::SDEF,
            Stats::SPEED,
        ]
        .into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Stats::CurrentHp => "CurrentHP",
            Stats::MaxHp => "MaxHP",
            Stats::ATK => "ATK",
            Stats::DEF => "DEF",
            Stats::SATK => "SATK",
            Stats::SDEF => "SDEF",
            Stats::SPEED => "SPEED",
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatStruct {
    pub values: HashMap<Stats, f64>,
}

impl FromStr for Stats {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stat: Self = match s {
            "CurrentHP" => Stats::CurrentHp,
            "MaxHP" => Stats::MaxHp,
            "ATK" => Stats::ATK,
            "DEF" => Stats::DEF,
            "SATK" => Stats::SATK,
            "SDEF" => Stats::SDEF,
            "SPEED" => Stats::SPEED,
            _ => return Err(()),
        };
        Ok(stat)
    }
}

#[derive(Clone, Debug)]
pub enum IVs {
    HP,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED,
}

impl IVs {
    pub fn as_str(&self) -> &str {
        match self {
            IVs::HP => "HP",
            IVs::ATK => "ATK",
            IVs::DEF => "DEF",
            IVs::SATK => "SATK",
            IVs::SDEF => "SDEF",
            IVs::SPEED => "SPEED",
        }
    }
    pub fn iter() -> impl Iterator<Item = IVs> {
        [
            IVs::HP,
            IVs::ATK,
            IVs::DEF,
            IVs::SATK,
            IVs::SDEF,
            IVs::SPEED,
        ]
        .into_iter()
    }

    pub fn from_stat(stat: Stats) -> Option<Self> {
        match stat {
            Stats::MaxHp => Some(IVs::HP),
            Stats::ATK => Some(IVs::ATK),
            Stats::DEF => Some(IVs::DEF),
            Stats::SATK => Some(IVs::SATK),
            Stats::SDEF => Some(IVs::SDEF),
            Stats::SPEED => Some(IVs::SPEED),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IVSpread {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub satk: i32,
    pub sdef: i32,
    pub speed: i32,
}

impl IVs {
    pub fn get_index(self) -> usize {
        match self {
            IVs::HP => 0,
            IVs::ATK => 1,
            IVs::DEF => 2,
            IVs::SATK => 3,
            IVs::SDEF => 4,
            IVs::SPEED => 5,
        }
    }
}

impl FromStr for IVs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iv: IVs = match s {
            "HP" => IVs::HP,
            "ATK" => IVs::ATK,
            "DEF" => IVs::DEF,
            "SATK" => IVs::SATK,
            "SDEF" => IVs::SDEF,
            "SPEED" => IVs::SPEED,
            _ => return Err(()),
        };
        Ok(iv)
    }
}
