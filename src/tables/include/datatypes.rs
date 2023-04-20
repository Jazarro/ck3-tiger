#[derive(Copy, Clone, Debug)]
enum DataType {
    Unknown,
    Application,
    CFixedPoint,
    CPdxFloatRect,
    CPdxIntRect,
    CString,
    CUTF8String,
    CVector2f,
    CVector2i,
    CVector3f,
    CVector3i,
    CVector4f,
    CVector4i,
    Date,
    Playable,
    VariableSystem,
    bool,
    double,
    float,
    int16,
    int32,
    int64,
    int8,
    uint16,
    uint32,
    uint64,
    uint8,
    void,
    Achievement,
    AchievementPopup,
    AchievementWindow,
    AddFriendWindow,
    AvailabilityEntry,
    CPdxInputBindingSetting,
    Character,
    Chat,
    ChatMessage,
    ChatNotificationMessage,
    ChatTab,
    ChatWindow,
    CoatOfArms,
    CountryEntry,
    CreateSocialProfile,
    CreateSocialProfileWindow,
    CreditsWindow,
    Encyclopedia,
    EncyclopediaEntry,
    EncyclopediaEntryView,
    EncyclopediaPage,
    EndPrepConfirm,
    EnumSettingEntry,
    ErrorMessageBox,
    Ethnicity,
    Friend,
    FriendListWindow,
    FriendRequest,
    FriendSearchResult,
    Friends,
    FrontEndCreditsView,
    FrontEndMultiplayerView,
    GUIAchievement,
    GeneCategory,
    GeneTemplate,
    Group,
    GuiContext,
    InputActionBinding,
    JominiGUISetting,
    JominiNotification,
    JominiNotificationOverlay,
    JominiPasswordPopup,
    JominiServer,
    JominiServerBrowserGui,
    JominiSettingsWindow,
    LanguageEntry,
    LobbyPlayer,
    LobbyView,
    MPConfig,
    MultiplayerSetupWindow,
    NotificationDummyContext,
    OosData,
    OosWindow,
    OutgoingFriendRequest,
    POPSCreateAccount,
    POPSLoginView,
    POPSStatusWidget,
    PdxGuiFoldOut,
    PdxGuiGfxVideoControl,
    PdxGuiTableRow,
    PdxGuiTreeTable,
    PdxGuiWidget,
    PlayerJoinRequest,
    PlotLine,
    PortraitTooltip,
    RemoveFriendConfirmWindow,
    ScopedJominiSettingsCategory,
    ScopedJominiSettingsPage,
    ServerInformation,
    SettingCategory,
    SettingsPage,
    Social,
    SocialNotificationWindow,
    SocialUI,
    SocialWidget,
    AnchorItem,
    AnimationCurveValue,
    Attribute,
    BlurThreshold,
    BrushBool,
    BrushFloat,
    BrushSettings,
    BrushSettingsDropdown,
    BrushSettingsGlobal,
    CPdxEnumValue,
    ChildGenerator,
    ChildItem,
    ConsoleMenuItem,
    ConsoleWindow,
    ContextMenuItem,
    CurveEditor,
    CurvePoint,
    DatatypesExplorer,
    DetailData,
    DockableLayout,
    DockableLayoutManager,
    DockableWindow,
    DrawCmdsList,
    DrawCmdsViewer,
    EntityEditor,
    EntityEditorAudioEventHandler,
    EntityEditorEventLayer,
    EntityEditorEventLayerPtr,
    EntityEditorKeyframe,
    EntityEditorTimelineState,
    EntityViewerProperties,
    EthnicityItem,
    EventLayerForEntityEditor,
    EventTargetSetupContext,
    ExportTool,
    EyeDropper,
    EyeDropperPackedSample,
    GeneItem,
    GenerationItem,
    GfxSkin,
    Graph,
    GraphPanel,
    GroupNodeWindow,
    GuiAnimationCurveEditor,
    GuiAnimationCurveEditorControlPoint,
    GuiAnimationCurveEditorLine,
    GuiAnimationCurveEditorViewport,
    GuiAnimationEditor,
    GuiAnimationEditorAnimSetEntry,
    GuiAnimationEditorAnimationEntry,
    GuiAnimationEditorAvailableTrack,
    GuiAnimationEditorKeyframe,
    GuiAnimationEditorMetadataCtx,
    GuiAnimationEditorPlayer,
    GuiAnimationEditorPlayerSpeedMultiplierEntry,
    GuiAnimationEditorTimeResolutionEntry,
    GuiAnimationEditorUniversalTrack,
    GuiAnimationEditorUserInput,
    GuiAnimationTimeline,
    GuiAnimationTimelineEventLayer,
    GuiAnimationTimelineEventLayerPtr,
    GuiAnimationTimelineKeyframe,
    GuiAnimationTimelineState,
    GuiAnimationTimelineWidgetEntry,
    GuiEditor,
    GuiEditorCategory,
    GuiEditorDockable,
    GuiEditorOutliner,
    GuiEditorProperties,
    GuiEditorProperty,
    GuiEditorTooltip,
    Heightmap,
    HeightmapPainter,
    HeightmapPainterMode,
    HeightmapResolution,
    ImportTool,
    Importable,
    ImportableGroup,
    InspectorPanel,
    KeyframeEditor,
    KeyframeEventEditor,
    KeyframeWidget,
    Layer,
    LayerTreeItem,
    LogEntry,
    LogViewer,
    LogViewerCategory,
    LogViewerEntry,
    LogViewerType,
    MapContentEditorMode,
    MapContentEditorOptions,
    MapContentEditorViewport,
    MapContentEntryDesc,
    MapContentLayerDesc,
    MapContentPanel,
    MapContentPropertyGroup,
    MapContentSelector,
    MapContentSelectorGui,
    MapEditor,
    MapEditorGui,
    MapEditorLayerBorder,
    MapEditorLayerBorderDockable,
    MapObjectMask,
    MapObjectPainter,
    MapObjectPainterMode,
    MapObjectPainterOptions,
    MapObjectTool,
    MaskEntry,
    MaskManagerEntry,
    MaskPainterManager,
    MaskPainterMapContentPanel,
    MaskPainterMode,
    MaskPainterTool,
    MaskPainterViewport,
    Material,
    MaterialBrowser,
    MaterialEntry,
    MaterialMix,
    MaterialMixBrush,
    MaterialMixEntry,
    MaterialPaintingMode,
    Materials,
    MaterialsSample,
    MixBrushMode,
    MoveTool,
    Node,
    NodeError,
    NodeLine,
    NodePin,
    NodeWindow,
    NonRegisteredDockable,
    NsDecomposeAggregateWindow,
    Nudger,
    NudgerLayerEntryMapObjectDesc,
    NudgerMapContentGui,
    NudgerMapObjectPropertyListDockable,
    NudgerMode,
    ObjectBrowser,
    ObjectBrowserView,
    ObjectInspector,
    ObjectInspectorDockable,
    ObjectInspectorPlugin,
    ObjectPreset,
    ObjectProvider,
    OutputEntry,
    ParametricSelect,
    PdxCoreSetting,
    PdxSetting,
    PdxSettingsWindow,
    PdxSettingsWindowCategory,
    PdxSettingsWindowScopedCategory,
    PdxValueSetting,
    Portrait3dView,
    PortraitDataContext,
    PortraitEditorAnimationItem,
    PortraitEditorWindow,
    PreviewMaskTexture,
    RepackWindow,
    Savable,
    SavableGroup,
    SaveDialog,
    SaveGameAnalysisView,
    ScopeDebugData,
    ScopeDebugInspectorPlugin,
    ScopeObjectEditor,
    ScopeObjectProvider,
    ScopeObjectType,
    ScriptRunnerInspector,
    ScriptRunnerResult,
    SelectTool,
    SelectionHistory,
    SelectionLine,
    SkinEditor,
    SmartBrushHeightRange,
    SmartBrushPattern,
    SmartBrushPresetManager,
    SmartMaterialPaintingMode,
    SplineAdjustmentTool,
    SplineAdjustmentToolMode,
    SplineRiverInteractionMode,
    SplineStripTool,
    SplineStripToolMode,
    SplineToolsMapContentPanel,
    SplineTypeCreateSelectionDropdown,
    SplineTypeItem,
    SplineTypeSwitchSelectionDropdown,
    SplineVisibilityDropdown,
    TerrainToolButton,
    TextureEntry,
    TextureImporter,
    TextureList,
    TextureListDirectory,
    TextureListTexture,
    TextureNodeWindow,
    TextureViewer,
    ThreadDebugTask,
    ThreadDebugThread,
    ThreadDebugView,
    TimelineEventLayerForGuiAnimation,
    TimelineKeyframe,
    ToolDialog,
    ToolDialogButton,
    ToolMessageDialog,
    ToolProperty,
    ToolPropertyBool,
    ToolPropertyCColor,
    ToolPropertyCString,
    ToolPropertyColor,
    ToolPropertyCurve,
    ToolPropertyFloat,
    ToolPropertyInt,
    ToolPropertyInt16,
    ToolPropertyInt8,
    ToolPropertyList,
    ToolPropertySearchList,
    ToolPropertyString,
    ToolPropertyUint,
    ToolPropertyUint16,
    ToolPropertyUint8,
    ToolPropertyUndoableSearchList,
    ToolPropertyVec1fPercent,
    ToolPropertyVec2f,
    ToolPropertyVec2fPercent,
    ToolPropertyVec2i,
    ToolPropertyVec3f,
    ToolPropertyVec3i,
    ToolPropertyVec4i,
    ToolsPropertyRangedValueFloat,
    ToolsPropertyRangedValueInt,
    ToolsSearch,
    ToolsSearchResult,
    ToolsUndoableValueBundleBool,
    ToolsUndoableValueBundleCColor,
    ToolsUndoableValueBundleCString,
    ToolsUndoableValueBundleColor,
    ToolsUndoableValueBundleFloat,
    ToolsUndoableValueBundleInt,
    ToolsUndoableValueBundleString,
    ToolsUndoableValueBundleUint,
    Tweaker,
    Type,
    UserDataNode,
    VariableEntry,
    VariableInspectorEntry,
    VariableInspectorPlugin,
    VariableInspectorVariable,
    VariableList,
    VariableListEntry,
    VariableListInspectorPlugin,
    VariableListStore,
    VariableStore,
    ViewerEntity,
    ViewerEntityLodInfo,
    ViewerEntityState,
    ViewerEntityStatePtr,
    ActiveCouncilTask,
    Activity,
    Army,
    Artifact,
    CharacterMemory,
    Combat,
    Container,
    Culture,
    Dynasty,
    DynastyHouse,
    Faction,
    Faith,
    GreatHolyWar,
    HolyOrder,
    Inspiration,
    MercenaryCompany,
    Province,
    Religion,
    Scheme,
    Scope,
    ScriptedGui,
    Secret,
    Story,
    Struggle,
    Title,
    TopScope,
    War,
    AIUnitWatchWindow,
    AIWarCoordinatorWatchWindow,
    AIWatchWindow,
    AccessoryItem,
    AccessoryItemOption,
    ActionItemHandler,
    ActiveCasusBelli,
    ActiveCouncilTaskIcon,
    ActivityMapItem,
    ActivityType,
    ActivityWindow,
    AddTraditionWindow,
    AllianceInfo,
    AllyListSubview,
    AnimationTestGroupItem,
    AnimationTestWindow,
    AppointCourtPositionView,
    ArmyComposition,
    ArmyRegiment,
    ArmyReorgWindow,
    ArmyWindow,
    ArtifactClaim,
    ArtifactClaimsList,
    ArtifactDetailsView,
    ArtifactHelperWindow,
    ArtifactHistory,
    ArtifactKillListWindow,
    ArtifactSettings,
    ArtifactType,
    ArtifactVisualType,
    AssetSettings,
    AttachToArmyWindow,
    BattleEvent,
    BattleSummaryWindow,
    BlackmailInteractionWindow,
    BlackmailSecretItem,
    Bookmark,
    BookmarkCharacter,
    BookmarkCharacterGUI,
    BookmarkGroup,
    BookmarkGroupItem,
    BookmarkItem,
    BookmarkPortrait,
    BottomBarSchemeItem,
    Building,
    BuildingLevelItem,
    CCourtLanguageMapIcon,
    CSelectCommanderWindow,
    CallAllyInteractionNotificationWindow,
    CallAllyInteractionWindow,
    CallAllyWarItem,
    Camera,
    CameraItem,
    CantCreateOrJoinFactionVassal,
    CapitalMapIcon,
    CasusBelliItem,
    CasusBelliTitleItem,
    CasusBelliType,
    CatalystEntry,
    CatalystHistory,
    CatalystType,
    ChangeGHWTargetWindow,
    ChangeGHWTargetWindowTitleItem,
    CharacterFilterPreset,
    CharacterFinderWindow,
    CharacterFocusWindow,
    CharacterInteraction,
    CharacterInteractionCategory,
    CharacterInteractionConfirmationWindow,
    CharacterInteractionEffectValues,
    CharacterInteractionMenuWindow,
    CharacterInteractionNotificationWindow,
    CharacterItem,
    CharacterLifestyleWindow,
    CharacterListFilter,
    CharacterListFilterCategory,
    CharacterListFilterOption,
    CharacterListItem,
    CharacterListSortItem,
    CharacterMemoryType,
    CharacterPoolWatchWindow,
    CharacterProperties,
    CharacterSelectionList,
    CharacterWindow,
    Claim,
    ClaimTitleItem,
    ClaimantEntry,
    ClaimantSortOption,
    ClaimantsWindow,
    CloudSaveData,
    CoatOfArmsDesigner,
    CoatOfArmsDesignerBackgroundPanel,
    CoatOfArmsDesignerEmblemInstance,
    CoatOfArmsDesignerEmblemInstancesPanel,
    CoatOfArmsDesignerEmblemLayout,
    CoatOfArmsDesignerEmblemLayoutPanel,
    CoatOfArmsDesignerEmblemTexture,
    CoatOfArmsDesignerPagedEmblemInstances,
    CoatOfArmsDesignerPaletteColor,
    CoatOfArmsDesignerPattern,
    CollapsibleCultureList,
    CollapsibleCultureListGroup,
    CollapsibleReligionList,
    CollapsibleReligionListGroup,
    ColorGenePicker,
    CombatEffect,
    CombatMaaItem,
    CombatMapIcon,
    CombatPredictionEdge,
    CombatPredictionMapIcon,
    CombatResultData,
    CombatRollModifiers,
    CombatSide,
    CombatSideModifierItem,
    CombatSideResultData,
    CombatWindow,
    ConcubineInfo,
    ConcubineInteractionWindow,
    CouncilPositionType,
    CouncilTaskInteractionItem,
    CouncilTaskInteractionWindow,
    CouncilTaskType,
    CouncilWindow,
    County,
    CountyGroup,
    CourtAmenitiesCategoryItem,
    CourtAmenitiesSetting,
    CourtAmenitiesSettingItem,
    CourtAmenitiesWindow,
    CourtEventItem,
    CourtEventWindow,
    CourtGrandeurData,
    CourtGrandeurLevel,
    CourtGrandeurWindow,
    CourtPosition,
    CourtPositionCategory,
    CourtPositionType,
    CourtPositionsWindow,
    CourtSceneEditorWindow,
    CourtSceneSettings,
    CourtToolset,
    CourtType,
    CourtTypeSettingItem,
    CourtTypeWindow,
    CourtWindow,
    CreateClaimantFactionAgainstWindow,
    CreateFactionItem,
    CreditPortraitData,
    CultureAesthetics,
    CultureEra,
    CultureEraType,
    CultureInnovation,
    CultureInnovationType,
    CulturePillar,
    CultureReformation,
    CultureTemplate,
    CultureTradition,
    CultureWindow,
    DeJureVassalGroupItem,
    DebugTutorialChainItem,
    DebugTutorialLessonItem,
    DebugTutorialStepItem,
    DebugTutorialWindow,
    Decision,
    DecisionDetailView,
    DecisionViewWidget,
    DecisionViewWidgetCreateHolyOrder,
    DecisionViewWidgetOptionList,
    DecisionsView,
    DecisionsViewItem,
    DeclareWarInteractionWindow,
    DesignateHeirWindow,
    DesignerCoA,
    DiplomacyItem,
    DiplomaticItem,
    DivergenceWindow,
    Dlc,
    DlcCollection,
    DlcInfoGui,
    DlcItem,
    DoctrineGroupWindow,
    DoctrineGroupingFetcher,
    DoctrineGroupingFetcher2,
    DuchyGroup,
    DynastyCustomizationWindow,
    DynastyHouseIcon,
    DynastyHouseMembersWindow,
    DynastyHouseTemplate,
    DynastyHouseView,
    DynastyLegacy,
    DynastyLegacyItem,
    DynastyPerk,
    DynastyPerkConfirmation,
    DynastyTemplate,
    DynastyTreeItem,
    DynastyTreeView,
    DynastyView,
    EmitterNodeWindow,
    EmployedPositionItem,
    EmptyCourtPosition,
    Entry,
    EventChainProgressEntry,
    EventInfo,
    EventOption,
    EventWindow,
    EventWindowCustomWidgetStruggleInfo,
    EventWindowData,
    EventWindowWidget,
    EventWindowWidgetChainProgress,
    EventWindowWidgetEnterText,
    EventWindowWidgetEnterTextDefaultEntry,
    EventWindowWidgetNameCharacter,
    FactionCharacterMember,
    FactionCountyMember,
    FactionItem,
    FactionsWindow,
    FaithConversionWindow,
    FaithCreationWindow,
    FaithDoctrine,
    FaithDoctrineGroup,
    FaithWindow,
    FeedMessageItem,
    FilterPresetItem,
    FindTitleView,
    FindVassalListWindow,
    Fleet,
    FleetPredictionMapIcon,
    Focus,
    FocusType,
    FrontEndLoadView,
    FrontEndMainView,
    FrontEndView,
    GUIAlertItem,
    GUIBuildingItem,
    GUICountyHolding,
    GUIPotentialBuildingItem,
    GUITrackItem,
    GameConceptTooltip,
    GameDialog,
    GameMpSetup,
    GameRule,
    GameRuleSetting,
    GameSetup,
    GeographicalRegion,
    GovernmentType,
    GovernmentTypeHeader,
    GrantTitleOffer,
    GrantTitlesInteractionWindow,
    GraphInterfaceNodeWindow,
    GreatHolyWarParticipant,
    GreatHolyWarParticipantScore,
    GreatHolyWarWindow,
    GuiAIWarCoordinator,
    GuiActionImportantActionItem,
    GuiActionItem,
    GuiClaimant,
    GuiCouncilPosition,
    GuiCultureEra,
    GuiCultureEraGroup,
    GuiCultureInnovation,
    GuiCultureTradition,
    GuiFaithCreationDoctrineItem,
    GuiFaithDoctrineItem,
    GuiFaithIcon,
    GuiGameRule,
    GuiGameRulePreset,
    GuiHolySiteItem,
    GuiLaw,
    GuiLawGroup,
    GuiMilitaryStrength,
    GuiPotentialCouncilTask,
    GuiUnitInfo,
    GuiVirtueOrSinItem,
    HairColorItem,
    HiredTroop,
    HiredTroopDetailView,
    HiredTroopItem,
    HiredTroopRegiment,
    HistoryEntry,
    Holding,
    HoldingItem,
    HoldingType,
    HoldingTypeItem,
    HoldingView,
    HolySite,
    HolySiteIcon,
    Hook,
    HouseCustomizationWindow,
    HouseOrderOption,
    HudBottomWidget,
    HybridizationWindow,
    Illustration,
    ImagePopup,
    ImportantActionItem,
    ImportantActionType,
    InFrontTopBar,
    InGameBottomBar,
    InGameTopbar,
    InspirationType,
    InspirationsWindow,
    InteractionCategoryItem,
    InteractionContainer,
    InteractionEffectsDescription,
    InteractionItem,
    InteractionOtherEffect,
    InteractionSchemeInfo,
    InteractionTitleItem,
    InterfereInWarInteractionNotificationWindow,
    InterfereInWarInteractionWindow,
    InterfereInWarWarItem,
    IntrigueWindow,
    IntrigueWindowHookItem,
    IntrigueWindowSecretGroup,
    IntrigueWindowSecretItem,
    Inventory,
    InventorySlot,
    InventorySlotType,
    InventoryView,
    InviteCreateClaimantFactionOffer,
    JominiGameRules,
    KillListWindow,
    KnightsView,
    LandedTitpleTemplate,
    LanguageWindow,
    Law,
    LawGroup,
    LawItem,
    LayeredIcon,
    LeaseOutBaroniesWindow,
    LeaseOutBaroniesWindowTitleItem,
    LegacyItem,
    LevyView,
    Lifestyle,
    Light,
    LoadIngameWindow,
    LocalPlayerCachedData,
    LocalPlayerCourtEvents,
    LocalPlayerNewArtifacts,
    MAAItem,
    MapContentPropertyGroupsGui,
    MapMode,
    MarriageInfo,
    MarriageInteractionNotificationWindow,
    MarriageInteractionWindow,
    MarriageOffer,
    MatchOffer,
    MatchmakerInteractionWindow,
    MatchmakerTraitInfo,
    MemoriesWindow,
    MemoryInfo,
    MenAtArmsType,
    MenAtArmsTypeView,
    MenAtArmsTypeViewTypeItem,
    MenAtArmsView,
    MessageFeedHandler,
    MessageType,
    MetaInfoWidget,
    MilitaryItem,
    MilitaryView,
    MilitaryViewEventTroop,
    ModifierItem,
    ModifyVassalContractInteractionWindow,
    ModifyVassalContractInteractionWindowObligationLevelOption,
    MpBookmarkItem,
    MyRealmWindow,
    MyRealmWindowVassalItem,
    Nickname,
    NodeEditorSearch,
    ObligationContainerData,
    ObligationLevel,
    ObligationLevelCheckbox,
    ObligationLevelLineConnection,
    ObligationLevelLineItem,
    ObligationLevelLineTree,
    ObligationLevelRadioButtons,
    OptionEffectItem,
    OptionItem,
    OrderFaithOption,
    Outliner,
    OutlinerHoldingItem,
    OutlinerPlayer,
    ParticipantInfo,
    ParticleUserData,
    PatternItem,
    PauseMenu,
    Perk,
    PerkGuiTree,
    PerkLineConnection,
    PerkLineItem,
    PlaceRallyPoint,
    PlayerMessageItem,
    PlayerValueItem,
    PortraitCustomizationWindow,
    PotentialAgentWindow,
    PotentialCouncillorWindow,
    PotentialElector,
    PotentialFactionMemberWindow,
    PotentialTaskLocationWindow,
    ProgressInterface,
    ProvinceIcon,
    ProvinceMovementAttritionIcon,
    Raid,
    RaidWindow,
    RallyPoint,
    RallyPointItem,
    RallyPointMapIcon,
    RallyPointWindow,
    RankedRoyalCourtItem,
    ReasonItem,
    ReforgeArtifactWindow,
    Regiment,
    RegimentCombatStats,
    RegimentReorgEntry,
    RegimentTerrainModifier,
    RegimentWinterModifier,
    ReligionFamily,
    ReligionWindow,
    RenamePopup,
    ReplacePillarWindow,
    ResignConfirmationWindow,
    RevokeTitlesInteractionWindow,
    RoyalCourtScreenshotWindow,
    RoyalCourtWindow,
    RulerDesignerLoadWindow,
    RulerDesignerPortraitModifier,
    RulerDesignerSaveWindow,
    RulerDesignerSkill,
    RulerDesignerWindow,
    SAICBTypeInfo,
    SAISchemeTypeInfo,
    SAIStrategyInfo,
    SAIValueInfo,
    SaveGame,
    SaveGameAnalyzer,
    SaveGameBlockData,
    SaveGameConfigView,
    SaveGameItem,
    SaveGameListView,
    SaveGameWindow,
    SaveRulerItem,
    SaveRulerSkillGui,
    SaveRulerTraitGui,
    SchemeAgentItem,
    SchemeItem,
    SchemeType,
    ScriptedRelation,
    SearchListNodeWindow,
    SecretType,
    SelectParticleUserDataDialog,
    SelectableTaskLocation,
    SelectedRallyPointItem,
    SelectedUnitItem,
    Siege,
    SiegeWindow,
    SkillItem,
    SkillSchemeGroup,
    StaticModifier,
    StoryInfo,
    StraitMapIcon,
    StruggleInvolvementWindow,
    StrugglePhase,
    StruggleType,
    StruggleWindow,
    SuccessionElectionWindow,
    SuccessionElectionWindowCandidate,
    SuccessionElectionWindowElector,
    SuccessionElectionWindowElectorVote,
    SuccessionElector,
    SuccessionEventWindow,
    SuccessionEventWindowLostTitlesItem,
    SuccessionLawChangeWindow,
    SuggestionItem,
    SuggestionType,
    Terrain,
    TimelineWidget,
    TitleAddLawWindow,
    TitleCapitalMapIcon,
    TitleCustomizationWindow,
    TitleHistory,
    TitleHistoryWindow,
    TitleItem,
    TitleSuccessionItem,
    TitleViewWindow,
    ToastMessageHandler,
    TokenParameter,
    TooltipInfo,
    TraditionGrouping,
    Trait,
    TraitGroup,
    TraitSlot,
    TraitSlotArray,
    TransferVassalWindow,
    TroopItem,
    Tutorial,
    TutorialWindow,
    UndoStack,
    UnitItem,
    UnitMapIcon,
    UnitMapIconHandler,
    ValueBreakdown,
    VariableInfo,
    VassalContract,
    VassalContractType,
    VassalConversionWindow,
    WarAllyItem,
    WarDeclaredOverviewWindow,
    WarInfo,
    WarItem,
    WarOverviewWindow,
    WarParticipantItem,
    WarResultsWindow,
    WatchWindow,
}
