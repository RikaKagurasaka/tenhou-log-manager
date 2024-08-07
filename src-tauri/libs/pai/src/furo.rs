use std::ops::Not;
use strum_macros::EnumIs;
use crate::Pai;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(EnumIs)]
pub enum FuroType {
    #[default]
    Unknown,
    Chi,
    Pon,
    MinKan,
    AnKan,
    Chakan,
}
impl FuroType {
    pub fn is_kan(&self) -> bool {
        match self {
            FuroType::MinKan | FuroType::AnKan | FuroType::Chakan => true,
            _ => false,
        }
    }

    pub fn is_kotsu(&self) -> bool {
        match self {
            FuroType::Pon | FuroType::MinKan | FuroType::AnKan | FuroType::Chakan => true,
            _ => false,
        }
    }

    pub fn is_menzen(&self) -> bool {
        self.is_an_kan()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Furo {
    pub furo_type: FuroType,
    pub target: Pai,
    pub consumed: [Pai; 3],
}
impl Furo {
    pub fn reduced(&self) -> ReducedFuro {
        let minimum = self.consumed.iter().chain(std::iter::once(&self.target)).map(|p| p.remove_aka()).filter(|p| p.is_unknown().not()).min().unwrap();
        ReducedFuro {
            furo_type: match self.furo_type {
                FuroType::Unknown => ReducedFuroType::Unknown,
                FuroType::Chi => ReducedFuroType::Chi,
                FuroType::Pon => ReducedFuroType::Pon,
                FuroType::MinKan | FuroType::Chakan => ReducedFuroType::MinKan,
                FuroType::AnKan => ReducedFuroType::Ankan,
            },
            minimum,
        }
    }

    pub fn pais(&self) -> Vec<Pai> {
        [self.target, self.consumed[0], self.consumed[1], self.consumed[2]].iter().filter(|&p| !p.is_unknown()).cloned().collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(EnumIs)]
pub enum ReducedFuroType {
    #[default]
    Unknown,
    Chi,
    Pon,
    MinKan,
    Ankan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReducedFuro {
    pub furo_type: ReducedFuroType,
    pub minimum: Pai,
}