#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use std::fmt::{Display, Formatter};

use crate::errorkey::ErrorKey;
use crate::errors::{warn, warn_info};
use crate::everything::Everything;
use crate::token::Token;

bitflags! {
    /// LAST UPDATED VERSION 1.9.0.2
    /// See `event_scopes.log` from the game data dumps.
    /// Keep in sync with the module constants below.
    pub struct Scopes: u64 {
        const None = 0x0000_0001;
        const Value = 0x0000_0002;
        const Bool = 0x0000_0004;
        const Flag = 0x0000_0008;
        const Character = 0x0000_0010;
        const LandedTitle = 0x0000_0020;
        const Activity = 0x0000_0040;
        const Secret = 0x0000_0080;
        const Province = 0x0000_0100;
        const Scheme = 0x0000_0200;
        const Combat = 0x0000_0400;
        const CombatSide = 0x0000_0800;
        const TitleAndVassalChange = 0x0000_1000;
        const Faith = 0x0000_2000;
        const GreatHolyWar = 0x0000_4000;
        const Religion = 0x0000_8000;
        const War = 0x0001_0000;
        const StoryCycle = 0x0002_0000;
        const CasusBelli = 0x0004_0000;
        const Dynasty = 0x0008_0000;
        const DynastyHouse = 0x0010_0000;
        const Faction = 0x0020_0000;
        const Culture = 0x0040_0000;
        const Army = 0x0080_0000;
        const HolyOrder = 0x0100_0000;
        const CouncilTask = 0x0200_0000;
        const MercenaryCompany = 0x0400_0000;
        const Artifact = 0x0800_0000;
        const Inspiration = 0x1000_0000;
        const Struggle = 0x2000_0000;
        const CharacterMemory = 0x4000_0000;
        const TravelPlan = 0x8000_0000;
        const Accolade = 0x0000_0001_0000_0000;

        const AccoladeType = 0x0000_0002_0000_0000;
        const Decision = 0x0000_0004_0000_0000;
        const Doctrine = 0x0000_0008_0000_0000;
        const ActivityType = 0x0000_0010_0000_0000;
        const CultureTradition = 0x0000_0020_0000_0000;
        const CulturePillar = 0x0000_0040_0000_0000;
        const GovernmentType = 0x0000_0080_0000_0000;
        const Trait = 0x0000_0100_0000_0000;
        const VassalContract = 0x0000_0200_0000_0000;
        const VassalContractObligationLevel = 0x0000_0400_0000_0000;
    }
}

impl Scopes {
    pub fn expect_scope(&mut self, key: &Token, expect: Scopes) {
        if self.intersects(expect) {
            *self &= expect;
        } else {
            let msg = format!("{key} is for {expect} but scope seems to be {self}");
            warn(key, ErrorKey::Scopes, &msg);
        }
    }

    pub fn non_primitive() -> Scopes {
        Scopes::all() ^ (Scopes::None | Scopes::Value | Scopes::Bool | Scopes::Flag)
    }

    pub fn all_but_none() -> Scopes {
        Scopes::all() ^ Scopes::None
    }
}

/// LAST UPDATED VERSION 1.9.0.2
/// See `event_scopes.log` from the game data dumps.
pub const None: u64 = 0x0000_0001;
pub const Value: u64 = 0x0000_0002;
pub const Bool: u64 = 0x0000_0004;
pub const Flag: u64 = 0x0000_0008;
pub const Character: u64 = 0x0000_0010;
pub const LandedTitle: u64 = 0x0000_0020;
pub const Activity: u64 = 0x0000_0040;
pub const Secret: u64 = 0x0000_0080;
pub const Province: u64 = 0x0000_0100;
pub const Scheme: u64 = 0x0000_0200;
pub const Combat: u64 = 0x0000_0400;
pub const CombatSide: u64 = 0x0000_0800;
pub const TitleAndVassalChange: u64 = 0x0000_1000;
pub const Faith: u64 = 0x0000_2000;
pub const GreatHolyWar: u64 = 0x0000_4000;
pub const Religion: u64 = 0x0000_8000;
pub const War: u64 = 0x0001_0000;
pub const StoryCycle: u64 = 0x0002_0000;
pub const CasusBelli: u64 = 0x0004_0000;
pub const Dynasty: u64 = 0x0008_0000;
pub const DynastyHouse: u64 = 0x0010_0000;
pub const Faction: u64 = 0x0020_0000;
pub const Culture: u64 = 0x0040_0000;
pub const Army: u64 = 0x0080_0000;
pub const HolyOrder: u64 = 0x0100_0000;
pub const CouncilTask: u64 = 0x0200_0000;
pub const MercenaryCompany: u64 = 0x0400_0000;
pub const Artifact: u64 = 0x0800_0000;
pub const Inspiration: u64 = 0x1000_0000;
pub const Struggle: u64 = 0x2000_0000;
pub const CharacterMemory: u64 = 0x4000_0000;
pub const TravelPlan: u64 = 0x8000_0000;
pub const Accolade: u64 = 0x0000_0001_0000_0000;
pub const AccoladeType: u64 = 0x0000_0002_0000_0000;
pub const Decision: u64 = 0x0000_0004_0000_0000;
pub const Doctrine: u64 = 0x0000_0008_0000_0000;
pub const ActivityType: u64 = 0x0000_0010_0000_0000;
pub const CultureTradition: u64 = 0x0000_0020_0000_0000;
pub const CulturePillar: u64 = 0x0000_0040_0000_0000;
pub const GovernmentType: u64 = 0x0000_0080_0000_0000;
pub const Trait: u64 = 0x0000_0100_0000_0000;
pub const VassalContract: u64 = 0x0000_0200_0000_0000;
pub const VassalContractObligationLevel: u64 = 0x0000_0400_0000_0000;
pub const ALL: u64 = 0x7fff_ffff_ffff_ffff;
pub const ALL_BUT_NONE: u64 = 0x7fff_ffff_ffff_fffe;

pub fn scope_from_snake_case(s: &str) -> Option<Scopes> {
    Some(match s {
        "none" => Scopes::None,
        "value" => Scopes::Value,
        "bool" => Scopes::Bool,
        "flag" => Scopes::Flag,
        "character" => Scopes::Character,
        "landed_title" => Scopes::LandedTitle,
        "activity" => Scopes::Activity,
        "secret" => Scopes::Secret,
        "province" => Scopes::Province,
        "scheme" => Scopes::Scheme,
        "combat" => Scopes::Combat,
        "combat_side" => Scopes::CombatSide,
        "title_and_vassal_change" => Scopes::TitleAndVassalChange,
        "faith" => Scopes::Faith,
        "ghw" => Scopes::GreatHolyWar, // Warning, this is an exception to the general rule
        "religion" => Scopes::Religion,
        "war" => Scopes::War,
        "story_cycle" => Scopes::StoryCycle,
        "casus_belli" => Scopes::CasusBelli,
        "dynasty" => Scopes::Dynasty,
        "dynasty_house" => Scopes::DynastyHouse,
        "faction" => Scopes::Faction,
        "culture" => Scopes::Culture,
        "army" => Scopes::Army,
        "holy_order" => Scopes::HolyOrder,
        "council_task" => Scopes::CouncilTask,
        "mercenary_company" => Scopes::MercenaryCompany,
        "artifact" => Scopes::Artifact,
        "inspiration" => Scopes::Inspiration,
        "struggle" => Scopes::Struggle,
        "character_memory" => Scopes::CharacterMemory,
        "travel_plan" => Scopes::TravelPlan,
        "accolade" => Scopes::Accolade,
        "accolade_type" => Scopes::AccoladeType,
        "decision" => Scopes::Decision,
        "doctrine" => Scopes::Doctrine,
        "activity_type" => Scopes::ActivityType,
        "culture_tradition" => Scopes::CultureTradition,
        "culture_pillar" => Scopes::CulturePillar,
        "government_type" => Scopes::GovernmentType,
        "trait" => Scopes::Trait,
        "vassal_contract" => Scopes::VassalContract,
        "vassal_contract_obligation_level" => Scopes::VassalContractObligationLevel,
        _ => return std::option::Option::None,
    })
}

pub fn scope_to_scope(name: &Token) -> Option<(Scopes, Scopes)> {
    for (from, s, to) in SCOPE_TO_SCOPE {
        if name.is(s) {
            return Some((
                Scopes::from_bits_truncate(*from),
                Scopes::from_bits_truncate(*to),
            ));
        }
    }
    for (s, version, explanation) in SCOPE_TO_SCOPE_REMOVED {
        if name.is(s) {
            let msg = format!("`{name}` was removed in {version}");
            warn_info(name, ErrorKey::Removed, &msg, explanation);
            return Some((Scopes::all(), Scopes::all_but_none()));
        }
    }
    std::option::Option::None
}

pub fn scope_prefix(prefix: &str) -> Option<(Scopes, Scopes)> {
    for (from, s, to) in SCOPE_FROM_PREFIX {
        if *s == prefix {
            return Some((
                Scopes::from_bits_truncate(*from),
                Scopes::from_bits_truncate(*to),
            ));
        }
    }
    std::option::Option::None
}

pub fn scope_value(name: &Token, data: &Everything) -> Option<Scopes> {
    for (from, s) in SCOPE_VALUE {
        if name.is(s) {
            return Some(Scopes::from_bits_truncate(*from));
        }
    }
    if let Some(relation) = name.as_str().strip_prefix("num_of_relation_") {
        if data.relations.exists(relation) {
            return Some(Scopes::Character);
        }
    } else if let Some(lifestyle) = name.as_str().strip_prefix("perks_in_") {
        if data.lifestyles.exists(lifestyle) {
            return Some(Scopes::Character);
        }
    } else if let Some(lifestyle) = name.as_str().strip_suffix("_perk_points") {
        if data.lifestyles.exists(lifestyle) {
            return Some(Scopes::Character);
        }
    } else if let Some(lifestyle) = name.as_str().strip_suffix("_perks") {
        if data.lifestyles.exists(lifestyle) {
            return Some(Scopes::Character);
        }
    } else if let Some(lifestyle) = name.as_str().strip_suffix("_unlockable_perks") {
        if data.lifestyles.exists(lifestyle) {
            return Some(Scopes::Character);
        }
    } else if let Some(lifestyle) = name.as_str().strip_suffix("_xp") {
        if data.lifestyles.exists(lifestyle) {
            return Some(Scopes::Character);
        }
    }
    std::option::Option::None
}

/// `name` is without the `every_`, `ordered_`, `random_`, or `any_`
pub fn scope_iterator(name: &Token, data: &Everything) -> Option<(Scopes, Scopes)> {
    for (from, s, to) in SCOPE_ITERATOR {
        if name.is(s) {
            return Some((
                Scopes::from_bits_truncate(*from),
                Scopes::from_bits_truncate(*to),
            ));
        }
    }
    for (s, version, explanation) in SCOPE_REMOVED_ITERATOR {
        if name.is(s) {
            let msg = format!("`{name}` iterators were removed in {version}");
            warn_info(name, ErrorKey::Removed, &msg, explanation);
            return Some((Scopes::all(), Scopes::all()));
        }
    }
    if data.scripted_lists.exists(name.as_str()) {
        return data
            .scripted_lists
            .base(name)
            .and_then(|name| scope_iterator(name, data));
    }
    std::option::Option::None
}

impl Display for Scopes {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        if *self == Scopes::all() {
            write!(f, "any scope")
        } else if *self == Scopes::non_primitive() {
            write!(f, "non-primitive scope")
        } else if *self == Scopes::all_but_none() {
            write!(f, "any except none scope")
        } else {
            let mut vec = Vec::new();
            if self.contains(Scopes::None) {
                vec.push("none");
            }
            if self.contains(Scopes::Value) {
                vec.push("value");
            }
            if self.contains(Scopes::Bool) {
                vec.push("bool");
            }
            if self.contains(Scopes::Flag) {
                vec.push("flag");
            }
            if self.contains(Scopes::Character) {
                vec.push("character");
            }
            if self.contains(Scopes::LandedTitle) {
                vec.push("landed title");
            }
            if self.contains(Scopes::Activity) {
                vec.push("activity");
            }
            if self.contains(Scopes::Secret) {
                vec.push("secret");
            }
            if self.contains(Scopes::Province) {
                vec.push("province");
            }
            if self.contains(Scopes::Scheme) {
                vec.push("scheme");
            }
            if self.contains(Scopes::Combat) {
                vec.push("combat");
            }
            if self.contains(Scopes::CombatSide) {
                vec.push("combat side");
            }
            if self.contains(Scopes::TitleAndVassalChange) {
                vec.push("title and vassal change");
            }
            if self.contains(Scopes::Faith) {
                vec.push("faith");
            }
            if self.contains(Scopes::GreatHolyWar) {
                vec.push("great holy war");
            }
            if self.contains(Scopes::Religion) {
                vec.push("religion");
            }
            if self.contains(Scopes::War) {
                vec.push("war");
            }
            if self.contains(Scopes::StoryCycle) {
                vec.push("story cycle");
            }
            if self.contains(Scopes::CasusBelli) {
                vec.push("casus belli");
            }
            if self.contains(Scopes::Dynasty) {
                vec.push("dynasty");
            }
            if self.contains(Scopes::DynastyHouse) {
                vec.push("dynasty house");
            }
            if self.contains(Scopes::Faction) {
                vec.push("faction");
            }
            if self.contains(Scopes::Culture) {
                vec.push("culture");
            }
            if self.contains(Scopes::Army) {
                vec.push("army");
            }
            if self.contains(Scopes::HolyOrder) {
                vec.push("holy order");
            }
            if self.contains(Scopes::CouncilTask) {
                vec.push("council task");
            }
            if self.contains(Scopes::MercenaryCompany) {
                vec.push("mercenary company");
            }
            if self.contains(Scopes::Artifact) {
                vec.push("artifact");
            }
            if self.contains(Scopes::Inspiration) {
                vec.push("inspiration");
            }
            if self.contains(Scopes::Struggle) {
                vec.push("struggle");
            }
            if self.contains(Scopes::CharacterMemory) {
                vec.push("character memory");
            }
            if self.contains(Scopes::TravelPlan) {
                vec.push("travel plan");
            }
            if self.contains(Scopes::Accolade) {
                vec.push("accolade");
            }
            if self.contains(Scopes::AccoladeType) {
                vec.push("accolade type");
            }
            if self.contains(Scopes::Decision) {
                vec.push("decision");
            }
            if self.contains(Scopes::Doctrine) {
                vec.push("doctrine");
            }
            if self.contains(Scopes::ActivityType) {
                vec.push("activity type");
            }
            if self.contains(Scopes::CultureTradition) {
                vec.push("culture tradition");
            }
            if self.contains(Scopes::CulturePillar) {
                vec.push("culture pillar");
            }
            if self.contains(Scopes::GovernmentType) {
                vec.push("government type");
            }
            if self.contains(Scopes::Trait) {
                vec.push("trait");
            }
            if self.contains(Scopes::VassalContract) {
                vec.push("vassal contract");
            }
            if self.contains(Scopes::VassalContractObligationLevel) {
                vec.push("vassal contract obligation level");
            }
            for i in 0..vec.len() {
                write!(f, "{}", vec[i])?;
                if i + 1 == vec.len() {
                } else if i + 2 == vec.len() {
                    write!(f, " or ")?;
                } else {
                    write!(f, ", ")?;
                }
            }
            Ok(())
        }
    }
}

/// LAST UPDATED VERSION 1.9.0.2
/// See `event_targets.log` from the game data dumps
/// These are scope transitions that can be chained like `root.joined_faction.faction_leader`
const SCOPE_TO_SCOPE: &[(u64, &str, u64)] = &[
    (Accolade, "acclaimed_knight", Character),
    (Character, "accolade", Accolade),
    (Accolade, "accolade_owner", Character),
    (Accolade, "accolade_successor", Character),
    (Activity, "activity_host", Character),
    (Activity, "activity_location", Province),
    (Activity, "activity_type", ActivityType),
    (Army, "army_commander", Character),
    (Army, "army_owner", Character),
    (Artifact, "artifact_age", Value),
    (Artifact, "artifact_owner", Character),
    (LandedTitle | Province, "barony", LandedTitle),
    (LandedTitle | Province, "barony_controller", Character),
    (Character, "betrothed", Character),
    (Culture, "calc_culture_dominant_faith", Faith),
    (Culture, "calc_culture_dominant_religion", Religion),
    (Character, "capital_barony", LandedTitle),
    (Character, "capital_county", LandedTitle),
    (Character, "capital_province", Province),
    (LandedTitle, "capital_vassal", LandedTitle),
    (War, "casus_belli", CasusBelli),
    (War | CasusBelli, "claimant", Character),
    (CombatSide, "combat", Combat),
    (Combat, "combat_attacker", CombatSide),
    (Combat, "combat_defender", CombatSide),
    (Combat, "combat_war", War),
    (Character, "commanding_army", Army),
    (Value, "compare_value", Value), // special
    (Character, "concubinist", Character),
    (Character, "council_task", CouncilTask), // also has a prefix form
    (CouncilTask, "councillor", Character),
    (Character, "councillor_task_target", ALL), // output scope depends on task
    (LandedTitle | Province, "county", LandedTitle),
    (LandedTitle | Province, "county_controller", Character),
    (Character, "court_owner", Character),
    (Artifact, "creator", Character),
    (Character | LandedTitle | Province, "culture", Culture),
    (Culture, "culture_head", Character),
    (LandedTitle, "current_heir", Character),
    (TravelPlan, "current_location", Province),
    (Character, "current_travel_plan", TravelPlan),
    (LandedTitle, "de_facto_liege", LandedTitle),
    (LandedTitle, "de_jure_liege", LandedTitle),
    (Character, "default_location", Province),
    (TravelPlan, "departure_location", Province),
    (Character, "designated_diarch", Character),
    (Character, "designated_heir", Character),
    (Character, "diarch", Character),
    (Character, "diarchy_successor", Character),
    (LandedTitle | Province, "duchy", LandedTitle),
    (None, "dummy_female", Character),
    (None, "dummy_male", Character),
    (Dynasty, "dynast", Character),
    (Character, "dynasty", Dynasty),
    (LandedTitle | Province, "empire", LandedTitle),
    (Character, "employer", Character),
    (CombatSide, "enemy_side", CombatSide),
    (Faction, "faction_leader", Character),
    (Faction, "faction_target", Character),
    (Faction, "faction_war", War),
    (
        Character | LandedTitle | Province | GreatHolyWar,
        "faith",
        Faith,
    ),
    (Character, "father", Character),
    (TravelPlan, "final_destination_province", Province),
    (Faith, "founder", Character),
    (Accolade, "founder_culture", Culture),
    (Accolade, "founder_dynasty", Dynasty),
    (Accolade, "founder_faith", Faith),
    (Accolade, "founder_house", DynastyHouse),
    (Character, "ghw_beneficiary", Character),
    (GreatHolyWar, "ghw_designated_winner", Character),
    (GreatHolyWar, "ghw_target_character", Character),
    (GreatHolyWar, "ghw_target_title", LandedTitle),
    (GreatHolyWar, "ghw_title_recipient", Character),
    (GreatHolyWar, "ghw_war", War),
    (GreatHolyWar, "ghw_war_declarer", Character),
    (Faith, "great_holy_war", GreatHolyWar),
    (LandedTitle, "holder", Character),
    (HolyOrder, "holy_order_patron", Character),
    (Character, "host", Character),
    (Character, "house", DynastyHouse),
    (DynastyHouse, "house_founder", Character),
    (DynastyHouse, "house_head", Character),
    (Character, "imprisoner", Character),
    (Character, "inspiration", Inspiration),
    (Inspiration, "inspiration_owner", Character),
    (Inspiration, "inspiration_sponsor", Character),
    (Character, "intent_target", Character),
    (Character, "involved_activity", Activity),
    (Character, "joined_faction", Faction),
    (Character, "killer", Character),
    (LandedTitle | Province, "kingdom", LandedTitle),
    (Character, "knight_army", Army),
    (DynastyHouse, "last_house_head", Character),
    (Character, "last_played_character", Character),
    (HolyOrder, "leader", Character),
    (LandedTitle, "lessee", Character),
    (LandedTitle, "lessee_title", LandedTitle),
    (Character, "liege", Character),
    (Character, "liege_or_court_owner", Character),
    (Character | Combat | Army, "location", Province),
    (Character, "matchmaker", Character),
    (MercenaryCompany, "mercenary_company_leader", Character),
    (CharacterMemory, "memory_owner", Character),
    (Character, "mother", Character),
    // named_script_value special
    (TravelPlan, "next_destination_province", Province),
    (TravelPlan, "next_location", Province),
    (None, "no", Bool),
    (Character, "player_heir", Character),
    (Character, "pregnancy_assumed_father", Character),
    (Character, "pregnancy_real_father", Character),
    // "prev" special
    (LandedTitle, "previous_holder", Character),
    (Artifact, "previous_owner", Character),
    (Artifact, "previous_owner_level_2", Character),
    (Artifact, "previous_owner_level_3", Character),
    (War | CasusBelli, "primary_attacker", Character),
    (War | CasusBelli, "primary_defender", Character),
    (Character, "primary_heir", Character),
    (Character, "primary_partner", Character),
    (Character, "primary_spouse", Character),
    (Character, "primary_title", LandedTitle),
    (Accolade, "primary_type", AccoladeType),
    (Province, "province_owner", Character),
    (Character, "real_father", Character),
    (Character, "realm_priest", Character),
    (
        Character | LandedTitle | Province | Faith | GreatHolyWar,
        "religion",
        Religion,
    ),
    (Faith, "religious_head", Character),
    (Faith, "religious_head_title", LandedTitle),
    // "root" special
    (Scheme, "scheme_artifact", Artifact),
    (Scheme, "scheme_defender", Character),
    (Scheme, "scheme_owner", Character),
    (Scheme, "scheme_target", Character),
    (Accolade, "secondary_type", AccoladeType),
    (Secret, "secret_owner", Character),
    (Secret, "secret_target", Character),
    (CombatSide, "side_commander", Character),
    (CombatSide, "side_primary_participant", Character),
    (Faction, "special_character", Character),
    (Faction, "special_title", LandedTitle),
    (StoryCycle, "story_owner", Character),
    // "this" special
    (HolyOrder, "title", LandedTitle),
    (LandedTitle, "title_capital_county", LandedTitle),
    (LandedTitle, "title_province", Province),
    (TravelPlan, "travel_leader", Character),
    (TravelPlan, "travel_plan_activity", Activity),
    (TravelPlan, "travel_plan_owner", Activity),
    (Character, "top_liege", Character),
    // "value" special
    (
        VassalContractObligationLevel,
        "vassal_contract_type",
        VassalContract,
    ),
    (CasusBelli, "war", War),
    (None, "yes", Bool),
];

/// LAST UPDATED VERSION 1.9.0.2
/// See `event_targets.log` from the game data dumps
/// These are absolute scopes (like character:100000) and scope transitions that require
/// a key (like `root.cp:councillor_steward`)
/// TODO: add the Item type here, so that it can be checked for existence.
const SCOPE_FROM_PREFIX: &[(u64, &str, u64)] = &[
    (None, "accolade_type", AccoladeType),
    (None, "activity_type", ActivityType),
    (Character, "aptitude", Value),
    (None, "array_define", Value),
    (None, "character", Character),
    (Value, "compare_complex_value", Value),
    (Character, "council_task", CouncilTask),
    (Character, "court_position", Character),
    (Character, "cp", Character), // councillor
    (None, "culture", Culture),
    (None, "culture_pillar", CulturePillar),
    (None, "culture_tradition", CultureTradition),
    (None, "decision", Decision),
    (None, "define", Value),
    (None, "doctrine", Doctrine),
    (None, "dynasty", Dynasty),
    (None, "event_id", Flag),
    (None, "faith", Faith),
    (None, "flag", Flag),
    (None, "global_var", ALL),
    (None, "government_type", GovernmentType),
    (None, "house", DynastyHouse),
    (None, "local_var", ALL),
    (Character, "mandate_type_qualification", Value),
    (CharacterMemory, "memory_participant", Character),
    (None, "province", Province),
    (None, "religion", Religion),
    (None, "scope", ALL),
    (Activity, "special_guest", Character),
    (None, "struggle", Struggle),
    (None, "title", LandedTitle),
    (None, "trait", Trait),
    (ALL, "var", ALL),
    (None, "vassal_contract", VassalContract),
    (Character, "vassal_contract_obligation_level", Value),
];

/// LAST UPDATED VERSION 1.9.0.2
/// See `triggers.log` from the game data dumps
/// These are 'triggers' that return a value.
const SCOPE_VALUE: &[(u64, &str)] = &[
    (Accolade, "accolade_rank"),
    (LandedTitle, "active_de_jure_drift_progress"),
    (Character, "age"),
    (Character, "ai_boldness"),
    (Character, "ai_compassion"),
    (Character, "ai_energy"),
    (Character, "ai_greed"),
    (Character, "ai_honor"),
    (Character, "ai_rationality"),
    (Character, "ai_sociability"),
    (Character, "ai_vengefulness"),
    (Character, "ai_zeal"),
    (Army, "army_max_size"),
    (Army, "army_size"),
    (Artifact, "artifact_durability"),
    (Artifact, "artifact_max_durability"),
    (War, "attacker_war_score"),
    (Character, "attraction"),
    (Province, "available_loot"),
    (Character, "average_amenity_level"),
    (Faction, "average_faction_opinion"),
    (Faction, "average_faction_opinion_not_powerful_vassal"),
    (Faction, "average_faction_opinion_powerful_vassal"),
    (Inspiration, "base_inspiration_gold_cost"),
    (Character, "base_weight"),
    (LandedTitle | Province, "building_levies"),
    (LandedTitle | Province, "building_max_garrison"),
    (Province, "building_slots"),
    (Province, "combined_building_level"),
    (Character, "council_task_monthly_progress"),
    (LandedTitle, "county_control"),
    (LandedTitle, "county_control_rate"),
    (LandedTitle, "county_control_rate_modifier"),
    (LandedTitle, "county_holder_opinion"),
    (LandedTitle, "county_opinion"),
    (Character, "court_grandeur_base"),
    (Character, "court_grandeur_current"),
    (Character, "court_grandeur_current_level"),
    (Character, "court_grandeur_minimum_expected"),
    (Character, "court_grandeur_minimum_expected_level"),
    (Character, "court_positions_currently_avaiable"),
    (Character, "court_positions_currently_filled"),
    (Culture, "culture_age"),
    (Culture, "culture_number_of_counties"),
    (None, "current_computer_date_day"),
    (None, "current_computer_date_month"),
    (None, "current_computer_date_year"),
    (TravelPlan, "current_danger_value"),
    (None, "current_day"),
    (Character, "current_military_strength"),
    (None, "current_month"),
    (None, "current_tooltip_depth"),
    (Character, "current_weight"),
    (Character, "current_weight_for_portrait"),
    (ALL_BUT_NONE, "current_year"), // should be None scope, but current_year is buggy
    (Character, "days_as_ruler"),
    (Character, "days_in_prison"),
    (Character, "days_of_continuous_peace"),
    (Character, "days_of_continuous_war"),
    (Inspiration, "days_since_creation"),
    (Character, "days_since_death"),
    (Character, "days_since_joined_court"),
    (War, "days_since_max_war_score"),
    (Inspiration, "days_since_sponsorship"),
    (TravelPlan, "days_travelled"),
    (GreatHolyWar, "days_until_ghw_launch"),
    (Character, "debt_level"),
    (War, "defender_war_score"),
    (TravelPlan, "departure_date"),
    (LandedTitle, "development_level"),
    (LandedTitle, "development_rate"),
    (LandedTitle, "development_rate_modifier"),
    (LandedTitle, "development_towards_level_increase"),
    (Character, "diarch_aptitude"),
    (Character, "diarch_loyalty"),
    (Character, "diarchy_swing"),
    (Character, "diplomacy"),
    (Character, "diplomacy_for_portrait"),
    (Faction, "discontent_per_month"),
    (Character, "domain_limit"),
    (Character, "domain_limit_available"),
    (Character, "domain_limit_percentage"),
    (Character, "domain_size"),
    (Character, "domain_size_excluding_grace_period"),
    (Character, "dread"),
    (Dynasty, "dynasty_num_unlocked_perks"),
    (Dynasty, "dynasty_prestige"),
    (Dynasty, "dynasty_prestige_level"),
    (Character, "effective_age"),
    (Faith, "estimated_faith_strength"),
    (Faction, "faction_discontent"),
    (Faction, "faction_power"),
    (Faction, "faction_power_threshold"),
    (Character, "fertility"),
    (Faith, "fervor"),
    (TravelPlan, "final_destination_arrival_date"),
    (TravelPlan, "final_destination_arrival_days"),
    (TravelPlan, "final_destination_progress"),
    (Character, "focus_progress"),
    (Province, "fort_level"),
    (Province, "free_building_slots"),
    (GreatHolyWar, "ghw_attackers_strength"),
    (GreatHolyWar, "ghw_defenders_strength"),
    (GreatHolyWar, "ghw_war_chest_gold"),
    (GreatHolyWar, "ghw_war_chest_piety"),
    (GreatHolyWar, "ghw_war_chest_prestige"),
    (Character, "gold"),
    (Character, "has_had_focus_for_days"),
    (Character, "health"),
    (Character, "highest_held_title_tier"),
    (Faith, "holy_sites_controlled"),
    (Inspiration, "inspiration_gold_invested"),
    (Inspiration, "inspiration_progress"),
    (Character, "intrigue"),
    (Character, "intrigue_for_portrait"),
    (Character, "learning"),
    (Character, "learning_for_portrait"),
    (Character, "long_term_gold"),
    (Character, "long_term_gold_maximum"),
    (Character, "martial"),
    (Character, "martial_for_portrait"),
    (Character, "max_active_accolades"),
    (Character, "max_military_strength"),
    (Character, "max_number_of_concubines"),
    (Character, "max_number_of_knights"),
    (CharacterMemory, "memory_age_years"),
    (CharacterMemory, "memory_creation_date"),
    (CharacterMemory, "memory_end_date"),
    (MercenaryCompany, "mercenary_company_expiration_days"),
    (Character, "missing_unique_ancestors"),
    (Character, "monthly_character_balance"),
    (Character, "monthly_character_expenses"),
    (Character, "monthly_character_income"),
    (Character, "monthly_character_income_long_term"),
    (Character, "monthly_character_income_reserved"),
    (Character, "monthly_character_income_short_term"),
    (Character, "monthly_character_income_war_chest"),
    (Province, "monthly_income"),
    (Character, "months_as_ruler"),
    (Faction, "months_until_max_discontent"),
    (TravelPlan, "next_destination_arrival_date"),
    (TravelPlan, "next_destination_arrival_days"),
    (TravelPlan, "next_destination_arrival_progress"),
    (Character, "num_active_accolades"),
    (Artifact, "num_artifact_kills"),
    (Province, "num_buildings"),
    (Faith, "num_character_followers"),
    (Faith, "num_county_followers"),
    (CombatSide, "num_enemies_killed"),
    (TravelPlan, "num_entourage_characters"),
    (Activity, "num_future_phases"),
    (Character, "num_inactive_accolades"),
    (HolyOrder, "num_leased_titles"),
    (Character, "num_of_bad_genetic_traits"),
    (Character, "num_of_good_genetic_traits"),
    (Character, "num_of_known_languages"),
    // num_of_relation_<relation>
    (TravelPlan, "num_options"),
    (Activity, "num_past_phases"),
    (Activity, "num_phases"),
    (Character, "num_sinful_traits"),
    (Combat, "num_total_troops"),
    (Character, "num_virtuous_traits"),
    (Province, "number_of_characters_in_pool"),
    (Character, "number_of_commander_traits"),
    (Character, "number_of_concubines"),
    (Character, "number_of_desired_concubines"),
    (Faction, "number_of_faction_members_in_council"),
    (Character, "number_of_fertile_concubines"),
    (Character, "number_of_knights"),
    (Character, "number_of_lifestyle_traits"),
    (Character, "number_of_maa_regiments"),
    (Character, "number_of_personality_traits"),
    (Character, "number_of_powerful_vassals"),
    (Character, "number_of_stationed_maa_regiments"),
    (Character, "number_of_traits"),
    (VassalContractObligationLevel, "obligation_level_score"),
    (CombatSide, "percent_enemies_killed"),
    (Character, "perk_points"),
    (Character, "perk_points_assigned"),
    // perks_in_<lifestyle>
    (Character, "piety"),
    (Character, "piety_level"),
    (Character, "pregnancy_days"),
    (Character, "prestige"),
    (Character, "prestige_level"),
    (Accolade, "primary_tier"),
    (Character, "prowess"),
    (Character, "prowess_for_portrait"),
    (Character, "prowess_no_portrait"),
    (Army, "raid_loot"),
    (Character, "ransom_cost"),
    (Character, "realm_size"),
    (Character, "reserved_gold"),
    (Character, "reserved_gold_maximum"),
    (Scheme, "scheme_duration_days"),
    (Scheme, "scheme_monthly_progress"),
    (Scheme, "scheme_number_of_agents"),
    (Scheme, "scheme_number_of_exposed_agents"),
    (Scheme, "scheme_power"),
    (Scheme, "scheme_power_resistance_difference"),
    (Scheme, "scheme_power_resistance_ratio"),
    (Scheme, "scheme_progress"),
    (Scheme, "scheme_resistance"),
    (Scheme, "scheme_secrecy"),
    (Scheme, "scheme_success_chance"),
    (Accolade, "secondary_tier"),
    (Character, "short_term_gold"),
    (Character, "short_term_gold_maximum"),
    (CombatSide, "side_army_size"),
    (CombatSide, "side_max_army_size"),
    (CombatSide, "side_soldiers"),
    (CombatSide, "side_strength"),
    (Character, "stewardship"),
    (Character, "stewardship_for_portrait"),
    (Character, "stress"),
    (Character, "stress_level"),
    (Character, "strife_opinion"),
    (Character, "sub_realm_size"),
    (Character, "target_weight"),
    (LandedTitle, "tier"),
    (LandedTitle, "title_held_years"), // TODO: warn if this is compared with =
    (Army, "total_army_damage"),
    (Army, "total_army_pursuit"),
    (Army, "total_army_screen"),
    (Army, "total_army_siege_value"),
    (Army, "total_army_toughness"),
    (Character, "travel_leader_cost"),
    (Character, "travel_leader_safety"),
    (Character, "travel_leader_speed"),
    (TravelPlan, "travel_safety"),
    (TravelPlan, "travel_speed"),
    (CombatSide, "troops_ratio"),
    (Character, "tyranny"),
    (Character, "vassal_contract_obligation_level_score"), // TODO: this takes a parameter
    (Character, "vassal_count"),
    (Character, "vassal_limit"),
    (Character, "vassal_limit_available"),
    (Character, "vassal_limit_percentage"),
    (Character, "war_chest_gold"),
    (Character, "war_chest_gold_maximum"),
    (War, "war_days"),
    (Combat, "warscore_value"),
    (Character, "year_of_birth"),
    (Character, "yearly_character_balance"),
    (Character, "yearly_character_expenses"),
    (Character, "yearly_character_income"),
    (Character, "years_as_diarch"),
    (Character, "years_as_ruler"),
    (Character, "years_in_diarchy"),
    (None, "years_from_game_start"),
];
// Special:
// <lifestyle>_perk_points
// <lifestyle>_perks
// <lifestyle>_unlockable_perks
// <lifestyle>_xp
//
// TODO Special:
// <legacy>_track_perks

/// LAST UPDATED VERSION 1.9.0.2
/// See `effects.log` from the game data dumps
/// These are the list iterators. Every entry represents
/// a every_, ordered_, random_, and any_ version.
const SCOPE_ITERATOR: &[(u64, &str, u64)] = &[
    (Activity, "acclaimed_knight", Character),
    (Character, "accolade", Accolade),
    (None, "accolade_type", AccoladeType),
    (Character, "active_accolade", Accolade),
    (None, "activity", Activity),
    (Activity, "activity_phase_location", Province),
    (Activity, "activity_phase_location_future", Province),
    (Activity, "activity_phase_location_past", Province),
    (None, "activity_type", ActivityType),
    (Character, "alert_creatable_title", LandedTitle),
    (Character, "alert_usurpable_title", LandedTitle),
    (Character, "ally", Character),
    (Character, "ancestor", Character),
    (Character, "army", Army),
    (Province, "army_in_location", Army),
    (None, "artifact", Artifact),
    (Artifact, "artifact_claimant", Character),
    (Artifact, "artifact_house_claimant", DynastyHouse),
    (Activity, "attending_character", Character),
    (None, "barony", LandedTitle),
    (Character, "character_artifact", Artifact),
    (Province, "character_in_location", Character),
    (Character, "character_struggle", Struggle),
    (
        Character,
        "character_to_title_neighboring_and_across_water_county",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_and_across_water_duchy",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_and_across_water_empire",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_and_across_water_kingdom",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_county",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_duchy",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_empire",
        LandedTitle,
    ),
    (
        Character,
        "character_to_title_neighboring_kingdom",
        LandedTitle,
    ),
    (Character, "character_trait", Trait),
    (Character, "character_war", War),
    (None, "character_with_royal_court", Character),
    (Character, "child", Character),
    (Character, "claim", LandedTitle),
    (LandedTitle, "claimant", Character),
    (Character, "claimed_artifact", Artifact),
    (Character, "close_family_member", Character),
    (Character, "close_or_extended_family_member", Character),
    (Combat, "combat_side", CombatSide),
    (Character, "concubine", Character),
    (LandedTitle, "connected_county", LandedTitle),
    (Character, "consort", Character),
    (LandedTitle, "controlled_faith", Faith),
    (Character, "councillor", Character),
    (None, "county", LandedTitle),
    (None, "county_in_region", LandedTitle), // TODO region = region_name inside it
    (LandedTitle, "county_province", Province),
    (LandedTitle, "county_struggle", Struggle),
    (Character, "court_position_employer", Character),
    (Character, "court_position_holder", Character), // TODO find out how court position is supplied
    (Character, "courtier", Character),
    (Character, "courtier_away", Character),
    (Character, "courtier_or_guest", Character),
    (Culture, "culture_county", LandedTitle),
    (Culture, "culture_duchy", LandedTitle),
    (Culture, "culture_empire", LandedTitle),
    (None, "culture_global", Culture),
    (Culture, "culture_kingdom", LandedTitle),
    (None, "culture_pillar", CulturePillar),
    (None, "culture_tradition", CultureTradition),
    (Character, "de_jure_claim", LandedTitle),
    (LandedTitle, "de_jure_county", LandedTitle),
    (LandedTitle, "de_jure_county_holder", Character),
    (LandedTitle, "de_jure_top_liege", Character),
    (None, "decision", Decision),
    (Faith, "defensive_great_holy_wars", GreatHolyWar),
    (LandedTitle, "dejure_vassal_title_holder", Character),
    (Character, "diarchy_succession_character", Character),
    (Character, "diplomacy_councillor", Character),
    (LandedTitle, "direct_de_facto_vassal_title", LandedTitle),
    (LandedTitle, "direct_de_jure_vassal_title", LandedTitle),
    (Character, "directly_owned_province", Province),
    (None, "doctrine", Doctrine),
    (None, "duchy", LandedTitle),
    (Dynasty, "dynasty_member", Character),
    (LandedTitle, "election_candidate", Character),
    (Character, "election_title", LandedTitle),
    (LandedTitle, "elector", Character),
    (None, "empire", LandedTitle),
    (TravelPlan, "entourage_character", Character),
    (Character, "equipped_character_artifact", Artifact),
    (Character, "extended_family_member", Character),
    (Faction, "faction_county_member", LandedTitle),
    (Faction, "faction_member", Character),
    (Religion, "faith", Faith),
    (Faith, "faith_character", Character),
    (Faith, "faith_holy_order", HolyOrder),
    (Faith, "faith_playable_ruler", Character),
    (Faith, "faith_ruler", Character),
    (Character, "foreign_court_guest", Character),
    (Character, "former_concubine", Character),
    (Character, "former_concubinist", Character),
    (Character, "former_spouse", Character),
    (TravelPlan, "future_path_location", Province),
    (Character, "general_councillor", Character),
    (Character, "government_type", GovernmentType),
    (Activity, "guest_subset", Character),
    (Activity, "guest_subset_current_phase", Character),
    (Character, "heir", Character),
    // TODO one of these might be reversed
    (Character, "heir_title", LandedTitle),
    (Character, "heir_to_title", LandedTitle),
    (Character, "held_title", LandedTitle),
    (Character, "hired_mercenary", MercenaryCompany),
    (Faith, "holy_site", LandedTitle),
    (Character, "hooked_character", Character),
    (Character, "hostile_raider", Character),
    (DynastyHouse, "house_claimed_artifact", Artifact),
    (DynastyHouse, "house_member", Character),
    (LandedTitle, "in_de_facto_hierarchy", LandedTitle), // TODO has continue section
    (LandedTitle, "in_de_jure_hierarchy", LandedTitle),  // TODO has continue section
    (None, "in_global_list", ALL),                       // TODO list = name or variable = name
    (None, "in_list", ALL),                              // TODO list = name or variable = name
    (None, "in_local_list", ALL),                        // TODO list = name or variable = name
    (None, "independent_ruler", Character),
    (None, "inspiration", Inspiration),
    (None, "inspired_character", Character),
    (Struggle, "interloper_ruler", Character),
    (Character, "intrigue_councillor", Character),
    (Character, "invited_activity", Activity),
    (Activity, "invited_character", Character),
    (Struggle, "involved_ruler", Character),
    (Character | Artifact, "killed_character", Character),
    (None, "kingdom", LandedTitle),
    (Character, "knight", Character),
    (Character, "known_secret", Secret),
    (Character, "learning_councillor", Character),
    (HolyOrder, "leased_title", LandedTitle),
    (Character, "liege_or_above", Character),
    (None, "living_character", Character),
    (Character, "martial_councillor", Character),
    (None, "mercenary_company", MercenaryCompany),
    (Character, "memory", CharacterMemory),
    (CharacterMemory, "memory_participant", Character),
    (
        Character,
        "neighboring_and_across_water_realm_same_rank_owner",
        Character,
    ),
    (
        Character,
        "neighboring_and_across_water_top_liege_realm",
        LandedTitle,
    ),
    (
        Character,
        "neighboring_and_across_water_top_liege_realm_owner",
        Character,
    ),
    (LandedTitle, "neighboring_county", LandedTitle),
    (Province, "neighboring_province", Province),
    (Character, "neighboring_realm_same_rank_owner", Character),
    (Character, "neighboring_top_liege_realm", LandedTitle),
    (Character, "neighboring_top_liege_realm_owner", Character),
    (None, "open_invite_activity", Activity),
    (Character, "opposite_sex_spouse_candidate", Character),
    (Trait, "opposite_trait", Trait),
    (Character, "owned_story", StoryCycle),
    (Character, "parent", Character),
    (Culture, "parent_culture", Culture),
    (Culture, "parent_culture_or_above", Culture),
    (LandedTitle, "past_holder", Character),
    (LandedTitle, "past_holder_reversed", Character),
    (Character, "patroned_holy_order", HolyOrder),
    (Character, "personal_claimed_artifact", Artifact),
    (Character, "pinned_character", Character),
    (Character, "pinning_character", Character),
    (Character, "played_character", Character),
    (None, "player", Character),
    (Character, "player_heir", Character),
    (GreatHolyWar, "pledged_attacker", Character),
    (GreatHolyWar, "pledged_defender", Character),
    (None, "pool_character", Character),
    (Character, "pool_guest", Character),
    (Character, "potential_marriage_option", Character),
    (Character, "powerful_vassal", Character),
    (Character, "pretender_title", LandedTitle),
    (Character, "primary_war_enemy", Character),
    (Character, "prisoner", Character),
    (None, "province", Province),
    (Character, "prowess_councillor", Character),
    (Character, "raid_target", Character),
    (Character, "realm_county", LandedTitle),
    (Character, "realm_de_jure_duchy", LandedTitle),
    (Character, "realm_de_jure_empire", LandedTitle),
    (Character, "realm_de_jure_kingdom", LandedTitle),
    (Character, "realm_province", Province),
    (Character, "relation", Character), // TODO takes a type
    (None, "religion_global", Religion),
    (None, "ruler", Character),
    (Character, "same_sex_spouse_candidate", Character),
    (Character, "scheme", Scheme),
    (Scheme, "scheme_agent", Character),
    (Character, "secret", Secret),
    (Secret, "secret_knower", Character),
    (Secret, "secret_participant", Character),
    (Character, "sibling", Character),
    (CombatSide, "side_commander", Character),
    (CombatSide, "side_knight", Character),
    (Character, "sponsored_inspiration", Inspiration),
    (Character, "spouse", Character),
    (Character, "spouse_candidate", Character),
    (Character, "stewardship_councillor", Character),
    (Character, "sub_realm_barony", LandedTitle),
    (Character, "sub_realm_county", LandedTitle),
    (Character, "sub_realm_duchy", LandedTitle),
    (Character, "sub_realm_empire", LandedTitle),
    (Character, "sub_realm_kingdom", LandedTitle),
    (Character, "sub_realm_title", LandedTitle),
    (CasusBelli, "target_title", LandedTitle),
    (Character, "targeting_faction", Faction),
    (Character, "targeting_scheme", Scheme),
    (Character, "targeting_secret", Secret),
    (LandedTitle, "this_title_or_de_jure_above", LandedTitle),
    (LandedTitle, "title_heir", Character),
    (LandedTitle, "title_joined_faction", Faction),
    (
        LandedTitle,
        "title_to_title_neighboring_and_across_water_county",
        LandedTitle,
    ),
    (
        LandedTitle,
        "title_to_title_neighboring_and_across_water_duchy",
        LandedTitle,
    ),
    (
        LandedTitle,
        "title_to_title_neighboring_and_across_water_empire",
        LandedTitle,
    ),
    (
        LandedTitle,
        "title_to_title_neighboring_and_across_water_kingdom",
        LandedTitle,
    ),
    (
        LandedTitle,
        "title_to_title_neighboring_county",
        LandedTitle,
    ),
    (LandedTitle, "title_to_title_neighboring_duchy", LandedTitle),
    (
        LandedTitle,
        "title_to_title_neighboring_empire",
        LandedTitle,
    ),
    (
        LandedTitle,
        "title_to_title_neighboring_kingdom",
        LandedTitle,
    ),
    (None, "trait", Trait),
    (None, "trait_in_category", Trait),
    (Character, "traveling_family_member", Character),
    (Character, "truce_holder", Character),
    (Character, "truce_target", Character),
    (Character, "unspent_known_secret", Secret),
    (Character, "vassal", Character),
    (None, "vassal_contract", VassalContract),
    (Character, "vassal_or_below", Character),
    (TravelPlan, "visited_location", Province),
    (Character, "war_ally", Character),
    (War, "war_attacker", Character),
    (War, "war_defender", Character),
    (Character, "war_enemy", Character),
    (War, "war_participant", Character),
];

/// LAST UPDATED VERSION 1.9.0.2
/// Every entry represents a every_, ordered_, random_, and any_ version.
const SCOPE_REMOVED_ITERATOR: &[(&str, &str, &str)] = &[
    ("activity_declined", "1.9", ""),
    ("activity_invited", "1.9", ""),
    ("participant", "1.9", ""),
];

const SCOPE_TO_SCOPE_REMOVED: &[(&str, &str, &str)] = &[
    ("activity", "1.9", ""),
    ("activity_owner", "1.9", "replaced by `activity_host`"),
    (
        "activity_province",
        "1.9",
        "replaced by `activity_location`",
    ),
];
