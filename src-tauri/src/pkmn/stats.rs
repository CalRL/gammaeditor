use std::str::FromStr;

pub enum Stats {
    CurrentHp,
    MaxHp,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED
}

impl Stats {
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

pub enum IVs {
    HP,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED
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
