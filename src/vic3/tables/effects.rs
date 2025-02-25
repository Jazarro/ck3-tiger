use crate::effect::Effect;
use crate::everything::Everything;
use crate::item::Item;
use crate::scopes::*;
use crate::token::Token;
use crate::vic3::effect_validation::{EvB, EvBv, EvV};

use Effect::*;

pub fn scope_effect(name: &Token, _data: &Everything) -> Option<(Scopes, Effect)> {
    let lwname = name.as_str().to_lowercase();

    for (from, s, effect) in SCOPE_EFFECT {
        if lwname == *s {
            return Some((Scopes::from_bits_truncate(*from), *effect));
        }
    }
    std::option::Option::None
}

/// LAST UPDATED VERSION 1.9.2
/// See `effects.log` from the game data dumps
const SCOPE_EFFECT: &[(u64, &str, Effect)] = &[
    (InterestGroup, "abandon_revolution", Boolean),
    (State, "activate_building", Item(Item::BuildingType)),
    (Country, "activate_law", Scope(Scopes::LawType)),
    (Country | State, "activate_production_method", VB(EvB::ActivateProductionMethod)),
    (StateRegion, "add_arable_land", Integer),
    (Country, "add_banned_goods", Scope(Scopes::Goods)),
    (Country, "add_change_relations_progress", Unchecked), // Not used in vanilla
    (Character, "add_character_role", Item(Item::CharacterRole)),
    (CivilWar, "add_civil_war_progress", ScriptValue),
    (StateRegion, "add_claim", Scope(Scopes::Country)),
    (Character, "add_commander_rank", Integer),
    (Culture, "add_cultural_obsession", Item(Item::Goods)),
    (State, "add_culture_standard_of_living_modifier", VB(EvB::AddCultureSolModifier)),
    (Country, "add_declared_interest", Item(Item::StrategicRegion)),
    (StateRegion, "add_devastation", Integer),
    (
        DiplomaticPlay,
        "add_diplomatic_play_war_support",
        TargetValue("target", Scopes::Country, "value"),
    ),
    (Country, "add_enactment_modifier", AddModifier),
    (Country, "add_enactment_phase", Integer),
    (Country, "add_enactment_setback", Integer),
    (Country, "add_era_researched", Item(Item::TechnologyEra)),
    (DiplomaticPlay, "add_escalation", Integer),
    (Character, "add_experience", ScriptValue),
    (StateRegion, "add_homeland", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (InterestGroup, "add_ideology", Item(Item::Ideology)),
    (Party, "add_ig_to_party", Scope(Scopes::InterestGroup)),
    (DiplomaticPlay, "add_initiator_backers", Unchecked), // Not used in vanilla
    (Country, "add_investment_pool", ScriptValue),
    (None, "add_journal_entry", VB(EvB::AddJournalentry)),
    (Country, "add_law_progress", ScriptValue),
    (Country, "add_loyalists", VB(EvB::AddLoyalists)),
    (State, "add_loyalists_in_state", VB(EvB::AddLoyalists)),
    (
        Country
            | Building
            | Character
            | Front
            | Institution
            | InterestGroup
            | Journalentry
            | PoliticalMovement
            | State,
        "add_modifier",
        Unchecked,
    ),
    (Party, "add_momentum", ScriptValue),
    (StateRegion, "add_pollution", Integer),
    (Pop, "add_pop_wealth", Unchecked),
    (Country, "add_primary_culture", Scope(Scopes::Culture)),
    (Country, "add_radicals", VB(EvB::AddLoyalists)),
    (State, "add_radicals_in_state", VB(EvB::AddLoyalists)),
    (Character, "add_random_trait", Choice(&["personality", "skill", "condition"])),
    (State, "add_religion_standard_of_living_modifier", Unchecked),
    (InterestGroup, "add_ruling_interest_group", Boolean),
    (DiplomaticPlay, "add_target_backers", Unchecked),
    (Country, "add_taxed_goods", Scope(Scopes::Goods)),
    (Country, "add_technology_progress", Unchecked),
    (Country, "add_technology_researched", Item(Item::Technology)),
    (None, "add_to_global_variable_list", VB(EvB::AddToVariableList)),
    (ALL_BUT_NONE, "add_to_list", VV(EvV::AddToList)),
    (None, "add_to_local_variable_list", VB(EvB::AddToVariableList)),
    (ALL_BUT_NONE, "add_to_temporary_list", VV(EvV::AddToList)),
    (None, "add_to_variable_list", VB(EvB::AddToVariableList)),
    (Character, "add_trait", Unchecked),
    (Country, "add_treasury", ScriptValue),
    (War, "add_war_exhaustion", Unchecked),
    (DiplomaticPlay, "add_war_goal", Unchecked),
    (War, "add_war_war_support", Unchecked),
    (Country, "annex", Scope(Scopes::Country)),
    (Country, "annex_as_civil_war", Scope(Scopes::Country)),
    (None, "assert_if", Unchecked),
    (None, "assert_read", Unchecked),
    (Country, "call_election", Unchecked),
    (Country, "cancel_enactment", Yes),
    (Character, "change_character_religion", Scope(Scopes::Religion)),
    (None, "change_global_variable", VB(EvB::ChangeVariable)),
    (None, "change_infamy", ScriptValue),
    (Country, "change_institution_investment_level", Unchecked),
    (None, "change_local_variable", VB(EvB::ChangeVariable)),
    (Pop, "change_pop_culture", Unchecked),
    (Pop, "change_pop_religion", Unchecked),
    (Pop, "change_poptype", Scope(Scopes::PopType)),
    (Country, "change_relations", Unchecked),
    (Country, "change_subject_type", Unchecked),
    (Country, "change_tag", Item(Item::Country)),
    (Country, "change_tension", Unchecked),
    (None, "change_variable", VB(EvB::ChangeVariable)),
    (None, "clamp_global_variable", VB(EvB::ClampVariable)),
    (None, "clamp_local_variable", VB(EvB::ClampVariable)),
    (None, "clamp_variable", VB(EvB::ClampVariable)),
    (Country, "clear_debt", Boolean),
    (Country, "clear_enactment_modifier", Yes),
    (None, "clear_global_variable_list", Unchecked),
    (None, "clear_local_variable_list", Unchecked),
    (None, "clear_saved_scope", Unchecked),
    (Country, "clear_scaled_debt", ScriptValue),
    (None, "clear_variable_list", Unchecked),
    (Country, "complete_objective_subgoal", Unchecked),
    (State, "convert_population", Unchecked),
    (State, "create_building", Unchecked),
    (Country, "create_character", Unchecked),
    (None, "create_country", Unchecked),
    (Country, "create_diplomatic_pact", Unchecked),
    (Country, "create_diplomatic_play", Unchecked),
    (Country, "create_incident", Unchecked),
    (State, "create_pop", Unchecked),
    (StateRegion, "create_state", Unchecked),
    (Country, "create_trade_route", Unchecked),
    (Country, "create_truce", Unchecked),
    (None, "custom_description", Control),
    (None, "custom_description_no_bullet", Control),
    (None, "custom_label", ControlOrLabel),
    (None, "custom_tooltip", ControlOrLabel),
    (State, "deactivate_building", Item(Item::BuildingType)),
    (Country, "deactivate_law", Scope(Scopes::LawType)),
    (Country, "deactivate_parties", Yes),
    (None, "debug_log", Unchecked),
    (None, "debug_log_scopes", Boolean),
    (Character, "demobilize", Yes),
    (Party, "disband_party", Yes),
    (Character, "disinherit_character", Yes),
    (None, "else", Control),
    (None, "else_if", Control),
    (DiplomaticPlay, "end_play", Boolean),
    (Country, "end_truce", Unchecked),
    (Character, "exile_character", Yes),
    (State, "force_resource_depletion", Unchecked),
    (State, "force_resource_discovery", Unchecked),
    (Character, "free_character_from_void", Yes),
    (None, "hidden_effect", Control),
    (None, "if", Control),
    (InterestGroup, "join_revolution", Boolean),
    (Character, "kill_character", Unchecked),
    (Country, "kill_population_percent", Unchecked),
    (State, "kill_population_percent_in_state", Unchecked),
    (TradeRoute, "lock_trade_route", Unchecked),
    (Country, "make_independent", Boolean),
    (Pop, "move_pop", Scope(Scopes::State)),
    (Character, "place_character_in_void", ScriptValue),
    (Country, "play_as", Scope(Scopes::Country)),
    (None, "post_notification", Unchecked),
    (None, "post_proposal", Unchecked),
    (None, "random", Control),
    (None, "random_list", VB(EvB::RandomList)),
    (None, "random_log_scopes", Boolean),
    (Country, "recalculate_pop_ig_support", Boolean),
    (Country, "remove_active_objective_subgoal", Unchecked),
    (Character, "remove_as_interest_group_leader", Yes),
    (Country, "remove_banned_goods", Scope(Scopes::Goods)),
    (State, "remove_building", Item(Item::BuildingType)),
    (Character, "remove_character_role", Item(Item::CharacterRole)),
    (StateRegion, "remove_claim", Scope(Scopes::Country)),
    (Culture, "remove_cultural_obsession", Item(Item::Goods)),
    (Country, "remove_diplomatic_pact", Unchecked),
    (Country, "remove_enactment_modifier", Unchecked),
    (ALL_BUT_NONE, "remove_from_list", VV(EvV::RemoveFromList)),
    (None, "remove_global_variable", Unchecked),
    (StateRegion, "remove_homeland", ScopeOrItem(Scopes::Culture, Item::Culture)),
    (InterestGroup, "remove_ideology", Item(Item::Ideology)),
    (Party, "remove_ig_from_party", Scope(Scopes::InterestGroup)),
    (DiplomaticPlay, "remove_initiator_backers", Unchecked),
    (None, "remove_list_global_variable", VB(EvB::AddToVariableList)),
    (None, "remove_list_local_variable", VB(EvB::AddToVariableList)),
    (None, "remove_list_variable", VB(EvB::AddToVariableList)),
    (None, "remove_local_variable", Unchecked),
    (
        Country
            | Building
            | Character
            | Front
            | Institution
            | InterestGroup
            | Journalentry
            | PoliticalMovement
            | State,
        "remove_modifier",
        Unchecked,
    ),
    (Country, "remove_primary_culture", Scope(Scopes::Culture)),
    (InterestGroup, "remove_ruling_interest_group", Boolean),
    (DiplomaticPlay, "remove_target_backers", Unchecked),
    (Country, "remove_taxed_goods", Scope(Scopes::Goods)),
    (Character, "remove_trait", Unchecked),
    (None, "remove_variable", Unchecked),
    (DiplomaticPlay, "remove_war_goal", Unchecked),
    (DiplomaticPlay, "resolve_play_for", Scope(Scopes::Country)),
    (None, "round_global_variable", VB(EvB::RoundVariable)),
    (None, "round_local_variable", VB(EvB::RoundVariable)),
    (None, "round_variable", VB(EvB::RoundVariable)),
    (ALL_BUT_NONE, "save_scope_as", VV(EvV::SaveScope)),
    (None, "save_scope_value_as", VB(EvB::SaveScopeValue)),
    (ALL_BUT_NONE, "save_temporary_scope_as", VV(EvV::SaveScope)),
    (None, "save_temporary_scope_value_as", VB(EvB::SaveScopeValue)),
    (Country, "seize_investment_pool", Boolean),
    (Character, "set_as_interest_group_leader", Yes),
    (State, "set_available_for_autonomous_investment", Scope(Scopes::BuildingType)),
    (Country, "set_capital", Item(Item::StateRegion)),
    (Character, "set_character_as_ruler", Yes),
    (Character, "set_character_busy", Boolean),
    (Character, "set_commander_rank", Integer),
    (Country, "set_country_type", Item(Item::CountryType)),
    (StateRegion, "set_devastation", Integer),
    (Country, "set_diplomats_expelled", Scope(Scopes::Country)),
    (None, "set_global_variable", VBv(EvBv::SetVariable)),
    (Country, "set_government_wage_level", Unchecked),
    (Character, "set_home_country", Scope(Scopes::Country)),
    (Character, "set_home_country_definition", Scope(Scopes::CountryDefinition)),
    (Character, "set_ideology", Scope(Scopes::Ideology)),
    (InterestGroup, "set_ig_bolstering", Boolean),
    (InterestGroup, "set_ig_suppression", Boolean),
    (InterestGroup, "set_ig_trait", Scope(Scopes::InterestGroupTrait)),
    (Country, "set_institution_investment_level", Unchecked),
    (InterestGroup, "set_interest_group_name", Item(Item::Localization)),
    (DiplomaticPlay, "set_key", Item(Item::Localization)),
    (None, "set_local_variable", VBv(EvBv::SetVariable)),
    (Country, "set_market_capital", Item(Item::StateRegion)),
    (Country, "set_military_wage_level", Unchecked),
    (Country, "set_mutual_secret_goal", Unchecked),
    (Country, "set_next_election_date", Date),
    (Country, "set_owes_obligation_to", Unchecked),
    (StateRegion, "set_owner_of_provinces", Unchecked),
    (Pop, "set_pop_literacy", Unchecked),
    (Pop, "set_pop_qualifications", Unchecked),
    (Pop, "set_pop_wealth", Unchecked),
    (Country, "set_relations", Unchecked),
    (Country, "set_ruling_interest_groups", Unchecked),
    (Party, "set_ruling_party", Yes),
    (Country, "set_secret_goal", Unchecked),
    (State, "set_state_owner", Scope(Scopes::Country)),
    (Country, "set_state_religion", Scope(Scopes::Religion)),
    (State, "set_state_type", Unchecked),
    (Country, "set_strategy", Unchecked),
    (Building, "set_subsidized", Boolean),
    (Journalentry, "set_target_technology", Unchecked),
    (Country, "set_tariffs_export_priority", Scope(Scopes::Goods)),
    (Country, "set_tariffs_import_priority", Scope(Scopes::Goods)),
    (Country, "set_tariffs_no_priority", Scope(Scopes::Goods)),
    (Country, "set_tax_level", Unchecked),
    (Country, "set_tension", Unchecked),
    (None, "set_variable", VBv(EvBv::SetVariable)),
    (DiplomaticPlay, "set_war", Boolean),
    (None, "show_as_tooltip", Control),
    (State, "start_building_construction", Item(Item::BuildingType)),
    (State, "start_privately_funded_building_construction", Item(Item::BuildingType)),
    (Country, "start_research_random_technology", Yes),
    (None, "start_tutorial_lesson", Unchecked),
    (None, "switch", VB(EvB::Switch)),
    (Country, "take_on_scaled_debt", Unchecked),
    (Character, "transfer_character", Scope(Scopes::Country)),
    (None, "trigger_event", Unchecked),
    (State, "unset_available_for_autonomous_investment", Scope(Scopes::BuildingType)),
    (Country, "update_party_support", Yes),
    (Country, "validate_subsidies", Boolean),
    (Country, "violate_sovereignty_join", Unchecked),
    (None, "while", Control),
];
