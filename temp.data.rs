use std::include_str;

pub const DATA: &[(&str, &'static str)] = &[
    "HeadlessWrapper.lua" => include_str!("../PathOfBuilding/src/HeadlessWrapper.lua"),
    "GameVersions.lua" => include_str!("../PathOfBuilding/src/GameVersions.lua"),
    "UpdateCheck.lua" => include_str!("../PathOfBuilding/src/UpdateCheck.lua"),
    "Launch.lua" => include_str!("../PathOfBuilding/src/Launch.lua"),
    "LaunchInstall.lua" => include_str!("../PathOfBuilding/src/LaunchInstall.lua"),
    "UpdateApply.lua" => include_str!("../PathOfBuilding/src/UpdateApply.lua"),
    "Export/statdesc.lua" => include_str!("../PathOfBuilding/src/Export/statdesc.lua"),
    "Export/Main.lua" => include_str!("../PathOfBuilding/src/Export/Main.lua"),
    "Export/browse.lua" => include_str!("../PathOfBuilding/src/Export/browse.lua"),
    "Export/Launch.lua" => include_str!("../PathOfBuilding/src/Export/Launch.lua"),
    "Export/essenceMatch.lua" => include_str!("../PathOfBuilding/src/Export/essenceMatch.lua"),
    "Export/passives.lua" => include_str!("../PathOfBuilding/src/Export/passives.lua"),
    "Export/psg.lua" => include_str!("../PathOfBuilding/src/Export/psg.lua"),
    "Export/spec.lua" => include_str!("../PathOfBuilding/src/Export/spec.lua"),
    "Export/Classes/SpecColListControl.lua" => include_str!("../PathOfBuilding/src/Export/Classes/SpecColListControl.lua"),
    "Export/Classes/Dat64File.lua" => include_str!("../PathOfBuilding/src/Export/Classes/Dat64File.lua"),
    "Export/Classes/ScriptListControl.lua" => include_str!("../PathOfBuilding/src/Export/Classes/ScriptListControl.lua"),
    "Export/Classes/RowListControl.lua" => include_str!("../PathOfBuilding/src/Export/Classes/RowListControl.lua"),
    "Export/Classes/DatFile.lua" => include_str!("../PathOfBuilding/src/Export/Classes/DatFile.lua"),
    "Export/Classes/GGPKData.lua" => include_str!("../PathOfBuilding/src/Export/Classes/GGPKData.lua"),
    "Export/Classes/DatListControl.lua" => include_str!("../PathOfBuilding/src/Export/Classes/DatListControl.lua"),
    "Export/Classes/GGPKSourceListControl.lua" => include_str!("../PathOfBuilding/src/Export/Classes/GGPKSourceListControl.lua"),
    "Export/Tree/tree_in.lua" => include_str!("../PathOfBuilding/src/Export/Tree/tree_in.lua"),
    "Export/Tree/tree.lua" => include_str!("../PathOfBuilding/src/Export/Tree/tree.lua"),
    "Export/Scripts/masters.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/masters.lua"),
    "Export/Scripts/statdesc.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/statdesc.lua"),
    "Export/Scripts/minions.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/minions.lua"),
    "Export/Scripts/enchant.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/enchant.lua"),
    "Export/Scripts/costs.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/costs.lua"),
    "Export/Scripts/essence.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/essence.lua"),
    "Export/Scripts/cluster.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/cluster.lua"),
    "Export/Scripts/pantheons.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/pantheons.lua"),
    "Export/Scripts/miscdata.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/miscdata.lua"),
    "Export/Scripts/mods.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/mods.lua"),
    "Export/Scripts/skills.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/skills.lua"),
    "Export/Scripts/legionPassives.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/legionPassives.lua"),
    "Export/Scripts/bossData.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/bossData.lua"),
    "Export/Scripts/crucible.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/crucible.lua"),
    "Export/Scripts/bases.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/bases.lua"),
    "Export/Scripts/tattooPassives.lua" => include_str!("../PathOfBuilding/src/Export/Scripts/tattooPassives.lua"),
    "Classes/ModDB.lua" => include_str!("../PathOfBuilding/src/Classes/ModDB.lua"),
    "Classes/BuildListControl.lua" => include_str!("../PathOfBuilding/src/Classes/BuildListControl.lua"),
    "Classes/TradeStatWeightMultiplierListControl.lua" => include_str!("../PathOfBuilding/src/Classes/TradeStatWeightMultiplierListControl.lua"),
    "Classes/ModStore.lua" => include_str!("../PathOfBuilding/src/Classes/ModStore.lua"),
    "Classes/TooltipHost.lua" => include_str!("../PathOfBuilding/src/Classes/TooltipHost.lua"),
    "Classes/SectionControl.lua" => include_str!("../PathOfBuilding/src/Classes/SectionControl.lua"),
    "Classes/CheckBoxControl.lua" => include_str!("../PathOfBuilding/src/Classes/CheckBoxControl.lua"),
    "Classes/PassiveSpecListControl.lua" => include_str!("../PathOfBuilding/src/Classes/PassiveSpecListControl.lua"),
    "Classes/PathControl.lua" => include_str!("../PathOfBuilding/src/Classes/PathControl.lua"),
    "Classes/TreeTab.lua" => include_str!("../PathOfBuilding/src/Classes/TreeTab.lua"),
    "Classes/FolderListControl.lua" => include_str!("../PathOfBuilding/src/Classes/FolderListControl.lua"),
    "Classes/ModList.lua" => include_str!("../PathOfBuilding/src/Classes/ModList.lua"),
    "Classes/EditControl.lua" => include_str!("../PathOfBuilding/src/Classes/EditControl.lua"),
    "Classes/DropDownControl.lua" => include_str!("../PathOfBuilding/src/Classes/DropDownControl.lua"),
    "Classes/TradeQuery.lua" => include_str!("../PathOfBuilding/src/Classes/TradeQuery.lua"),
    "Classes/UndoHandler.lua" => include_str!("../PathOfBuilding/src/Classes/UndoHandler.lua"),
    "Classes/CalcBreakdownControl.lua" => include_str!("../PathOfBuilding/src/Classes/CalcBreakdownControl.lua"),
    "Classes/Item.lua" => include_str!("../PathOfBuilding/src/Classes/Item.lua"),
    "Classes/ImportTab.lua" => include_str!("../PathOfBuilding/src/Classes/ImportTab.lua"),
    "Classes/TradeQueryRateLimiter.lua" => include_str!("../PathOfBuilding/src/Classes/TradeQueryRateLimiter.lua"),
    "Classes/PassiveTree.lua" => include_str!("../PathOfBuilding/src/Classes/PassiveTree.lua"),
    "Classes/MinionListControl.lua" => include_str!("../PathOfBuilding/src/Classes/MinionListControl.lua"),
    "Classes/ItemSlotControl.lua" => include_str!("../PathOfBuilding/src/Classes/ItemSlotControl.lua"),
    "Classes/SkillsTab.lua" => include_str!("../PathOfBuilding/src/Classes/SkillsTab.lua"),
    "Classes/TradeQueryGenerator.lua" => include_str!("../PathOfBuilding/src/Classes/TradeQueryGenerator.lua"),
    "Classes/ScrollBarControl.lua" => include_str!("../PathOfBuilding/src/Classes/ScrollBarControl.lua"),
    "Classes/ButtonControl.lua" => include_str!("../PathOfBuilding/src/Classes/ButtonControl.lua"),
    "Classes/CalcsTab.lua" => include_str!("../PathOfBuilding/src/Classes/CalcsTab.lua"),
    "Classes/Control.lua" => include_str!("../PathOfBuilding/src/Classes/Control.lua"),
    "Classes/NotesTab.lua" => include_str!("../PathOfBuilding/src/Classes/NotesTab.lua"),
    "Classes/PassiveMasteryControl.lua" => include_str!("../PathOfBuilding/src/Classes/PassiveMasteryControl.lua"),
    "Classes/TradeQueryRequests.lua" => include_str!("../PathOfBuilding/src/Classes/TradeQueryRequests.lua"),
    "Classes/SkillSetListControl.lua" => include_str!("../PathOfBuilding/src/Classes/SkillSetListControl.lua"),
    "Classes/CalcSectionControl.lua" => include_str!("../PathOfBuilding/src/Classes/CalcSectionControl.lua"),
    "Classes/TimelessJewelSocketControl.lua" => include_str!("../PathOfBuilding/src/Classes/TimelessJewelSocketControl.lua"),
    "Classes/PassiveTreeView.lua" => include_str!("../PathOfBuilding/src/Classes/PassiveTreeView.lua"),
    "Classes/PopupDialog.lua" => include_str!("../PathOfBuilding/src/Classes/PopupDialog.lua"),
    "Classes/TimelessJewelListControl.lua" => include_str!("../PathOfBuilding/src/Classes/TimelessJewelListControl.lua"),
    "Classes/ItemDBControl.lua" => include_str!("../PathOfBuilding/src/Classes/ItemDBControl.lua"),
    "Classes/LabelControl.lua" => include_str!("../PathOfBuilding/src/Classes/LabelControl.lua"),
    "Classes/RectangleOutlineControl.lua" => include_str!("../PathOfBuilding/src/Classes/RectangleOutlineControl.lua"),
    "Classes/Tooltip.lua" => include_str!("../PathOfBuilding/src/Classes/Tooltip.lua"),
    "Classes/ListControl.lua" => include_str!("../PathOfBuilding/src/Classes/ListControl.lua"),
    "Classes/SearchHost.lua" => include_str!("../PathOfBuilding/src/Classes/SearchHost.lua"),
    "Classes/SliderControl.lua" => include_str!("../PathOfBuilding/src/Classes/SliderControl.lua"),
    "Classes/PowerReportListControl.lua" => include_str!("../PathOfBuilding/src/Classes/PowerReportListControl.lua"),
    "Classes/PartyTab.lua" => include_str!("../PathOfBuilding/src/Classes/PartyTab.lua"),
    "Classes/SharedItemListControl.lua" => include_str!("../PathOfBuilding/src/Classes/SharedItemListControl.lua"),
    "Classes/ControlHost.lua" => include_str!("../PathOfBuilding/src/Classes/ControlHost.lua"),
    "Classes/SharedItemSetListControl.lua" => include_str!("../PathOfBuilding/src/Classes/SharedItemSetListControl.lua"),
    "Classes/SkillListControl.lua" => include_str!("../PathOfBuilding/src/Classes/SkillListControl.lua"),
    "Classes/ItemsTab.lua" => include_str!("../PathOfBuilding/src/Classes/ItemsTab.lua"),
    "Classes/NotableDBControl.lua" => include_str!("../PathOfBuilding/src/Classes/NotableDBControl.lua"),
    "Classes/ItemSetListControl.lua" => include_str!("../PathOfBuilding/src/Classes/ItemSetListControl.lua"),
    "Classes/TextListControl.lua" => include_str!("../PathOfBuilding/src/Classes/TextListControl.lua"),
    "Classes/ItemListControl.lua" => include_str!("../PathOfBuilding/src/Classes/ItemListControl.lua"),
    "Classes/PassiveSpec.lua" => include_str!("../PathOfBuilding/src/Classes/PassiveSpec.lua"),
    "Classes/ConfigTab.lua" => include_str!("../PathOfBuilding/src/Classes/ConfigTab.lua"),
    "Classes/GemSelectControl.lua" => include_str!("../PathOfBuilding/src/Classes/GemSelectControl.lua"),
    "TreeData/3_11/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_11/tree.lua"),
    "TreeData/3_22/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_22/tree.lua"),
    "TreeData/3_10/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_10/tree.lua"),
    "TreeData/3_15/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_15/tree.lua"),
    "TreeData/3_24_ruthless/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_24_ruthless/tree.lua"),
    "TreeData/3_17/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_17/tree.lua"),
    "TreeData/3_13/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_13/tree.lua"),
    "TreeData/3_16/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_16/tree.lua"),
    "TreeData/2_6/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/2_6/tree.lua"),
    "TreeData/3_22_ruthless/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_22_ruthless/tree.lua"),
    "TreeData/3_8/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_8/tree.lua"),
    "TreeData/3_12/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_12/tree.lua"),
    "TreeData/3_20/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_20/tree.lua"),
    "TreeData/3_7/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_7/tree.lua"),
    "TreeData/3_19/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_19/tree.lua"),
    "TreeData/3_23_ruthless/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_23_ruthless/tree.lua"),
    "TreeData/3_24/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_24/tree.lua"),
    "TreeData/legion/tree-legion.lua" => include_str!("../PathOfBuilding/src/TreeData/legion/tree-legion.lua"),
    "TreeData/3_6/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_6/tree.lua"),
    "TreeData/3_14/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_14/tree.lua"),
    "TreeData/3_23/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_23/tree.lua"),
    "TreeData/3_21/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_21/tree.lua"),
    "TreeData/3_18/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_18/tree.lua"),
    "TreeData/3_9/tree.lua" => include_str!("../PathOfBuilding/src/TreeData/3_9/tree.lua"),
    "Modules/Common.lua" => include_str!("../PathOfBuilding/src/Modules/Common.lua"),
    "Modules/Data.lua" => include_str!("../PathOfBuilding/src/Modules/Data.lua"),
    "Modules/Main.lua" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
    "Modules/CalcMirages.lua" => include_str!("../PathOfBuilding/src/Modules/CalcMirages.lua"),
    "Modules/CalcTools.lua" => include_str!("../PathOfBuilding/src/Modules/CalcTools.lua"),
    "Modules/CalcOffence.lua" => include_str!("../PathOfBuilding/src/Modules/CalcOffence.lua"),
    "Modules/PantheonTools.lua" => include_str!("../PathOfBuilding/src/Modules/PantheonTools.lua"),
    "Modules/ItemTools.lua" => include_str!("../PathOfBuilding/src/Modules/ItemTools.lua"),
    "Modules/ModParser.lua" => include_str!("../PathOfBuilding/src/Modules/ModParser.lua"),
    "Modules/Build.lua" => include_str!("../PathOfBuilding/src/Modules/Build.lua"),
    "Modules/ModTools.lua" => include_str!("../PathOfBuilding/src/Modules/ModTools.lua"),
    "Modules/CalcBreakdown.lua" => include_str!("../PathOfBuilding/src/Modules/CalcBreakdown.lua"),
    "Modules/BuildSiteTools.lua" => include_str!("../PathOfBuilding/src/Modules/BuildSiteTools.lua"),
    "Modules/Calcs.lua" => include_str!("../PathOfBuilding/src/Modules/Calcs.lua"),
    "Modules/ConfigOptions.lua" => include_str!("../PathOfBuilding/src/Modules/ConfigOptions.lua"),
    "Modules/CalcSections.lua" => include_str!("../PathOfBuilding/src/Modules/CalcSections.lua"),
    "Modules/CalcTriggers.lua" => include_str!("../PathOfBuilding/src/Modules/CalcTriggers.lua"),
    "Modules/CalcActiveSkill.lua" => include_str!("../PathOfBuilding/src/Modules/CalcActiveSkill.lua"),
    "Modules/DataLegionLookUpTableHelper.lua" => include_str!("../PathOfBuilding/src/Modules/DataLegionLookUpTableHelper.lua"),
    "Modules/CalcSetup.lua" => include_str!("../PathOfBuilding/src/Modules/CalcSetup.lua"),
    "Modules/CalcPerform.lua" => include_str!("../PathOfBuilding/src/Modules/CalcPerform.lua"),
    "Modules/StatDescriber.lua" => include_str!("../PathOfBuilding/src/Modules/StatDescriber.lua"),
    "Modules/CalcDefence.lua" => include_str!("../PathOfBuilding/src/Modules/CalcDefence.lua"),
    "Modules/BuildList.lua" => include_str!("../PathOfBuilding/src/Modules/BuildList.lua"),
    "Data/SkillStatMap.lua" => include_str!("../PathOfBuilding/src/Data/SkillStatMap.lua"),
    "Data/Rares.lua" => include_str!("../PathOfBuilding/src/Data/Rares.lua"),
    "Data/ModFlask.lua" => include_str!("../PathOfBuilding/src/Data/ModFlask.lua"),
    "Data/Pantheons.lua" => include_str!("../PathOfBuilding/src/Data/Pantheons.lua"),
    "Data/Global.lua" => include_str!("../PathOfBuilding/src/Data/Global.lua"),
    "Data/TattooPassives.lua" => include_str!("../PathOfBuilding/src/Data/TattooPassives.lua"),
    "Data/ModMaster.lua" => include_str!("../PathOfBuilding/src/Data/ModMaster.lua"),
    "Data/EnchantmentWeapon.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentWeapon.lua"),
    "Data/ModJewelCluster.lua" => include_str!("../PathOfBuilding/src/Data/ModJewelCluster.lua"),
    "Data/ModMap.lua" => include_str!("../PathOfBuilding/src/Data/ModMap.lua"),
    "Data/ModTincture.lua" => include_str!("../PathOfBuilding/src/Data/ModTincture.lua"),
    "Data/EnchantmentHelmet.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentHelmet.lua"),
    "Data/Gems.lua" => include_str!("../PathOfBuilding/src/Data/Gems.lua"),
    "Data/ModJewel.lua" => include_str!("../PathOfBuilding/src/Data/ModJewel.lua"),
    "Data/Crucible.lua" => include_str!("../PathOfBuilding/src/Data/Crucible.lua"),
    "Data/ModCache.lua" => include_str!("../PathOfBuilding/src/Data/ModCache.lua"),
    "Data/Spectres.lua" => include_str!("../PathOfBuilding/src/Data/Spectres.lua"),
    "Data/Minions.lua" => include_str!("../PathOfBuilding/src/Data/Minions.lua"),
    "Data/Essence.lua" => include_str!("../PathOfBuilding/src/Data/Essence.lua"),
    "Data/Bosses.lua" => include_str!("../PathOfBuilding/src/Data/Bosses.lua"),
    "Data/EnchantmentGloves.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentGloves.lua"),
    "Data/BossSkills.lua" => include_str!("../PathOfBuilding/src/Data/BossSkills.lua"),
    "Data/ModItem.lua" => include_str!("../PathOfBuilding/src/Data/ModItem.lua"),
    "Data/EnchantmentBoots.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentBoots.lua"),
    "Data/Misc.lua" => include_str!("../PathOfBuilding/src/Data/Misc.lua"),
    "Data/ClusterJewels.lua" => include_str!("../PathOfBuilding/src/Data/ClusterJewels.lua"),
    "Data/Costs.lua" => include_str!("../PathOfBuilding/src/Data/Costs.lua"),
    "Data/ModVeiled.lua" => include_str!("../PathOfBuilding/src/Data/ModVeiled.lua"),
    "Data/EnchantmentBelt.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentBelt.lua"),
    "Data/EnchantmentFlask.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentFlask.lua"),
    "Data/ModNecropolis.lua" => include_str!("../PathOfBuilding/src/Data/ModNecropolis.lua"),
    "Data/EnchantmentBody.lua" => include_str!("../PathOfBuilding/src/Data/EnchantmentBody.lua"),
    "Data/QueryMods.lua" => include_str!("../PathOfBuilding/src/Data/QueryMods.lua"),
    "Data/ModJewelCharm.lua" => include_str!("../PathOfBuilding/src/Data/ModJewelCharm.lua"),
    "Data/ModJewelAbyss.lua" => include_str!("../PathOfBuilding/src/Data/ModJewelAbyss.lua"),
    "Data/Skills/sup_str.lua" => include_str!("../PathOfBuilding/src/Data/Skills/sup_str.lua"),
    "Data/Skills/spectre.lua" => include_str!("../PathOfBuilding/src/Data/Skills/spectre.lua"),
    "Data/Skills/act_str.lua" => include_str!("../PathOfBuilding/src/Data/Skills/act_str.lua"),
    "Data/Skills/minion.lua" => include_str!("../PathOfBuilding/src/Data/Skills/minion.lua"),
    "Data/Skills/other.lua" => include_str!("../PathOfBuilding/src/Data/Skills/other.lua"),
    "Data/Skills/sup_dex.lua" => include_str!("../PathOfBuilding/src/Data/Skills/sup_dex.lua"),
    "Data/Skills/glove.lua" => include_str!("../PathOfBuilding/src/Data/Skills/glove.lua"),
    "Data/Skills/sup_int.lua" => include_str!("../PathOfBuilding/src/Data/Skills/sup_int.lua"),
    "Data/Skills/act_dex.lua" => include_str!("../PathOfBuilding/src/Data/Skills/act_dex.lua"),
    "Data/Skills/act_int.lua" => include_str!("../PathOfBuilding/src/Data/Skills/act_int.lua"),
    "Data/StatDescriptions/buff_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/buff_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/minion_spell_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_spell_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/brand_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/brand_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/tincture_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/tincture_stat_descriptions.lua"),
    "Data/StatDescriptions/monster_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/monster_stat_descriptions.lua"),
    "Data/StatDescriptions/debuff_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/debuff_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/skill_stat_descriptions.lua"),
    "Data/StatDescriptions/variable_duration_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/variable_duration_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/single_minion_spell_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/single_minion_spell_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/gem_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/gem_stat_descriptions.lua"),
    "Data/StatDescriptions/offering_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/offering_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/curse_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/curse_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/active_skill_gem_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/active_skill_gem_stat_descriptions.lua"),
    "Data/StatDescriptions/banner_aura_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/banner_aura_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/secondary_debuff_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/secondary_debuff_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/beam_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/beam_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/stat_descriptions.lua"),
    "Data/StatDescriptions/minion_attack_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_attack_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/aura_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/aura_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/minion_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_skill_stat_descriptions.lua"),
    "Data/StatDescriptions/minion_spell_damage_skill_stat_descriptions.lua" => include_str!("../PathOfBuilding/src/Data/StatDescriptions/minion_spell_damage_skill_stat_descriptions.lua"),
    "Data/TimelessJewelData/LegionPassives.lua" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/LegionPassives.lua"),
    "Data/TimelessJewelData/LegionTradeIds.lua" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/LegionTradeIds.lua"),
    "Data/TimelessJewelData/NodeIndexMapping.lua" => include_str!("../PathOfBuilding/src/Data/TimelessJewelData/NodeIndexMapping.lua"),
    "Data/Uniques/flask.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/flask.lua"),
    "Data/Uniques/body.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/body.lua"),
    "Data/Uniques/boots.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/boots.lua"),
    "Data/Uniques/fishing.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/fishing.lua"),
    "Data/Uniques/axe.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/axe.lua"),
    "Data/Uniques/gloves.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/gloves.lua"),
    "Data/Uniques/dagger.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/dagger.lua"),
    "Data/Uniques/quiver.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/quiver.lua"),
    "Data/Uniques/bow.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/bow.lua"),
    "Data/Uniques/wand.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/wand.lua"),
    "Data/Uniques/staff.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/staff.lua"),
    "Data/Uniques/tincture.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/tincture.lua"),
    "Data/Uniques/mace.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/mace.lua"),
    "Data/Uniques/claw.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/claw.lua"),
    "Data/Uniques/helmet.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/helmet.lua"),
    "Data/Uniques/jewel.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/jewel.lua"),
    "Data/Uniques/sword.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/sword.lua"),
    "Data/Uniques/belt.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/belt.lua"),
    "Data/Uniques/shield.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/shield.lua"),
    "Data/Uniques/amulet.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/amulet.lua"),
    "Data/Uniques/ring.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/ring.lua"),
    "Data/Uniques/Special/Generated.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/Generated.lua"),
    "Data/Uniques/Special/race.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/race.lua"),
    "Data/Uniques/Special/New.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/New.lua"),
    "Data/Uniques/Special/WatchersEye.lua" => include_str!("../PathOfBuilding/src/Data/Uniques/Special/WatchersEye.lua"),
    "Data/Bases/flask.lua" => include_str!("../PathOfBuilding/src/Data/Bases/flask.lua"),
    "Data/Bases/body.lua" => include_str!("../PathOfBuilding/src/Data/Bases/body.lua"),
    "Data/Bases/boots.lua" => include_str!("../PathOfBuilding/src/Data/Bases/boots.lua"),
    "Data/Bases/fishing.lua" => include_str!("../PathOfBuilding/src/Data/Bases/fishing.lua"),
    "Data/Bases/axe.lua" => include_str!("../PathOfBuilding/src/Data/Bases/axe.lua"),
    "Data/Bases/gloves.lua" => include_str!("../PathOfBuilding/src/Data/Bases/gloves.lua"),
    "Data/Bases/dagger.lua" => include_str!("../PathOfBuilding/src/Data/Bases/dagger.lua"),
    "Data/Bases/quiver.lua" => include_str!("../PathOfBuilding/src/Data/Bases/quiver.lua"),
    "Data/Bases/bow.lua" => include_str!("../PathOfBuilding/src/Data/Bases/bow.lua"),
    "Data/Bases/wand.lua" => include_str!("../PathOfBuilding/src/Data/Bases/wand.lua"),
    "Data/Bases/staff.lua" => include_str!("../PathOfBuilding/src/Data/Bases/staff.lua"),
    "Data/Bases/tincture.lua" => include_str!("../PathOfBuilding/src/Data/Bases/tincture.lua"),
    "Data/Bases/mace.lua" => include_str!("../PathOfBuilding/src/Data/Bases/mace.lua"),
    "Data/Bases/claw.lua" => include_str!("../PathOfBuilding/src/Data/Bases/claw.lua"),
    "Data/Bases/helmet.lua" => include_str!("../PathOfBuilding/src/Data/Bases/helmet.lua"),
    "Data/Bases/jewel.lua" => include_str!("../PathOfBuilding/src/Data/Bases/jewel.lua"),
    "Data/Bases/sword.lua" => include_str!("../PathOfBuilding/src/Data/Bases/sword.lua"),
    "Data/Bases/belt.lua" => include_str!("../PathOfBuilding/src/Data/Bases/belt.lua"),
    "Data/Bases/shield.lua" => include_str!("../PathOfBuilding/src/Data/Bases/shield.lua"),
    "Data/Bases/amulet.lua" => include_str!("../PathOfBuilding/src/Data/Bases/amulet.lua"),
    "Data/Bases/ring.lua" => include_str!("../PathOfBuilding/src/Data/Bases/ring.lua"),
];
