use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoStaticStr, Hash, PartialOrd, Ord, EnumIter)]
#[strum(serialize_all = "snake_case")]
#[cfg(feature = "vic3")]
pub enum Item {
    Accessory,
    AccessoryTag,
    AccessoryVariation,
    AccessoryVariationLayout,
    AccessoryVariationTextures,
    AiStrategy,
    Asset,
    Attitude,
    BattleCondition,
    BlendShape,
    BuildingGroup,
    BuildingType,
    CanalType,
    Country,
    CountryTier,
    Culture,
    CustomLocalization,
    Define,
    Dlc,
    DlcFeature,
    EffectLocalization,
    Entity,
    Ethnicity,
    Event,
    EventNamespace,
    File,
    GameConcept,
    GeneAgePreset,
    GeneAttribute,
    GeneCategory,
    Goods,
    Ideology,
    Institution,
    InterestGroup,
    InterestGroupTrait,
    LawGroup,
    LawType,
    Level,
    Localization,
    MapLayer,
    MediaAlias,
    Modifier,
    NamedColor,
    OnAction,
    Pdxmesh,
    PopType,
    PortraitAnimation,
    ProductionMethod,
    ProductionMethodGroup,
    Province,
    Religion,
    ScriptedEffect,
    ScriptedGui,
    ScriptedList,
    ScriptedModifier,
    ScriptedTrigger,
    ScriptValue,
    SecretGoal,
    Sound,
    StateRegion,
    StateTrait,
    StrategicRegion,
    Technology,
    TechnologyEra,
    Terrain,
    TerrainLabel,
    TerrainManipulator,
    TerrainMask,
    TerrainMaterial,
    TextureFile,
    TransferOfPower,
    TriggerLocalization,
    TutorialLesson,
    Wargoal,
}

impl Item {
    #[cfg(feature = "vic3")]
    pub fn path(self) -> &'static str {
        #[allow(clippy::match_same_arms)]
        match self {
            Item::Accessory => "gfx/portraits/accessories/",
            Item::AccessoryTag => "gfx/portraits/accessories/",
            Item::AccessoryVariation => "gfx/portraits/accessory_variations/",
            Item::AccessoryVariationLayout => "gfx/portraits/accessory_variations/",
            Item::AccessoryVariationTextures => "gfx/portraits/accessory_variations/",
            Item::AiStrategy => "common/ai_strategies/",
            Item::Asset => "gfx/models/",
            Item::Attitude => "",
            Item::BattleCondition => "common/battle_conditions/",
            Item::BlendShape => "gfx/models/",
            Item::BuildingGroup => "common/building_groups/",
            Item::BuildingType => "common/buildings/",
            Item::CanalType => "common/canals/",
            Item::Country => "common/country_definitions/",
            Item::CountryTier => "",
            Item::Culture => "common/cultures/",
            Item::CustomLocalization => "common/customizable_localization/",
            Item::Define => "common/defines/",
            Item::Dlc => "",
            Item::DlcFeature => "",
            Item::EffectLocalization => "common/effect_localization/",
            Item::Entity => "gfx/models/",
            Item::Ethnicity => "common/ethnicities/",
            Item::Event => "events/",
            Item::EventNamespace => "events/",
            Item::File => "",
            Item::GameConcept => "common/game_concepts/",
            Item::GeneAgePreset => "common/genes/",
            Item::GeneAttribute => "gfx/models/",
            Item::GeneCategory => "common/genes/",
            Item::Goods => "common/goods/",
            Item::Ideology => "common/ideologies/",
            Item::Institution => "common/institutions/",
            Item::InterestGroup => "common/interest_groups/",
            Item::InterestGroupTrait => "common/interest_group_traits/",
            Item::LawGroup => "common/law_groups/",
            Item::LawType => "common/laws/",
            Item::Level => "",
            Item::Localization => "localization/",
            Item::MapLayer => "gfx/map/map_object_data/layers.txt",
            Item::MediaAlias => "gfx/media_aliases/",
            Item::Modifier => "common/modifiers/",
            Item::NamedColor => "common/named_colors/",
            Item::OnAction => "common/on_actions/",
            Item::Pdxmesh => "gfx/models/",
            Item::PopType => "common/pop_types/",
            Item::PortraitAnimation => "gfx/portraits/portrait_animations/",
            Item::ProductionMethod => "common/production_methods/",
            Item::ProductionMethodGroup => "common/production_method_groups/",
            Item::Province => "map_data/provinces.png",
            Item::Religion => "common/religions/",
            Item::ScriptedEffect => "common/scripted_effects/",
            Item::ScriptedGui => "common/scripted_guis/",
            Item::ScriptedList => "common/scripted_lists/",
            Item::ScriptedModifier => "common/scripted_modifiers/",
            Item::ScriptedTrigger => "common/scripted_triggers/",
            Item::ScriptValue => "common/script_values/",
            Item::SecretGoal => "common/secret_goals/",
            Item::Sound => "",
            Item::StateRegion => "map_data/state_regions/",
            Item::StateTrait => "common/state_traits/",
            Item::StrategicRegion => "common/strategic_regions/",
            Item::Technology => "common/technology/technologies/",
            Item::TechnologyEra => "common/technology/eras/",
            Item::Terrain => "common/terrain/",
            Item::TerrainLabel => "common/labels",
            Item::TerrainManipulator => "common/terrain_manipulators/",
            Item::TerrainMask => "gfx/map/masks/",
            Item::TerrainMaterial => "gfx/map/terrain/materials.settings",
            Item::TextureFile => "gfx/models/",
            Item::TransferOfPower => "common/government_types/",
            Item::TriggerLocalization => "common/trigger_localization/",
            Item::TutorialLesson => "common/tutorial_lessons/",
            Item::Wargoal => "",
        }
    }
}
