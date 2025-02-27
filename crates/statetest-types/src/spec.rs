use std::fmt;

use revm::specification::hardfork::SpecId;
use serde::Deserialize;

/// Ethereum specification names
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash)]
pub enum SpecName {
    Frontier,
    FrontierToHomesteadAt5,
    Homestead,
    HomesteadToDaoAt5,
    HomesteadToEIP150At5,
    EIP150,
    EIP158, // EIP-161: State trie clearing
    EIP158ToByzantiumAt5,
    Byzantium,
    ByzantiumToConstantinopleAt5, // SKIPPED
    ByzantiumToConstantinopleFixAt5,
    Constantinople, // SKIPPED
    ConstantinopleFix,
    Istanbul,
    Berlin,
    BerlinToLondonAt5,
    London,
    Paris,
    Merge,
    Shanghai,
    Cancun,
    Prague,
    Osaka, // SKIPPED
    #[serde(other)]
    Unknown,
}

impl fmt::Display for SpecName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SpecName::Frontier => "Frontier",
            SpecName::FrontierToHomesteadAt5 => "FrontierToHomesteadAt5",
            SpecName::Homestead => "Homestead",
            SpecName::HomesteadToDaoAt5 => "HomesteadToDaoAt5",
            SpecName::HomesteadToEIP150At5 => "HomesteadToEIP150At5",
            SpecName::EIP150 => "EIP150",
            SpecName::EIP158 => "EIP158",
            SpecName::EIP158ToByzantiumAt5 => "EIP158ToByzantiumAt5",
            SpecName::Byzantium => "Byzantium",
            SpecName::ByzantiumToConstantinopleAt5 => "ByzantiumToConstantinopleAt5",
            SpecName::ByzantiumToConstantinopleFixAt5 => "ByzantiumToConstantinopleFixAt5",
            SpecName::Constantinople => "Constantinople",
            SpecName::ConstantinopleFix => "ConstantinopleFix",
            SpecName::Istanbul => "Istanbul",
            SpecName::Berlin => "Berlin",
            SpecName::BerlinToLondonAt5 => "BerlinToLondonAt5",
            SpecName::London => "London",
            SpecName::Paris => "Paris",
            SpecName::Merge => "Merge",
            SpecName::Shanghai => "Shanghai",
            SpecName::Cancun => "Cancun",
            SpecName::Prague => "Prague",
            SpecName::Osaka => "Osaka",
            SpecName::Unknown => "Unknown",
        };
        write!(f, "{}", name)
    }
}

impl SpecName {
    /// Converts to a [SpecId].
    pub fn to_spec_id(&self) -> SpecId {
        match self {
            Self::Frontier => SpecId::FRONTIER,
            Self::Homestead | Self::FrontierToHomesteadAt5 => SpecId::HOMESTEAD,
            Self::EIP150 | Self::HomesteadToDaoAt5 | Self::HomesteadToEIP150At5 => {
                SpecId::TANGERINE
            }
            Self::EIP158 => SpecId::SPURIOUS_DRAGON,
            Self::Byzantium | Self::EIP158ToByzantiumAt5 => SpecId::BYZANTIUM,
            Self::ConstantinopleFix | Self::ByzantiumToConstantinopleFixAt5 => SpecId::PETERSBURG,
            Self::Istanbul => SpecId::ISTANBUL,
            Self::Berlin => SpecId::BERLIN,
            Self::London | Self::BerlinToLondonAt5 => SpecId::LONDON,
            Self::Paris | Self::Merge => SpecId::MERGE,
            Self::Shanghai => SpecId::SHANGHAI,
            Self::Cancun => SpecId::CANCUN,
            Self::Prague => SpecId::PRAGUE,
            Self::Osaka => SpecId::OSAKA,
            Self::ByzantiumToConstantinopleAt5 | Self::Constantinople => {
                panic!("Overridden with PETERSBURG")
            }
            Self::Unknown => panic!("Unknown spec"),
        }
    }

    pub fn sup_network(&self) -> String {
        ">=".to_string() + &self.to_string()
    }
}
