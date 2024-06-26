pub fn get_lua_source(path: &str) -> &'static str {
    match path {
        "HeadlessWrapper" => include_str!("../PathOfBuilding/src/HeadlessWrapper.lua"),
        "GameVersions" => include_str!("../PathOfBuilding/src/GameVersions.lua"),
        "UpdateCheck" => include_str!("../PathOfBuilding/src/UpdateCheck.lua"),
        "Launch" => include_str!("../PathOfBuilding/src/Launch.lua"),
        "LaunchInstall" => include_str!("../PathOfBuilding/src/LaunchInstall.lua"),
        "UpdateApply" => include_str!("../PathOfBuilding/src/UpdateApply.lua"),
        "Export/statdesc" => include_str!("../PathOfBuilding/src/Export/statdesc.lua"),
        "Export/Main" => include_str!("../PathOfBuilding/src/Export/Main.lua"),
        "Export/browse" => include_str!("../PathOfBuilding/src/Export/browse.lua"),
        "Export/Launch" => include_str!("../PathOfBuilding/src/Export/Launch.lua"),
        "Export/essenceMatch" => include_str!("../PathOfBuilding/src/Export/essenceMatch.lua"),
        "Export/passives" => include_str!("../PathOfBuilding/src/Export/passives.lua"),
        "Export/psg" => include_str!("../PathOfBuilding/src/Export/psg.lua"),
        "Export/spec" => include_str!("../PathOfBuilding/src/Export/spec.lua"),
        "Export/Classes/SpecColListControl" => include_str!("../PathOfBuilding/src/Export/Classes/SpecColListControl.lua"),
        "Export/Classes/Dat64File" => include_str!("../PathOfBuilding/src/Export/Classes/Dat64File.lua"),
        "Export/Classes/ScriptListControl" => include_str!("../PathOfBuilding/src/Export/Classes/ScriptListControl.lua"),
        "Export/Classes/RowListControl" => include_str!("../PathOfBuilding/src/Export/Classes/RowListControl.lua"),
        "Export/Classes/DatFile" => include_str!("../PathOfBuilding/src/Export/Classes/DatFile.lua"),
        "Export/Classes/GGPKData" => include_str!("../PathOfBuilding/src/Export/Classes/GGPKData.lua"),
        "Export/Classes/DatListControl" => include_str!("../PathOfBuilding/src/Export/Classes/DatListControl.lua"),
        "Export/Classes/GGPKSourceListControl" => include_str!("../PathOfBuilding/src/Export/Classes/GGPKSourceListControl.lua"),
        "Export/Tree/tree_in" => include_str!("../PathOfBuilding/src/Export/Tree/tree_in.lua"),
        "Export/Tree/tree" => include_str!("../PathOfBuilding/src/Export/Tree/tree.lua"),
        "Export/Scripts/masters" => include_str!("../PathOfBuilding/src/Export/Scripts/masters.lua"),
        "Export/Scripts/statdesc" => include_str!("../PathOfBuilding/src/Export/Scripts/statdesc.lua"),
        "Export/Scripts/minions" => include_str!("../PathOfBuilding/src/Export/Scripts/minions.lua"),
        "Export/Scripts/enchant" => include_str!("../PathOfBuilding/src/Export/Scripts/enchant.lua"),
        "Export/Scripts/costs" => include_str!("../PathOfBuilding/src/Export/Scripts/costs.lua"),
        "Export/Scripts/essence" => include_str!("../PathOfBuilding/src/Export/Scripts/essence.lua"),
        "Export/Scripts/cluster" => include_str!("../PathOfBuilding/src/Export/Scripts/cluster.lua"),
        "Export/Scripts/pantheons" => include_str!("../PathOfBuilding/src/Export/Scripts/pantheons.lua"),
        "Export/Scripts/miscdata" => include_str!("../PathOfBuilding/src/Export/Scripts/miscdata.lua"),
        "Export/Scripts/mods" => include_str!("../PathOfBuilding/src/Export/Scripts/mods.lua"),
        "Export/Scripts/skills" => include_str!("../PathOfBuilding/src/Export/Scripts/skills.lua"),
        "Export/Scripts/legionPassives" => include_str!("../PathOfBuilding/src/Export/Scripts/legionPassives.lua"),
        "Export/Scripts/bossData" => include_str!("../PathOfBuilding/src/Export/Scripts/bossData.lua"),
        "Export/Scripts/crucible" => include_str!("../PathOfBuilding/src/Export/Scripts/crucible.lua"),
        "Export/Scripts/bases" => include_str!("../PathOfBuilding/src/Export/Scripts/bases.lua"),
        "Export/Scripts/tattooPassives" => include_str!("../PathOfBuilding/src/Export/Scripts/tattooPassives.lua"),
        "Classes/ModDB" => include_str!("../PathOfBuilding/src/Classes/ModDB.lua"),
        "Classes/BuildListControl" => include_str!("../PathOfBuilding/src/Classes/BuildListControl.lua"),
        "Classes/TradeStatWeightMultiplierListControl" => include_str!("../PathOfBuilding/src/Classes/TradeStatWeightMultiplierListControl.lua"),
        "Classes/ModStore" => include_str!("../PathOfBuilding/src/Classes/ModStore.lua"),
        "Classes/TooltipHost" => include_str!("../PathOfBuilding/src/Classes/TooltipHost.lua"),
        "Classes/SectionControl" => include_str!("../PathOfBuilding/src/Classes/SectionControl.lua"),
        "Classes/CheckBoxControl" => include_str!("../PathOfBuilding/src/Classes/CheckBoxControl.lua"),
        "Classes/PassiveSpecListControl" => include_str!("../PathOfBuilding/src/Classes/PassiveSpecListControl.lua"),
        "Classes/PathControl" => include_str!("../PathOfBuilding/src/Classes/PathControl.lua"),
        "Classes/TreeTab" => include_str!("../PathOfBuilding/src/Classes/TreeTab.lua"),
        "Classes/FolderListControl" => include_str!("../PathOfBuilding/src/Classes/FolderListControl.lua"),
        "Classes/ModList" => include_str!("../PathOfBuilding/src/Classes/ModList.lua"),
        "Classes/EditControl" => include_str!("../PathOfBuilding/src/Classes/EditControl.lua"),
        "Classes/DropDownControl" => include_str!("../PathOfBuilding/src/Classes/DropDownControl.lua"),
        "Classes/TradeQuery" => include_str!("../PathOfBuilding/src/Classes/TradeQuery.lua"),
        "Classes/UndoHandler" => include_str!("../PathOfBuilding/src/Classes/UndoHandler.lua"),
        "Classes/CalcBreakdownControl" => include_str!("../PathOfBuilding/src/Classes/CalcBreakdownControl.lua"),
        "Classes/Item" => include_str!("../PathOfBuilding/src/Classes/Item.lua"),
        "Classes/ImportTab" => include_str!("../PathOfBuilding/src/Classes/ImportTab.lua"),
        "Classes/TradeQueryRateLimiter" => include_str!("../PathOfBuilding/src/Classes/TradeQueryRateLimiter.lua"),
        "Classes/PassiveTree" => include_str!("../PathOfBuilding/src/Classes/PassiveTree.lua"),
        "Classes/MinionListControl" => include_str!("../PathOfBuilding/src/Classes/MinionListControl.lua"),
        "Classes/ItemSlotControl" => include_str!("../PathOfBuilding/src/Classes/ItemSlotControl.lua"),
        "Classes/SkillsTab" => include_str!("../PathOfBuilding/src/Classes/SkillsTab.lua"),
        "Classes/TradeQueryGenerator" => include_str!("../PathOfBuilding/src/Classes/TradeQueryGenerator.lua"),
        "Classes/ScrollBarControl" => include_str!("../PathOfBuilding/src/Classes/ScrollBarControl.lua"),
        "Classes/ButtonControl" => include_str!("../PathOfBuilding/src/Classes/ButtonControl.lua"),
        "Classes/CalcsTab" => include_str!("../PathOfBuilding/src/Classes/CalcsTab.lua"),
        "Classes/Control" => include_str!("../PathOfBuilding/src/Classes/Control.lua"),
        "Classes/NotesTab" => include_str!("../PathOfBuilding/src/Classes/NotesTab.lua"),
        "Classes/PassiveMasteryControl" => include_str!("../PathOfBuilding/src/Classes/PassiveMasteryControl.lua"),
        "Classes/TradeQueryRequests" => include_str!("../PathOfBuilding/src/Classes/TradeQueryRequests.lua"),
        "Classes/SkillSetListControl" => include_str!("../PathOfBuilding/src/Classes/SkillSetListControl.lua"),
        "Classes/CalcSectionControl" => include_str!("../PathOfBuilding/src/Classes/CalcSectionControl.lua"),
        "Classes/TimelessJewelSocketControl" => include_str!("../PathOfBuilding/src/Classes/TimelessJewelSocketControl.lua"),
        "Classes/PassiveTreeView" => include_str!("../PathOfBuilding/src/Classes/PassiveTreeView.lua"),
        "Classes/PopupDialog" => include_str!("../PathOfBuilding/src/Classes/PopupDialog.lua"),
        "Classes/TimelessJewelListControl" => include_str!("../PathOfBuilding/src/Classes/TimelessJewelListControl.lua"),
        "Classes/ItemDBControl" => include_str!("../PathOfBuilding/src/Classes/ItemDBControl.lua"),
        "Classes/LabelControl" => include_str!("../PathOfBuilding/src/Classes/LabelControl.lua"),
        "Classes/RectangleOutlineControl" => include_str!("../PathOfBuilding/src/Classes/RectangleOutlineControl.lua"),
        "Classes/Tooltip" => include_str!("../PathOfBuilding/src/Classes/Tooltip.lua"),
        "Classes/ListControl" => include_str!("../PathOfBuilding/src/Classes/ListControl.lua"),
        "Classes/SearchHost" => include_str!("../PathOfBuilding/src/Classes/SearchHost.lua"),
        "Classes/SliderControl" => include_str!("../PathOfBuilding/src/Classes/SliderControl.lua"),
        "Classes/PowerReportListControl" => include_str!("../PathOfBuilding/src/Classes/PowerReportListControl.lua"),
        "Classes/PartyTab" => include_str!("../PathOfBuilding/src/Classes/PartyTab.lua"),
        "Classes/SharedItemListControl" => include_str!("../PathOfBuilding/src/Classes/SharedItemListControl.lua"),
        "Classes/ControlHost" => include_str!("../PathOfBuilding/src/Classes/ControlHost.lua"),
        "Classes/SharedItemSetListControl" => include_str!("../PathOfBuilding/src/Classes/SharedItemSetListControl.lua"),
        "Classes/SkillListControl" => include_str!("../PathOfBuilding/src/Classes/SkillListControl.lua"),
        "Classes/ItemsTab" => include_str!("../PathOfBuilding/src/Classes/ItemsTab.lua"),
        "Classes/NotableDBControl" => include_str!("../PathOfBuilding/src/Classes/NotableDBControl.lua"),
        "Classes/ItemSetListControl" => include_str!("../PathOfBuilding/src/Classes/ItemSetListControl.lua"),
        "Classes/TextListControl" => include_str!("../PathOfBuilding/src/Classes/TextListControl.lua"),
        "Classes/ItemListControl" => include_str!("../PathOfBuilding/src/Classes/ItemListControl.lua"),
        "Classes/PassiveSpec" => include_str!("../PathOfBuilding/src/Classes/PassiveSpec.lua"),
        "Classes/ConfigTab" => include_str!("../PathOfBuilding/src/Classes/ConfigTab.lua"),
        "Classes/GemSelectControl" => include_str!("../PathOfBuilding/src/Classes/GemSelectControl.lua"),
        "TreeData/3_11/tree" => include_str!("../PathOfBuilding/src/TreeData/3_11/tree.lua"),
        "TreeData/3_22/tree" => include_str!("../PathOfBuilding/src/TreeData/3_22/tree.lua"),
        "TreeData/3_10/tree" => include_str!("../PathOfBuilding/src/TreeData/3_10/tree.lua"),
        "TreeData/3_15/tree" => include_str!("../PathOfBuilding/src/TreeData/3_15/tree.lua"),
        "TreeData/3_24_ruthless/tree" => include_str!("../PathOfBuilding/src/TreeData/3_24_ruthless/tree.lua"),
        "TreeData/3_17/tree" => include_str!("../PathOfBuilding/src/TreeData/3_17/tree.lua"),
        "TreeData/3_13/tree" => include_str!("../PathOfBuilding/src/TreeData/3_13/tree.lua"),
        "TreeData/3_16/tree" => include_str!("../PathOfBuilding/src/TreeData/3_16/tree.lua"),
        "TreeData/2_6/tree" => include_str!("../PathOfBuilding/src/TreeData/2_6/tree.lua"),
        "TreeData/3_22_ruthless/tree" => include_str!("../PathOfBuilding/src/TreeData/3_22_ruthless/tree.lua"),
        "TreeData/3_8/tree" => include_str!("../PathOfBuilding/src/TreeData/3_8/tree.lua"),
        "TreeData/3_12/tree" => include_str!("../PathOfBuilding/src/TreeData/3_12/tree.lua"),
        "TreeData/3_20/tree" => include_str!("../PathOfBuilding/src/TreeData/3_20/tree.lua"),
        "TreeData/3_7/tree" => include_str!("../PathOfBuilding/src/TreeData/3_7/tree.lua"),
        "TreeData/3_19/tree" => include_str!("../PathOfBuilding/src/TreeData/3_19/tree.lua"),
        "TreeData/3_23_ruthless/tree" => include_str!("../PathOfBuilding/src/TreeData/3_23_ruthless/tree.lua"),
        "TreeData/3_24/tree" => include_str!("../PathOfBuilding/src/TreeData/3_24/tree.lua"),
        "TreeData/legion/tree-legion" => include_str!("../PathOfBuilding/src/TreeData/legion/tree-legion.lua"),
        "TreeData/3_6/tree" => include_str!("../PathOfBuilding/src/TreeData/3_6/tree.lua"),
        "TreeData/3_14/tree" => include_str!("../PathOfBuilding/src/TreeData/3_14/tree.lua"),
        "TreeData/3_23/tree" => include_str!("../PathOfBuilding/src/TreeData/3_23/tree.lua"),
        "TreeData/3_21/tree" => include_str!("../PathOfBuilding/src/TreeData/3_21/tree.lua"),
        "TreeData/3_18/tree" => include_str!("../PathOfBuilding/src/TreeData/3_18/tree.lua"),
        "TreeData/3_9/tree" => include_str!("../PathOfBuilding/src/TreeData/3_9/tree.lua"),
        "Modules/Common" => include_str!("../PathOfBuilding/src/Modules/Common.lua"),
        "Modules/Data" => include_str!("../PathOfBuilding/src/Modules/Data.lua"),
        "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
        "Modules/CalcMirages" => include_str!("../PathOfBuilding/src/Modules/CalcMirages.lua"),
        "Modules/CalcTools" => include_str!("../PathOfBuilding/src/Modules/CalcTools.lua"),
        "Modules/CalcOffence" => include_str!("../PathOfBuilding/src/Modules/CalcOffence.lua"),
        "Modules/PantheonTools" => include_str!("../PathOfBuilding/src/Modules/PantheonTools.lua"),
        "Modules/ItemTools" => include_str!("../PathOfBuilding/src/Modules/ItemTools.lua"),
        "Modules/ModParser" => include_str!("../PathOfBuilding/src/Modules/ModParser.lua"),
        "Modules/Build" => include_str!("../PathOfBuilding/src/Modules/Build.lua"),
        "Modules/ModTools" => include_str!("../PathOfBuilding/src/Modules/ModTools.lua"),
        "Modules/CalcBreakdown" => include_str!("../PathOfBuilding/src/Modules/CalcBreakdown.lua"),
        "Modules/BuildSiteTools" => include_str!("../PathOfBuilding/src/Modules/BuildSiteTools.lua"),
        "Modules/Calcs" => include_str!("../PathOfBuilding/src/Modules/Calcs.lua"),
        "Modules/ConfigOptions" => include_str!("../PathOfBuilding/src/Modules/ConfigOptions.lua"),
        "Modules/CalcSections" => include_str!("../PathOfBuilding/src/Modules/CalcSections.lua"),
        "Modules/CalcTriggers" => include_str!("../PathOfBuilding/src/Modules/CalcTriggers.lua"),
        "Modules/CalcActiveSkill" => include_str!("../PathOfBuilding/src/Modules/CalcActiveSkill.lua"),
        "Modules/DataLegionLookUpTableHelper" => include_str!("../PathOfBuilding/src/Modules/DataLegionLookUpTableHelper.lua"),
        "Modules/CalcSetup" => include_str!("../PathOfBuilding/src/Modules/CalcSetup.lua"),
        "Modules/CalcPerform" => include_str!("../PathOfBuilding/src/Modules/CalcPerform.lua"),
        "Modules/StatDescriber" => include_str!("../PathOfBuilding/src/Modules/StatDescriber.lua"),
        "Modules/CalcDefence" => include_str!("../PathOfBuilding/src/Modules/CalcDefence.lua"),
        "Modules/BuildList" => include_str!("../PathOfBuilding/src/Modules/BuildList.lua"),
        "Data/SkillStatMap" => include_str!("../PathOfBuilding/src/Data/SkillStatMap.lua"),
        "Data/Rares" => include_str!("../PathOfBuilding/src/Data/Rares.lua"),
        "Data/ModFlask" => include_str!("../PathOfBuilding/src/Data/ModFlask.lua"),
        "Data/Pantheons" => include_str!("../PathOfBuilding/src/Data/Pantheons.lua"),
        "Data/Global" => include_str!("../PathOfBuilding/src/Data/Global.lua"),
        "Data/TattooPassives" => include_str!("../PathOfBuilding/src/Data/TattooPassives.lua"),
        "Data/ModMaster" => include_str!("../PathOfBuilding/src/Data/ModMaster.lua"),
        "Data/EnchantmentWeapon" => include_str!("../PathOfBuilding/src/Data/EnchantmentWeapon.lua"),
        "Data/ModJewelCluster" => include_str!("../PathOfBuilding/src/Data/ModJewelCluster.lua"),
        "Data/ModMap" => include_str!("../PathOfBuilding/src/Data/ModMap.lua"),
        "Data/ModTincture" => include_str!("../PathOfBuilding/src/Data/ModTincture.lua"),
        "Data/EnchantmentHelmet" => include_str!("../PathOfBuilding/src/Data/EnchantmentHelmet.lua"),
        "Data/Gems" => include_str!("../PathOfBuilding/src/Data/Gems.lua"),
        "Data/ModJewel" => include_str!("../PathOfBuilding/src/Data/ModJewel.lua"),
        "Data/Crucible" => include_str!("../PathOfBuilding/src/Data/Crucible.lua"),
        "Data/ModCache" => include_str!("../PathOfBuilding/src/Data/ModCache.lua"),
        "Data/Spectres" => include_str!("../PathOfBuilding/src/Data/Spectres.lua"),
        "Data/Minions" => include_str!("../PathOfBuilding/src/Data/Minions.lua"),
        "Data/Essence" => include_str!("../PathOfBuilding/src/Data/Essence.lua"),
        "Data/Bosses" => include_str!("../PathOfBuilding/src/Data/Bosses.lua"),
        "Data/EnchantmentGloves" => include_str!("../PathOfBuilding/src/Data/EnchantmentGloves.lua"),
        "Data/BossSkills" => include_str!("../PathOfBuilding/src/Data/BossSkills.lua"),
        "Data/ModItem" => include_str!("../PathOfBuilding/src/Data/ModItem.lua"),
        "Data/EnchantmentBoots" => include_str!("../PathOfBuilding/src/Data/EnchantmentBoots.lua"),
        "Data/Misc" => include_str!("../PathOfBuilding/src/Data/Misc.lua"),
        "Data/ClusterJewels" => include_str!("../PathOfBuilding/src/Data/ClusterJewels.lua"),
        "Data/Costs" => include_str!("../PathOfBuilding/src/Data/Costs.lua"),
        "Data/ModVeiled" => include_str!("../PathOfBuilding/src/Data/ModVeiled.lua"),
        "Data/EnchantmentBelt" => include_str!("../PathOfBuilding/src/Data/EnchantmentBelt.lua"),
        "Data/EnchantmentFlask" => include_str!("../PathOfBuilding/src/Data/EnchantmentFlask.lua"),
        "Data/ModNecropolis" => include_str!("../PathOfBuilding/src/Data/ModNecropolis.lua"),
        "Data/EnchantmentBody" => include_str!("../PathOfBuilding/src/Data/EnchantmentBody.lua"),
        "Data/QueryMods" => include_str!("../PathOfBuilding/src/Data/QueryMods.lua"),
        "Data/ModJewelCharm" => include_str!("../PathOfBuilding/src/Data/ModJewelCharm.lua"),
        "Data/ModJewelAbyss" => include_str!("../PathOfBuilding/src/Data/ModJewelAbyss.lua"),
        "Data/Skills/sup_str" => include_str!("../PathOfBuilding/src/Data/Skills/sup_str.lua"),
        "Data/Skills/spectre" => include_str!("../PathOfBuilding/src/Data/Skills/spectre.lua"),
        "Data/Skills/act_str" => include_str!("../PathOfBuilding/src/Data/Skills/act_str.lua"),
        "Data/Skills/minion" => include_str!("../PathOfBuilding/src/Data/Skills/minion.lua"),
        "Data/Skills/other" => include_str!("../PathOfBuilding/src/Data/Skills/other.lua"),
        "Data/Skills/sup_dex" => include_str!("../PathOfBuilding/src/Data/Skills/sup_dex.lua"),
        "Data/Skills/glove" => include_str!("../PathOfBuilding/src/Data/Skills/glove.lua"),
        "Data/Skills/sup_int" => include_str!("../PathOfBuilding/src/Data/Skills/sup_int.lua"),
        "Data/Skills/act_dex" => include_str!("../PathOfBuilding/src/Data/Skills/act_dex.lua"),
        "Data/Skills/act_int" => include_str!("../PathOfBuilding/src/Data/Skills/act_int.lua"),
        "Data/StatDescriptions/buff_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/buff_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/minion_spell_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_spell_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/brand_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/brand_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/tincture_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/tincture_stat_descriptions.lua"),
        "Data/StatDescriptions/monster_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/monster_stat_descriptions.lua"),
        "Data/StatDescriptions/debuff_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/debuff_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/skill_stat_descriptions.lua"),
        "Data/StatDescriptions/variable_duration_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/variable_duration_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/single_minion_spell_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/single_minion_spell_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/gem_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/gem_stat_descriptions.lua"),
        "Data/StatDescriptions/offering_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/offering_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/curse_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/curse_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/active_skill_gem_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/active_skill_gem_stat_descriptions.lua"),
        "Data/StatDescriptions/banner_aura_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/banner_aura_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/secondary_debuff_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/secondary_debuff_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/beam_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/beam_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/stat_descriptions.lua"),
        "Data/StatDescriptions/minion_attack_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_attack_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/aura_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/aura_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/minion_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_skill_stat_descriptions.lua"),
        "Data/StatDescriptions/minion_spell_damage_skill_stat_descriptions" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_spell_damage_skill_stat_descriptions.lua"),
        "Data/TimelessJewelData/LegionPassives" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/LegionPassives.lua"),
        "Data/TimelessJewelData/LegionTradeIds" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/LegionTradeIds.lua"),
        "Data/TimelessJewelData/NodeIndexMapping" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/NodeIndexMapping.lua"),
        "Data/Uniques/flask" => include_str!("../PathOfBuilding/src/Data/Uniques/flask.lua"),
        "Data/Uniques/body" => include_str!("../PathOfBuilding/src/Data/Uniques/body.lua"),
        "Data/Uniques/boots" => include_str!("../PathOfBuilding/src/Data/Uniques/boots.lua"),
        "Data/Uniques/fishing" => include_str!("../PathOfBuilding/src/Data/Uniques/fishing.lua"),
        "Data/Uniques/axe" => include_str!("../PathOfBuilding/src/Data/Uniques/axe.lua"),
        "Data/Uniques/gloves" => include_str!("../PathOfBuilding/src/Data/Uniques/gloves.lua"),
        "Data/Uniques/dagger" => include_str!("../PathOfBuilding/src/Data/Uniques/dagger.lua"),
        "Data/Uniques/quiver" => include_str!("../PathOfBuilding/src/Data/Uniques/quiver.lua"),
        "Data/Uniques/bow" => include_str!("../PathOfBuilding/src/Data/Uniques/bow.lua"),
        "Data/Uniques/wand" => include_str!("../PathOfBuilding/src/Data/Uniques/wand.lua"),
        "Data/Uniques/staff" => include_str!("../PathOfBuilding/src/Data/Uniques/staff.lua"),
        "Data/Uniques/tincture" => include_str!("../PathOfBuilding/src/Data/Uniques/tincture.lua"),
        "Data/Uniques/mace" => include_str!("../PathOfBuilding/src/Data/Uniques/mace.lua"),
        "Data/Uniques/claw" => include_str!("../PathOfBuilding/src/Data/Uniques/claw.lua"),
        "Data/Uniques/helmet" => include_str!("../PathOfBuilding/src/Data/Uniques/helmet.lua"),
        "Data/Uniques/jewel" => include_str!("../PathOfBuilding/src/Data/Uniques/jewel.lua"),
        "Data/Uniques/sword" => include_str!("../PathOfBuilding/src/Data/Uniques/sword.lua"),
        "Data/Uniques/belt" => include_str!("../PathOfBuilding/src/Data/Uniques/belt.lua"),
        "Data/Uniques/shield" => include_str!("../PathOfBuilding/src/Data/Uniques/shield.lua"),
        "Data/Uniques/amulet" => include_str!("../PathOfBuilding/src/Data/Uniques/amulet.lua"),
        "Data/Uniques/ring" => include_str!("../PathOfBuilding/src/Data/Uniques/ring.lua"),
        "Data/Uniques/Special/Generated" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/Generated.lua"),
        "Data/Uniques/Special/race" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/race.lua"),
        "Data/Uniques/Special/New" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/New.lua"),
        "Data/Uniques/Special/WatchersEye" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/WatchersEye.lua"),
        "Data/Bases/flask" => include_str!("../PathOfBuilding/src/Data/Bases/flask.lua"),
        "Data/Bases/body" => include_str!("../PathOfBuilding/src/Data/Bases/body.lua"),
        "Data/Bases/boots" => include_str!("../PathOfBuilding/src/Data/Bases/boots.lua"),
        "Data/Bases/fishing" => include_str!("../PathOfBuilding/src/Data/Bases/fishing.lua"),
        "Data/Bases/axe" => include_str!("../PathOfBuilding/src/Data/Bases/axe.lua"),
        "Data/Bases/gloves" => include_str!("../PathOfBuilding/src/Data/Bases/gloves.lua"),
        "Data/Bases/dagger" => include_str!("../PathOfBuilding/src/Data/Bases/dagger.lua"),
        "Data/Bases/quiver" => include_str!("../PathOfBuilding/src/Data/Bases/quiver.lua"),
        "Data/Bases/bow" => include_str!("../PathOfBuilding/src/Data/Bases/bow.lua"),
        "Data/Bases/wand" => include_str!("../PathOfBuilding/src/Data/Bases/wand.lua"),
        "Data/Bases/staff" => include_str!("../PathOfBuilding/src/Data/Bases/staff.lua"),
        "Data/Bases/tincture" => include_str!("../PathOfBuilding/src/Data/Bases/tincture.lua"),
        "Data/Bases/mace" => include_str!("../PathOfBuilding/src/Data/Bases/mace.lua"),
        "Data/Bases/claw" => include_str!("../PathOfBuilding/src/Data/Bases/claw.lua"),
        "Data/Bases/helmet" => include_str!("../PathOfBuilding/src/Data/Bases/helmet.lua"),
        "Data/Bases/jewel" => include_str!("../PathOfBuilding/src/Data/Bases/jewel.lua"),
        "Data/Bases/sword" => include_str!("../PathOfBuilding/src/Data/Bases/sword.lua"),
        "Data/Bases/belt" => include_str!("../PathOfBuilding/src/Data/Bases/belt.lua"),
        "Data/Bases/shield" => include_str!("../PathOfBuilding/src/Data/Bases/shield.lua"),
        "Data/Bases/amulet" => include_str!("../PathOfBuilding/src/Data/Bases/amulet.lua"),
        "Data/Bases/ring" => include_str!("../PathOfBuilding/src/Data/Bases/ring.lua"),
        _ => unreachable!(),
    }
}
