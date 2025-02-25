#[derive(Copy, Clone, Debug, Eq, PartialEq, Display, EnumString)]
pub enum Datatype {
    Unknown,
    AnyScope,
    AIAttitude,
    AIStrategy,
    Achievement,
    AchievementPopup,
    AchievementWindow,
    AddFriendWindow,
    AddWarGoalPanel,
    AgitatorSlot,
    Alert,
    AlertGroup,
    AnchorItem,
    Application,
    Attribute,
    AvailabilityEntry,
    Battle,
    BattleBuildingEntry,
    BattleCondition,
    BattleMarker,
    BattlePanel,
    BattleParticipant,
    BattleParticipantsPanel,
    BlurThreshold,
    BrowserWindow,
    BrushBool,
    BrushFloat,
    BrushSettings,
    BrushSettingsDropdown,
    BrushSettingsGlobal,
    BudgetPanel,
    Building,
    BuildingDetailsPanel,
    BuildingMarker,
    BuildingPotentialMarker,
    BuildingType,
    CFixedPoint,
    CPdxEnumValue,
    CPdxFloatRect,
    CPdxInputBindingSetting,
    CPdxIntRect,
    CString,
    CUTF8String,
    CVector2f,
    CVector2i,
    CVector3f,
    CVector3i,
    CVector4f,
    CVector4i,
    CanalType,
    ChangeLawPanel,
    ChangeProductionMethodMenuItem,
    ChangeProductionMethodsMenuItems,
    Character,
    CharacterInteraction,
    CharacterTrait,
    Chat,
    ChatMessage,
    ChatNotificationMessage,
    ChatTab,
    ChatWindow,
    ChildGenerator,
    ChildItem,
    Cities,
    CityMarker,
    CivilWar,
    CloudSaveData,
    CoatOfArms,
    CoatOfArmsPreviewWindow,
    ColonyMarker,
    CombatUnit,
    Command,
    CommanderOrder,
    CommanderOrderMarker,
    CommanderOrderMarkerListItem,
    CommanderOrderType,
    CommanderPanel,
    CommanderRank,
    ConfirmationWindow,
    ConsoleMenuItem,
    ConsoleWindow,
    ConstructionQueueElement,
    Container,
    ContextMenuItem,
    ContextualDiplomaticActionType,
    ContextualDiplomaticPact,
    CountriesPanel,
    Country,
    CountryCreation,
    CountryDefinition,
    CountryEntry,
    CountryFormation,
    CountryFormationPanel,
    CountryPanel,
    CountryTrendPair,
    CreateSocialProfile,
    CreateSocialProfileWindow,
    CreditsWindow,
    Culture,
    CultureCasualtyStatistics,
    CultureInfoPanel,
    CurveEditor,
    CurvePoint,
    DataDynamicTrend,
    DataTrend,
    DatatypesExplorer,
    Date,
    Decision,
    Decree,
    DecreeMarker,
    DecreeType,
    DetailData,
    DiplomaticAction,
    DiplomaticActionType,
    DiplomaticActionWindow,
    DiplomaticPact,
    DiplomaticPlay,
    DiplomaticPlayConfirmation,
    DiplomaticPlayPanel,
    DiplomaticPlayType,
    DiplomaticRelations,
    DiscriminationTrait,
    Dlc,
    DockableLayout,
    DockableLayoutManager,
    DockableWindow,
    DrawCmdsList,
    DrawCmdsViewer,
    Election,
    ElectionPanel,
    EmitterNodeWindow,
    EmployeeTransfer,
    Encyclopedia,
    EncyclopediaEntry,
    EncyclopediaEntryView,
    EncyclopediaPage,
    EndPrepConfirm,
    EntityEditor,
    EntityEditorAudioEventHandler,
    EntityEditorEventLayer,
    EntityEditorEventLayerPtr,
    EntityEditorKeyframe,
    EntityEditorTimelineState,
    EntityViewerProperties,
    EnumSettingEntry,
    Era,
    ErrorDeer,
    ErrorMessageBox,
    Ethnicity,
    EthnicityItem,
    Event,
    EventIcon,
    EventLayerForEntityEditor,
    EventMarker,
    EventMarkerListItem,
    EventOption,
    EventTargetSetupContext,
    EventWindow,
    ExilePool,
    ExportTool,
    EyeDropper,
    EyeDropperPackedSample,
    FeedMessageItem,
    Friend,
    FriendListWindow,
    FriendRequest,
    FriendSearchResult,
    Friends,
    Front,
    FrontEndCreditsView,
    FrontEndLoadConfirmationWindow,
    FrontEndLoadView,
    FrontEndMainView,
    FrontEndMultiplayerView,
    FrontMarker,
    FrontPanel,
    FrontParticipant,
    GUIAchievement,
    GameConceptType,
    GameMpSetup,
    GameOverScreen,
    GameRule,
    GameRuleSetting,
    GameSetup,
    GarrisonMarker,
    GeneCategory,
    GeneItem,
    GeneTemplate,
    GenerationItem,
    GfxSkin,
    Goods,
    GoodsExpenseItem,
    GoodsPanel,
    GoodsPanelValue,
    GoodsProductionInfo,
    GoodsStatePanel,
    GoodsUsagePanel,
    GovernmentType,
    Graph,
    GraphInterfaceNodeWindow,
    GraphPanel,
    Group,
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
    GuiAnimationEditorRuler,
    GuiAnimationEditorRulerResolutionEntry,
    GuiAnimationEditorUniversalTrack,
    GuiAnimationEditorViewportBase,
    GuiAnimationEditorViewportUserInput,
    GuiAnimationTimelineViewport,
    GuiContext,
    GuiEditor,
    GuiEditorCategory,
    GuiEditorDockable,
    GuiEditorOutliner,
    GuiEditorProperties,
    GuiEditorProperty,
    GuiEditorTooltip,
    GuiGameRule,
    GuiGameRulePreset,
    HQMarker,
    HQPanel,
    Heightmap,
    HeightmapPainter,
    HeightmapPainterMode,
    HeightmapResolution,
    HighlightManager,
    HomelandMarker,
    Hq,
    Ideology,
    ImportTool,
    Importable,
    ImportableGroup,
    ImportantAlert,
    InfoboxNodeWindow,
    InformationPanel,
    InformationPanelBar,
    IngameHud,
    InputActionBinding,
    InspectorPanel,
    Institution,
    InstitutionInvestmentLevel,
    InstitutionType,
    Interest,
    InterestGroup,
    InterestGroupAndTraitPair,
    InterestGroupMarker,
    InterestGroupPanel,
    InterestGroupTrait,
    JominiGUISetting,
    JominiGameRules,
    JominiNotification,
    JominiNotificationOverlay,
    JominiPasswordPopup,
    JominiServer,
    JominiServerBrowserGui,
    JominiSettingsWindow,
    JournalEntry,
    JournalEntryPanel,
    JournalEntryType,
    JournalPanel,
    KeyframeEditor,
    KeyframeEventEditor,
    KeyframeWidget,
    Label,
    LabelingHelper,
    LanguageEntry,
    Law,
    LawGroup,
    LawType,
    Layer,
    LayerTreeItem,
    LegitimacyLevel,
    LensOption,
    LensTab,
    LensToolbar,
    LoadIngameWindow,
    LoadingScreenManager,
    LobbyPlayer,
    LobbyView,
    Location,
    LocationFinder,
    LogEntry,
    LogViewer,
    LogViewerCategory,
    LogViewerEntry,
    LogViewerType,
    LoyaltyType,
    MPConfig,
    MapContentEditorMode,
    MapContentEditorOptions,
    MapContentEditorViewport,
    MapContentEntryDesc,
    MapContentLayerDesc,
    MapContentPanel,
    MapContentPropertyGroup,
    MapContentPropertyGroupsGui,
    MapContentSelector,
    MapContentSelectorGui,
    MapEditor,
    MapEditorGui,
    MapEditorLayerBorder,
    MapEditorLayerBorderDockable,
    MapInteraction,
    MapInteractionManager,
    MapListActivateConscriptionCenterOption,
    MapListActivateConscriptionCenterPanel,
    MapListAdvanceFrontOption,
    MapListAdvanceFrontPanel,
    MapListBuildingOption,
    MapListBuildingPanel,
    MapListColonyOption,
    MapListCountriesPanel,
    MapListCountryOption,
    MapListDecreeOption,
    MapListDecreePanel,
    MapListDiploActionOption,
    MapListHQOption,
    MapListHQPanel,
    MapListInterestGroupOption,
    MapListInterestGroupsPanel,
    MapListMarketOption,
    MapListMarketsPanel,
    MapListOption,
    MapListPanel,
    MapListPanelManager,
    MapListStateOption,
    MapListStatePanel,
    MapListStrategicRegionOption,
    MapListStrategicRegionsPanel,
    MapListTradeRouteOption,
    MapNotification,
    MapObjectMask,
    MapObjectPainter,
    MapObjectPainterMode,
    MapObjectPainterOptions,
    MapObjectTool,
    Market,
    MarketGoods,
    MarketPanel,
    MarketsMarker,
    MarketsMarkerListItem,
    MaskEntry,
    MaskManagerEntry,
    MaskPainterDynamicTerrain,
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
    MessageFeedHandler,
    MessageSettingsItem,
    MessageSettingsWindow,
    MilitaryPanelBuildingEntry,
    MilitaryPanelCommanderEntry,
    MilitaryPanelHQEntry,
    MixBrushMode,
    Modifier,
    ModifierEntry,
    ModifierNodeData,
    ModifierNodeDebuggerView,
    ModifierNodeDetailsView,
    ModifierNodeGraphItem,
    ModifierNodeGraphLine,
    ModifierNodeListView,
    ModifiersPanel,
    MoveTool,
    MultiplayerSetupWindow,
    MultiplayerSynchronizationInfo,
    MusicPlayer,
    MusicPlayerCategory,
    MusicTrack,
    NavalInvasion,
    Node,
    NodeEditorSearch,
    NodeError,
    NodeLine,
    NodePin,
    NodeWindow,
    NonRegisteredDockable,
    NotificationDummyContext,
    NotificationOptionItem,
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
    Objective,
    ObjectiveType,
    OosData,
    OosWindow,
    OpinionMarker,
    OutgoingFriendRequest,
    Outliner,
    OutlinerEntry,
    OutputEntry,
    POPSCreateAccount,
    POPSLoginView,
    POPSStatusWidget,
    PanelMilitary,
    ParametricSelect,
    ParticleUserData,
    Party,
    PartyPanel,
    PastBattleMarker,
    PauseMenu,
    PdxCoreSetting,
    PdxGuiFoldOut,
    PdxGuiGfxVideoControl,
    PdxGuiTableRow,
    PdxGuiTreeTable,
    PdxGuiWidget,
    PdxSetting,
    PdxSettingsWindow,
    PdxSettingsWindowCategory,
    PdxSettingsWindowScopedCategory,
    PdxValueSetting,
    PieTimer,
    PieTimerSlice,
    Playable,
    Player,
    PlayerJoinRequest,
    PlayerMessageItem,
    PlotLine,
    PoliticalMovement,
    PoliticalMovementPanel,
    PoliticsPanel,
    Pop,
    PopConsumptionGoods,
    PopDetailsPanel,
    PopList,
    PopListItem,
    PopNeed,
    PopType,
    PopWithIG,
    PopsOverviewPanel,
    PopulationGrouping,
    PopupManager,
    Portrait3dView,
    PortraitDataContext,
    PortraitEditorAnimationItem,
    PortraitEditorWindow,
    PortraitTooltip,
    PreviewMaskTexture,
    ProductionMarker,
    ProductionMethod,
    ProductionMethodGroup,
    ProductionMethodsPanel,
    ProductionMethodsPanelEntry,
    ProgressInterface,
    Proposal,
    Province,
    RandomizableValueFloat,
    RandomizableValueInt,
    ReformGovernment,
    ReleaseCountryWindow,
    Religion,
    ReligionInfoPanel,
    RemoveFriendConfirmWindow,
    RepackWindow,
    ResignConfirmationWindow,
    RevolutionaryMovementMarker,
    RightClickMenuManager,
    Savable,
    SavableGroup,
    SaveDialog,
    SaveGame,
    SaveGameAnalysisView,
    SaveGameAnalyzer,
    SaveGameBlockData,
    SaveGameConfigView,
    SaveGameItem,
    SaveGameListView,
    SaveGameWindow,
    Scope,
    ScopeDebugData,
    ScopeDebugInspectorPlugin,
    ScopeObjectEditor,
    ScopeObjectProvider,
    ScopeObjectType,
    ScopedJominiSettingsCategory,
    ScopedJominiSettingsPage,
    ScriptRunnerInspector,
    ScriptRunnerResult,
    ScriptedButton,
    ScriptedGui,
    SeaNodeMarker,
    SeaRegionInfraMarker,
    SeaRegionPanel,
    SearchListNodeWindow,
    SelectParticleUserDataDialog,
    SelectTool,
    SelectionHistory,
    SelectionLine,
    ServerInformation,
    SettingCategory,
    SettingsPage,
    ShippingLane,
    SkinEditor,
    SmartBrushHeightRange,
    SmartBrushPattern,
    SmartBrushPresetManager,
    SmartMaterialPaintingMode,
    Social,
    SocialNotificationWindow,
    SocialUI,
    SocialWidget,
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
    State,
    StateInfraMarker,
    StateMarker,
    StatePopulationPanel,
    StateRegion,
    StateTrait,
    StatesPanel,
    StrategicRegion,
    StrategicRegionMarker,
    SupplyNetworkEntry,
    SwayCountryPanel,
    TaxBurdenItem,
    TechTreeItem,
    TechTreeLine,
    TechTreePanel,
    Technology,
    TechnologyUnlock,
    TerrainToolButton,
    TextureEntry,
    TextureImporter,
    TextureList,
    TextureListDirectory,
    TextureListTexture,
    TextureNodeWindow,
    TextureViewer,
    Theater,
    TickTaskData,
    TickTaskDebuggerView,
    TickTaskDetailsView,
    TickTaskGraphItem,
    TickTaskGraphLine,
    TickTaskListView,
    TimeKeeper,
    TimedModifier,
    TimelineKeyframe,
    ToastMessageHandler,
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
    TooltipInfo,
    TopFrontend,
    TopScope,
    TradeRoute,
    Truce,
    Tutorial,
    Tweakable,
    TweakableCategory,
    TweakableUiEntry,
    Tweaker,
    Type,
    UndoStack,
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
    VariableSystem,
    Vec4f,
    ViewerEntity,
    ViewerEntityLodInfo,
    ViewerEntityState,
    ViewerEntityStatePtr,
    War,
    WarGoal,
    WarGoalMarker,
    WarGoalPanelPair,
    WarGoalType,
    WarManager,
    WarPanel,
    WarParticipant,
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
}
