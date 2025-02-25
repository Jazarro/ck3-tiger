&[
    ("Abs_CFixedPoint", Args(&[DType(CFixedPoint)]), CFixedPoint),
    ("Abs_float", Args(&[DType(float)]), float),
    ("Abs_int32", Args(&[DType(int32)]), int32),
    ("Abs_int64", Args(&[DType(int64)]), int64),
    ("AccessConstructionSpeeds", Args(&[]), Unknown),
    ("AccessMapEditorLayerBorders", Args(&[]), Unknown),
    ("AccessMetaPlayer", Args(&[]), Unknown),
    ("AccessPlayer", Args(&[]), Unknown),
    ("AddLocalizationIf", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("AddTextIf", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("Add_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), CFixedPoint),
    ("Add_CVector2f", Args(&[DType(CVector2f), DType(CVector2f)]), CVector2f),
    ("Add_float", Args(&[DType(float), DType(float)]), float),
    ("Add_int32", Args(&[DType(int32), DType(int32)]), int32),
    ("Add_int64", Args(&[DType(int64), DType(int64)]), int64),
    ("Add_uint32", Args(&[DType(uint32), DType(uint32)]), uint32),
    ("Add_uint64", Args(&[DType(uint64), DType(uint64)]), uint64),
    ("And", Args(&[DType(Unknown), DType(Unknown)]), bool),
    ("AreAchievementsAvailable", Args(&[]), bool),
    ("BindFoldOutContext", Args(&[]), void),
    ("BoolTo1And2", Args(&[DType(Unknown)]), int32),
    ("BoolTo2And1", Args(&[DType(Unknown)]), int32),
    ("CalcDynTrendLatestDate", Args(&[DType(Unknown)]), CString),
    ("CalcDynTrendOldestDate", Args(&[DType(Unknown)]), CString),
    ("CanEditSettingsAfterHost", Args(&[]), bool),
    ("CanGetAchievements", Args(&[]), bool),
    ("CanOpenLobby", Args(&[]), bool),
    ("CanOpenRightClickContextMenu", Args(&[]), bool),
    ("ChangeAllConstructionSpeed", Args(&[DType(Unknown)]), Command),
    ("ChangeAllConstructionSpeedTooltip", Args(&[DType(Unknown)]), CString),
    ("ClearDynamicTerrainOverrideForAllProvinces", Args(&[]), void),
    ("ClearHostError", Args(&[]), void),
    ("CloseAllTooltips", Args(&[]), void),
    ("ConcatIfNeitherEmpty", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("Concatenate", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("Concept", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("CopyBuildVersionInfoToClipboard", Args(&[]), void),
    ("CopyServerID", Args(&[]), void),
    ("CurrentAndMaxToProgressbarValueInt32", Args(&[DType(Unknown), DType(Unknown)]), float),
    ("DataModelFirst", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("DataModelHasItems", Args(&[DType(Unknown)]), bool),
    ("DataModelLast", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("DataModelSkipFirst", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("DataModelSkipLast", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("DataModelSubSpan", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), Unknown),
    ("DebugGetMapMode", Args(&[]), CString),
    ("DoubleToFloat", Args(&[DType(Unknown)]), float),
    ("EqualTo_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("EqualTo_CVector2f", Args(&[DType(CVector2f), DType(CVector2f)]), bool),
    ("EqualTo_float", Args(&[DType(float), DType(float)]), bool),
    ("EqualTo_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("EqualTo_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("EqualTo_string", Args(&[DType(CString), DType(CString)]), bool),
    ("EqualTo_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("EqualTo_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("Execute", Args(&[DType(Unknown)]), void),
    ("ExecuteConsoleCommand", Args(&[DType(Unknown)]), void),
    ("ExecuteConsoleCommands", Args(&[DType(Unknown)]), void),
    ("ExecuteConsoleCommandsForced", Args(&[DType(Unknown)]), void),
    ("FixedPointToFloat", Args(&[DType(Unknown)]), float),
    ("FixedPointToInt", Args(&[DType(Unknown)]), int32),
    ("FixedPointToProgressbarValue", Args(&[DType(Unknown)]), float),
    ("FocusCameraOnCityOutOfState", Args(&[]), void),
    ("GameHasMultiplePlayers", Args(&[]), bool),
    ("GameIsMultiplayer", Args(&[]), bool),
    ("GetAchievementsAvailableString", Args(&[]), CString),
    ("GetApprovalRatingFromValue", Args(&[DType(Unknown)]), CString),
    ("GetArrowIcon_CFixedPoint", Args(&[DType(Unknown)]), CString),
    ("GetArrowIcon_int32", Args(&[DType(Unknown)]), CString),
    ("GetArrowIcon_int64", Args(&[DType(Unknown)]), CString),
    ("GetAutosaveName", Args(&[]), CString),
    ("GetBattleCondition", Args(&[DType(Unknown)]), Unknown),
    ("GetBuildVersionDescriptionWithClickToCopy", Args(&[]), CString),
    ("GetBuildingType", Args(&[DType(Unknown)]), Unknown),
    ("GetCanGoToFrontend", Args(&[]), bool),
    ("GetCanGoToFrontendDesc", Args(&[]), CString),
    ("GetConfirmationDesc", Args(&[DType(Unknown)]), CString),
    ("GetConstructionSectorType", Args(&[]), Unknown),
    ("GetCurrentFps", Args(&[]), float),
    ("GetCurrentLoadingScreen", Args(&[]), Unknown),
    ("GetDataModelSize", Args(&[DType(Unknown)]), int32),
    ("GetDecreeType", Args(&[DType(Unknown)]), Unknown),
    ("GetDefaultServerName", Args(&[]), CUTF8String),
    ("GetDefine", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("GetDefineAtIndex", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), Unknown),
    ("GetDesc", Args(&[DType(Unknown)]), CString),
    ("GetDescriptiveText", Args(&[DType(Unknown)]), CString),
    ("GetDiplomaticActionType", Args(&[DType(Unknown)]), Unknown),
    ("GetDiplomaticPlayType", Args(&[DType(Unknown)]), Unknown),
    ("GetDlcCollection", Args(&[]), Unknown),
    ("GetDynTrendMax", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetDynTrendMin", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetDynTrendPlotPoints", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), Unknown),
    ("GetEnumIndex", Args(&[DType(Unknown)]), CVector2i),
    ("GetEthnicities", Args(&[]), Unknown),
    ("GetGameVersionDisplay", Args(&[]), CString),
    ("GetGameVersionInfo", Args(&[]), CString),
    ("GetGameVersionInfoShort", Args(&[]), CString),
    ("GetGlobalList", Args(&[DType(Unknown)]), Unknown),
    ("GetGlobalMaxLandDefense", Args(&[]), CFixedPoint),
    ("GetGlobalMaxLandOffense", Args(&[]), CFixedPoint),
    ("GetGlobalMaxNavalDefense", Args(&[]), CFixedPoint),
    ("GetGlobalMaxNavalOffense", Args(&[]), CFixedPoint),
    ("GetGlobalVariable", Args(&[DType(Unknown)]), Unknown),
    ("GetGoods", Args(&[DType(Unknown)]), Unknown),
    ("GetGuiPositionFromPercentCoordinates", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), Unknown),
    ("GetHostError", Args(&[]), CUTF8String),
    ("GetIdeology", Args(&[DType(Unknown)]), Unknown),
    ("GetInstitutionType", Args(&[DType(Unknown)]), Unknown),
    ("GetInteger", Args(&[DType(Unknown)]), int32),
    ("GetInterestGroupVariant", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("GetInvParentScale", Args(&[DType(Unknown)]), float),
    ("GetIsChecked", Args(&[DType(Unknown)]), bool),
    ("GetLargeFlagFrameSize", Args(&[]), CVector2i),
    ("GetLatestDate", Args(&[DType(Unknown)]), CString),
    ("GetLawGroup", Args(&[DType(Unknown)]), Unknown),
    ("GetLawType", Args(&[DType(Unknown)]), Unknown),
    ("GetLegitimacyLevel", Args(&[DType(Unknown)]), Unknown),
    ("GetLegitimacyLevels", Args(&[]), Unknown),
    ("GetLensToolbar", Args(&[]), Unknown),
    ("GetLoadingScreenManager", Args(&[]), Unknown),
    ("GetLocalizedDefine", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("GetLocalizedDefineAtIndex", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), CString),
    ("GetLoyaltyTypes", Args(&[]), Unknown),
    ("GetMPChecksum", Args(&[]), CString),
    ("GetManpowerColorVec", Args(&[DType(Unknown)]), CVector4f),
    ("GetMapModeIndices", Args(&[]), Unknown),
    ("GetMapModeName", Args(&[DType(Unknown)]), CString),
    ("GetMax", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetMediumFlagFrameSize", Args(&[]), CVector2i),
    ("GetMetaPlayer", Args(&[]), Unknown),
    ("GetMin", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetMoraleColorString", Args(&[DType(Unknown)]), CString),
    ("GetMoraleColorVec", Args(&[DType(Unknown)]), CVector4f),
    ("GetNullProperty", Args(&[]), Unknown),
    ("GetNumberAbove_int32", Args(&[DType(int32), DType(int32)]), int32),
    ("GetOldestDate", Args(&[DType(Unknown)]), CString),
    ("GetOpenLobbyTooltip", Args(&[]), CString),
    ("GetPlayer", Args(&[]), Country),
    ("GetPlayersCount", Args(&[]), int32),
    ("GetPopType", Args(&[DType(Unknown)]), Unknown),
    ("GetPopTypesInLowerStrataDesc", Args(&[]), CString),
    ("GetPopTypesInMiddleStrataDesc", Args(&[]), CString),
    ("GetPopTypesInUpperStrataDesc", Args(&[]), CString),
    ("GetPortraitTextureFromDna", Args(&[DType(Unknown), DType(Unknown)]), Unknown),
    ("GetPrevTrendValue", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetPrevTrendValueFor", Args(&[DType(Unknown), DType(Unknown)]), CFixedPoint),
    ("GetRandomLogInfo", Args(&[]), CString),
    ("GetRankModifier", Args(&[DType(Unknown)]), Unknown),
    ("GetRawTextTooltipTag", Args(&[DType(Unknown)]), CString),
    ("GetRawTrendPlotRectWithFixedY", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), CPdxFloatRect),
    ("GetResolutionWithAspectRatio", Args(&[]), CVector2f),
    ("GetSelectedMapMode", Args(&[]), CVector2i),
    ("GetSelectedMapModeName", Args(&[]), CString),
    ("GetSmallFlagFrameSize", Args(&[]), CVector2i),
    ("GetSmartBrushInterpolationNames", Args(&[]), Unknown),
    ("GetSmartBrushPatternNames", Args(&[]), Unknown),
    ("GetStaticModifier", Args(&[IType(Item::Modifier)]), Unknown),
    ("GetStringSettingText", Args(&[DType(Unknown)]), CString),
    ("GetString_CPdxFloatRect", Args(&[DType(CPdxFloatRect)]), CString),
    ("GetString_CPdxIntRect", Args(&[DType(CPdxIntRect)]), CString),
    ("GetString_CVector2f", Args(&[DType(CVector2f)]), CString),
    ("GetString_CVector2i", Args(&[DType(CVector2i)]), CString),
    ("GetString_CVector3f", Args(&[DType(CVector3f)]), CString),
    ("GetString_CVector3i", Args(&[DType(CVector3i)]), CString),
    ("GetString_CVector4f", Args(&[DType(CVector4f)]), CString),
    ("GetString_CVector4i", Args(&[DType(CVector4i)]), CString),
    ("GetSupplyNetworkColorstring", Args(&[DType(Unknown)]), CString),
    ("GetTrendIcon", Args(&[DType(Unknown)]), CString),
    ("GetTrendIconFor", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("GetTrendPlotPoints", Args(&[DType(Unknown)]), Unknown),
    ("GetTrendPlotPointsAbsolute", Args(&[DType(Unknown)]), Unknown),
    ("GetTrendPlotPointsNormalized", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), Unknown),
    ("GetTrendValue", Args(&[DType(Unknown)]), CFixedPoint),
    ("GetTrendValueFor", Args(&[DType(Unknown), DType(Unknown)]), CFixedPoint),
    ("GetUnitStrengthHeader", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), CString),
    ("GetUnitStrengthHeaderNoIcon", Args(&[DType(Unknown), DType(Unknown)]), CString),
    ("GetVarTimeRemaining", Args(&[DType(Unknown), DType(Unknown)]), int32),
    ("GfxGetSkins", Args(&[]), Unknown),
    ("GfxSetActiveSkin", Args(&[DType(Unknown)]), void),
    ("GfxSkinIsActive", Args(&[DType(Unknown)]), bool),
    ("GoToFrontend", Args(&[]), void),
    ("GreaterThanOrEqualTo_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("GreaterThanOrEqualTo_float", Args(&[DType(float), DType(float)]), bool),
    ("GreaterThanOrEqualTo_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("GreaterThanOrEqualTo_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("GreaterThanOrEqualTo_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("GreaterThanOrEqualTo_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("GreaterThan_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("GreaterThan_float", Args(&[DType(float), DType(float)]), bool),
    ("GreaterThan_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("GreaterThan_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("GreaterThan_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("GreaterThan_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("HasAnySelections", Args(&[DType(Unknown)]), bool),
    ("HasCitiesOutOfState", Args(&[]), bool),
    ("HasDlcFeature", Args(&[DType(Unknown)]), bool),
    ("HasGameStartedForTheFirstTime", Args(&[]), bool),
    ("HasGameState", Args(&[]), bool),
    ("HasHostError", Args(&[]), bool),
    ("InDebugMode", Args(&[]), bool),
    ("InReleaseMode", Args(&[]), bool),
    ("IntToFixedPoint", Args(&[DType(Unknown)]), float),
    ("IntToFloat", Args(&[DType(Unknown)]), float),
    ("IntToFrameIndex", Args(&[DType(Unknown)]), int32),
    ("IntToUnsigned", Args(&[DType(Unknown)]), float),
    ("IsCameraRestrictionsEnabled", Args(&[]), bool),
    ("IsCurrentProductionMethod", Args(&[DType(Unknown)]), bool),
    ("IsDataModelEmpty", Args(&[DType(Unknown)]), bool),
    ("IsDrawCityDebugGrid", Args(&[]), bool),
    ("IsDrawCityDebugSpheres", Args(&[]), bool),
    ("IsDynTrendEmpty", Args(&[DType(Unknown)]), bool),
    ("IsDynamicTerrainAutoUpdate", Args(&[]), bool),
    ("IsDynamicsEnabled", Args(&[]), bool),
    ("IsEmpty", Args(&[DType(Unknown)]), bool),
    ("IsGameChecksumOk", Args(&[]), bool),
    ("IsGameOverScreenShown", Args(&[]), bool),
    ("IsGamePaused", Args(&[]), bool),
    ("IsHost", Args(&[]), bool),
    ("IsInGame", Args(&[]), bool),
    ("IsIronmanEnabled", Args(&[]), bool),
    ("IsLoadedFromSave", Args(&[]), bool),
    ("IsMapModeForced", Args(&[]), bool),
    ("IsMapModeSelected", Args(&[DType(Unknown)]), bool),
    ("IsObserver", Args(&[]), bool),
    ("IsPauseMenuShown", Args(&[]), bool),
    ("IsPlayerInLeft", Args(&[DType(Unknown)]), bool),
    ("IsPotential", Args(&[DType(Unknown)]), bool),
    ("IsPreparationLobby", Args(&[]), bool),
    ("IsSaving", Args(&[]), bool),
    ("IsTerrainManipulatorProvincesPreview", Args(&[]), bool),
    ("IsTutorialLessonActive", Args(&[DType(Unknown)]), bool),
    ("IsValid", Args(&[DType(Unknown)]), bool),
    ("IsZoomLevelFar", Args(&[]), bool),
    ("IsZoomLevelMid", Args(&[]), bool),
    ("IsZoomLevelNear", Args(&[]), bool),
    ("JoinText", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), CString),
    ("JominiAccessPlayerJoinRequests", Args(&[]), Unknown),
    ("JominiAreAchievementsAvailable", Args(&[]), bool),
    ("JominiGetAchievementsNotAvailableString", Args(&[]), CString),
    ("JominiGetMultiplayerAccessibleString", Args(&[]), CString),
    ("JominiHasPlayerJoinRequests", Args(&[]), bool),
    ("JominiIsHostOrLocal", Args(&[]), bool),
    ("JominiIsMultiplayerAccessible", Args(&[]), bool),
    ("JominiMultiplayerIsCrossplayEnabled", Args(&[]), bool),
    ("JominiMultiplayerIsCrossplayFilterAvailable", Args(&[]), bool),
    ("JominiPlayer", Args(&[]), Unknown),
    ("LessThanOrEqualTo_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("LessThanOrEqualTo_float", Args(&[DType(float), DType(float)]), bool),
    ("LessThanOrEqualTo_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("LessThanOrEqualTo_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("LessThanOrEqualTo_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("LessThanOrEqualTo_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("LessThan_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("LessThan_float", Args(&[DType(float), DType(float)]), bool),
    ("LessThan_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("LessThan_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("LessThan_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("LessThan_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("Localize", Args(&[DType(Unknown)]), CString),
    ("MakeScopeBool", Args(&[DType(Unknown)]), Scope),
    ("MakeScopeFlag", Args(&[DType(Unknown)]), Scope),
    ("MakeScopeValue", Args(&[DType(Unknown)]), Scope),
    ("Max_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), CFixedPoint),
    ("Max_float", Args(&[DType(float), DType(float)]), float),
    ("Max_int32", Args(&[DType(int32), DType(int32)]), int32),
    ("Max_int64", Args(&[DType(int64), DType(int64)]), int64),
    ("Max_uint32", Args(&[DType(uint32), DType(uint32)]), uint32),
    ("Max_uint64", Args(&[DType(uint64), DType(uint64)]), uint64),
    ("Min_CFixedPoint", Args(&[DType(Unknown), DType(Unknown)]), CFixedPoint),
    ("Min_float", Args(&[DType(Unknown), DType(Unknown)]), float),
    ("Min_int32", Args(&[DType(Unknown), DType(Unknown)]), int32),
    ("Min_int64", Args(&[DType(Unknown), DType(Unknown)]), int64),
    ("Min_uint32", Args(&[DType(Unknown), DType(Unknown)]), uint32),
    ("Min_uint64", Args(&[DType(Unknown), DType(Unknown)]), uint64),
    ("Multiply_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), CFixedPoint),
    ("Multiply_CVector2f", Args(&[DType(CVector2f), DType(CVector2f)]), CVector2f),
    ("Multiply_float", Args(&[DType(float), DType(float)]), float),
    ("Multiply_int32", Args(&[DType(int32), DType(int32)]), int32),
    ("Multiply_int64", Args(&[DType(int64), DType(int64)]), int64),
    ("Multiply_uint32", Args(&[DType(uint32), DType(uint32)]), uint32),
    ("Multiply_uint64", Args(&[DType(uint64), DType(uint64)]), uint64),
    ("NOP", Args(&[]), void),
    ("Nbsp", Args(&[]), CString),
    ("Negate_CFixedPoint", Args(&[DType(CFixedPoint)]), CFixedPoint),
    ("Negate_float", Args(&[DType(float)]), float),
    ("Negate_int32", Args(&[DType(int32)]), int32),
    ("Negate_int64", Args(&[DType(int64)]), int64),
    ("Not", Args(&[DType(Unknown)]), bool),
    ("NotEqualTo_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), bool),
    ("NotEqualTo_CVector2f", Args(&[DType(CVector2f), DType(CVector2f)]), bool),
    ("NotEqualTo_float", Args(&[DType(float), DType(float)]), bool),
    ("NotEqualTo_int32", Args(&[DType(int32), DType(int32)]), bool),
    ("NotEqualTo_int64", Args(&[DType(int64), DType(int64)]), bool),
    ("NotEqualTo_uint32", Args(&[DType(uint32), DType(uint32)]), bool),
    ("NotEqualTo_uint64", Args(&[DType(uint64), DType(uint64)]), bool),
    ("ObjectsEqual", Args(&[DType(Unknown), DType(Unknown)]), bool),
    ("ObserveCountryCommand", Args(&[DType(Unknown)]), Command),
    ("OnCreateAccount", Args(&[]), void),
    ("OnToggleLoginWindow", Args(&[]), void),
    ("OpenAchievements", Args(&[]), void),
    ("OpenGameRules", Args(&[]), void),
    ("Or", Args(&[DType(Unknown), DType(Unknown)]), bool),
    ("POPSStatusGetLoginStatus", Args(&[]), CUTF8String),
    ("POPSStatusIsAccountConnected", Args(&[]), bool),
    ("POPSStatusIsLoggedIn", Args(&[]), bool),
    ("POPSStatusIsLoggingIn", Args(&[]), bool),
    ("POPSStatusIsOffline", Args(&[]), bool),
    ("POPSStatusIsSupportConnectedAccount", Args(&[]), bool),
    ("POPStatusGetUserEmailMasked", Args(&[]), CUTF8String),
    ("POPStatusGetUserName", Args(&[]), CUTF8String),
    ("POPStatusIsUserNameEmpty", Args(&[]), bool),
    ("PauseAllConstruction", Args(&[]), Command),
    ("PauseAllConstructionTooltip", Args(&[]), CString),
    ("PdxClearEditBoxText", Args(&[DType(Unknown)]), void),
    ("PdxGetProfilerNames", Args(&[]), Unknown),
    ("PdxGetWidgetScreenSize", Args(&[DType(Unknown)]), CVector2f),
    ("PdxGuiDestroyWidget", Args(&[DType(Unknown)]), void),
    ("PdxGuiEditorMessageBox", Args(&[DType(Unknown)]), void),
    ("PdxGuiInterruptAllAnimations", Args(&[DType(Unknown)]), void),
    ("PdxGuiInterruptThenTriggerAllAnimations", Args(&[DType(Unknown), DType(Unknown)]), void),
    ("PdxGuiTriggerAllAnimations", Args(&[DType(Unknown)]), void),
    ("PdxProfilerFilterNext", Args(&[]), void),
    ("PdxProfilerFilterPrev", Args(&[]), void),
    ("PdxProfilerFilterTimers", Args(&[]), void),
    ("PdxProfilerGetCurrentFrame", Args(&[]), int32),
    ("PdxProfilerGetFrameTimeMs", Args(&[]), float),
    ("PdxProfilerGetNsPerTick", Args(&[]), float),
    ("PdxProfilerGuiGraphLinesEnabled", Args(&[]), bool),
    ("PdxProfilerGuiToggleGraphLines", Args(&[]), void),
    ("PdxProfilerGuiToggleStats", Args(&[]), void),
    ("PdxProfilerGuiTrackCurrentFrame", Args(&[]), void),
    ("PdxProfilerGuiWriteFrameCSV", Args(&[]), void),
    ("PdxProfilerIsRecording", Args(&[]), bool),
    ("PdxProfilerSelectThread", Args(&[]), void),
    ("PdxProfilerSetFrame", Args(&[]), void),
    ("PdxProfilerToggleRecording", Args(&[]), void),
    ("PlayerIsHost", Args(&[]), bool),
    ("ReleaseMode", Args(&[]), bool),
    ("ResumeAllConstruction", Args(&[]), Command),
    ("ResumeAllConstructionTooltip", Args(&[]), CString),
    ("ReturnToMenu", Args(&[]), void),
    ("ScaleToFitElementInside", Args(&[DType(Unknown), DType(Unknown)]), float),
    ("ScaleToFitElementOutside", Args(&[DType(Unknown), DType(Unknown)]), float),
    ("SelectEnumWithString", Args(&[DType(Unknown), DType(Unknown)]), void),
    ("SelectLocalization", Args(&[DType(Unknown), DType(Unknown), DType(Unknown)]), CString),
    ("SelectMapMode", Args(&[]), void),
    ("SelectMapModeByKey", Args(&[DType(Unknown)]), void),
    ("Select_CFixedPoint", Args(&[DType(bool), DType(CFixedPoint), DType(CFixedPoint)]), CFixedPoint),
    ("Select_CString", Args(&[DType(bool), DType(CString), DType(CString)]), CString),
    ("Select_CVector2f", Args(&[DType(bool), DType(CVector2f), DType(CVector2f)]), CVector2f),
    ("Select_CVector2i", Args(&[DType(bool), DType(CVector2i), DType(CVector2i)]), CVector2i),
    ("Select_CVector3f", Args(&[DType(bool), DType(CVector3f), DType(CVector3f)]), CVector3f),
    ("Select_CVector3i", Args(&[DType(bool), DType(CVector3i), DType(CVector3i)]), CVector3i),
    ("Select_CVector4f", Args(&[DType(bool), DType(CVector4f), DType(CVector4f)]), CVector4f),
    ("Select_CVector4i", Args(&[DType(bool), DType(CVector4i), DType(CVector4i)]), CVector4i),
    ("Select_float", Args(&[DType(bool), DType(float), DType(float)]), float),
    ("Select_int16", Args(&[DType(bool), DType(int16), DType(int16)]), int16),
    ("Select_int32", Args(&[DType(bool), DType(int32), DType(int32)]), int32),
    ("Select_int64", Args(&[DType(bool), DType(int64), DType(int64)]), int64),
    ("Select_int8", Args(&[DType(bool), DType(int8), DType(int8)]), int8),
    ("Select_uint16", Args(&[DType(bool), DType(uint16), DType(uint16)]), uint16),
    ("Select_uint32", Args(&[DType(bool), DType(uint32), DType(uint32)]), uint32),
    ("Select_uint64", Args(&[DType(bool), DType(uint64), DType(uint64)]), uint64),
    ("Select_uint8", Args(&[DType(bool), DType(uint8), DType(uint8)]), uint8),
    ("SetCameraRestrictionsEnabled", Args(&[DType(Unknown)]), void),
    ("SetCanOpenRightClickContextMenu", Args(&[DType(Unknown)]), void),
    ("SetDrawCityDebugGrid", Args(&[DType(Unknown)]), void),
    ("SetDrawCityDebugSpheres", Args(&[DType(Unknown)]), void),
    ("SetDynamicTerrainAutoUpdate", Args(&[DType(Unknown)]), void),
    ("SetDynamicsEnabled", Args(&[DType(Unknown)]), void),
    ("SetInteraction", Args(&[DType(Unknown)]), void),
    ("SetIronmanEnabledStatus", Args(&[DType(Unknown)]), void),
    ("SetRandomPlayableObserverCharacter", Args(&[]), void),
    ("SetTerrainManipulatorProvincesPreview", Args(&[DType(Unknown)]), void),
    ("ShouldAskConfirmation", Args(&[DType(Unknown)]), bool),
    ("ShouldShowAnimationInfo", Args(&[]), bool),
    ("ShouldShowSegmentedControlForSetting", Args(&[DType(Unknown)]), bool),
    ("StringIsEmpty", Args(&[DType(Unknown)]), bool),
    ("Subtract_CFixedPoint", Args(&[DType(CFixedPoint), DType(CFixedPoint)]), CFixedPoint),
    ("Subtract_CVector2f", Args(&[DType(CVector2f), DType(CVector2f)]), CVector2f),
    ("Subtract_float", Args(&[DType(float), DType(float)]), float),
    ("Subtract_int32", Args(&[DType(int32), DType(int32)]), int32),
    ("Subtract_int64", Args(&[DType(int64), DType(int64)]), int64),
    ("Subtract_uint32", Args(&[DType(uint32), DType(uint32)]), uint32),
    ("Subtract_uint64", Args(&[DType(uint64), DType(uint64)]), uint64),
    ("TextureListFormatSize", Args(&[DType(Unknown)]), CString),
    ("TextureListFormatkB", Args(&[DType(Unknown)]), CString),
    ("ToggleEncyclopedia", Args(&[]), void),
    ("ToggleInteraction", Args(&[DType(Unknown)]), void),
    ("ToggleMainMenu", Args(&[]), void),
    ("ToggleMessageSettings", Args(&[]), void),
    ("ToggleMessageSettingsForSingleItem", Args(&[DType(Unknown)]), void),
    ("ToggleMusicPlayer", Args(&[]), void),
    ("ToggleObserveCountry", Args(&[DType(Unknown)]), void),
    ("TransparentIfFalse", Args(&[DType(Unknown)]), float),
    ("TransparentIfTrue", Args(&[DType(Unknown)]), float),
    ("TransparentIfZero", Args(&[DType(Unknown)]), float),
    ("TransparentIfZero_int32", Args(&[DType(Unknown)]), float),
    ("TransparentIfZero_int64", Args(&[DType(Unknown)]), float),
    ("Unfocus", Args(&[DType(Unknown)]), void),
    ("UnforceMapMode", Args(&[]), void),
    ("UpdateDynamicTerrainInAllProvinces", Args(&[]), void),
    ("UsesTimerLocking", Args(&[]), bool),
]
