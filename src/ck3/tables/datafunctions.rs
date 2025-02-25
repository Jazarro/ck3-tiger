#![allow(non_camel_case_types)]

use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::datatype::{Arg, Args, LookupResult};
use crate::item::Item;
use crate::scopes::Scopes;

use Arg::*;
use Datatype::*;

// Validate the "code" blocks in localization files and in the gui files.
// The include/ files are converted from the game's data_type_* output files.

include!("include/datatypes.rs");

pub fn lookup_global_promote(lookup_name: &str) -> Option<(Args, Datatype)> {
    if let Ok(idx) = GLOBAL_PROMOTES.binary_search_by_key(&lookup_name, |(name, _, _)| name) {
        let (_name, args, rtype) = GLOBAL_PROMOTES[idx];
        return Some((args, rtype));
    }

    // Datatypes can be used directly as global promotes, taking their value from the gui context.
    if let Ok(dtype) = Datatype::from_str(lookup_name) {
        return Some((Args(&[]), dtype));
    }

    None
}

pub fn lookup_global_function(lookup_name: &str) -> Option<(Args, Datatype)> {
    if let Ok(idx) = GLOBAL_FUNCTIONS.binary_search_by_key(&lookup_name, |(name, _, _)| name) {
        let (_name, args, rtype) = GLOBAL_FUNCTIONS[idx];
        return Some((args, rtype));
    }
    None
}

fn lookup_promote_or_function(
    lookup_name: &str,
    ltype: Datatype,
    global: &[(&str, Datatype, Args, Datatype)],
) -> LookupResult {
    let start = global.partition_point(|(name, _, _, _)| name < &lookup_name);
    let mut found_any = false;
    let mut possible_args = None;
    let mut possible_rtype = None;
    for (name, intype, args, rtype) in global.iter().skip(start) {
        if lookup_name != *name {
            break;
        }
        found_any = true;
        if ltype == Datatype::Unknown {
            if possible_rtype.is_none() {
                possible_args = Some(*args);
                possible_rtype = Some(*rtype);
            } else if possible_rtype != Some(*rtype) {
                possible_rtype = Some(Datatype::Unknown);
            }
        } else if ltype == *intype {
            return LookupResult::Found(*args, *rtype);
        }
    }

    if found_any {
        if ltype == Datatype::Unknown {
            LookupResult::Found(possible_args.unwrap(), possible_rtype.unwrap())
        } else {
            LookupResult::WrongType
        }
    } else {
        LookupResult::NotFound
    }
}

pub fn lookup_promote(lookup_name: &str, ltype: Datatype) -> LookupResult {
    lookup_promote_or_function(lookup_name, ltype, PROMOTES)
}

pub fn lookup_function(lookup_name: &str, ltype: Datatype) -> LookupResult {
    lookup_promote_or_function(lookup_name, ltype, FUNCTIONS)
}

/// Find an alternative datafunction to suggest when `lookup_name` has not been found.
/// This is a fairly expensive lookup.
/// Currently it only looks for different-case variants.
/// TODO: make it consider misspellings as well
pub fn lookup_alternative(
    lookup_name: &str,
    first: std::primitive::bool,
    last: std::primitive::bool,
) -> Option<&'static str> {
    let lc = lookup_name.to_lowercase();
    if first {
        for (name, _, _) in GLOBAL_PROMOTES {
            if name.to_lowercase() == lc {
                return Some(name);
            }
        }
        if last {
            for (name, _, _) in GLOBAL_FUNCTIONS {
                if name.to_lowercase() == lc {
                    return Some(name);
                }
            }
        }
    } else {
        for (name, _, _, _) in PROMOTES {
            if name.to_lowercase() == lc {
                return Some(name);
            }
        }
        if last {
            for (name, _, _, _) in FUNCTIONS {
                if name.to_lowercase() == lc {
                    return Some(name);
                }
            }
        }
    }
    None
}

/// TODO: make a lookup for this table, rather than sequential scanning
const DATATYPE_AND_SCOPE: &[(Datatype, Scopes)] = &[
    (Datatype::Character, Scopes::Character),
    (Datatype::Title, Scopes::LandedTitle),
    (Datatype::Activity, Scopes::Activity),
    (Datatype::Secret, Scopes::Secret),
    (Datatype::Province, Scopes::Province),
    (Datatype::Scheme, Scopes::Scheme),
    (Datatype::Combat, Scopes::Combat),
    (Datatype::CombatSide, Scopes::CombatSide),
    (Datatype::Faith, Scopes::Faith),
    (Datatype::GreatHolyWar, Scopes::GreatHolyWar),
    (Datatype::Religion, Scopes::Religion),
    (Datatype::War, Scopes::War),
    (Datatype::Story, Scopes::StoryCycle),
    (Datatype::CasusBelliItem, Scopes::CasusBelli),
    (Datatype::Dynasty, Scopes::Dynasty),
    (Datatype::DynastyHouse, Scopes::DynastyHouse),
    (Datatype::Faction, Scopes::Faction),
    (Datatype::Culture, Scopes::Culture),
    (Datatype::Army, Scopes::Army),
    (Datatype::HolyOrder, Scopes::HolyOrder),
    (Datatype::ActiveCouncilTask, Scopes::CouncilTask),
    (Datatype::MercenaryCompany, Scopes::MercenaryCompany),
    (Datatype::Artifact, Scopes::Artifact),
    (Datatype::Inspiration, Scopes::Inspiration),
    (Datatype::Struggle, Scopes::Struggle),
    (Datatype::CharacterMemory, Scopes::CharacterMemory),
    (Datatype::TravelPlan, Scopes::TravelPlan),
    (Datatype::Accolade, Scopes::Accolade),
    (Datatype::AccoladeType, Scopes::AccoladeType),
    (Datatype::Decision, Scopes::Decision),
    (Datatype::FaithDoctrine, Scopes::Doctrine),
    (Datatype::ActivityType, Scopes::ActivityType),
    (Datatype::CultureTradition, Scopes::CultureTradition),
    (Datatype::CulturePillar, Scopes::CulturePillar),
    (Datatype::GovernmentType, Scopes::GovernmentType),
    (Datatype::Trait, Scopes::Trait),
    (Datatype::VassalContract, Scopes::VassalContract),
    (Datatype::ObligationLevel, Scopes::VassalObligationLevel),
];

/// Return the scope type that best matches `dtype`, or `None` if there is no match.
/// Nearly every scope type has a matching datatype, but there are far more datatypes than scope types.
pub fn scope_from_datatype(dtype: Datatype) -> Option<Scopes> {
    for (dt, s) in DATATYPE_AND_SCOPE {
        if dtype == *dt {
            return Some(*s);
        }
    }
    None
}

/// Return the datatype that best matches `scopes`, or `Datatype::Unknown` if there is no match.
/// Nearly every scope type has a matching datatype, but there are far more datatypes than scope types.
/// Note that only `Scopes` values that are narrowed down to a single scope type can be matched.
pub fn datatype_from_scopes(scopes: Scopes) -> Datatype {
    for (dt, s) in DATATYPE_AND_SCOPE {
        if scopes == *s {
            return *dt;
        }
    }
    Datatype::Unknown
}

const GLOBAL_PROMOTES: &[(&str, Args, Datatype)] = include!("include/data_global_promotes.rs");

const GLOBAL_FUNCTIONS: &[(&str, Args, Datatype)] = include!("include/data_global_functions.rs");

const PROMOTES: &[(&str, Datatype, Args, Datatype)] = include!("include/data_promotes.rs");

const FUNCTIONS: &[(&str, Datatype, Args, Datatype)] = include!("include/data_functions.rs");
