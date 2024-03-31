use std::{ops::{BitAnd, BitOr}, time::{SystemTime, UNIX_EPOCH}};

use mlua::prelude::*;
mod lua_sources;
fn run() -> Result<(), LuaError> {
    let lua = unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) };

    let get_time = lua
        .create_function(|_, ()| -> LuaResult<i64> {
            let now = SystemTime::now();
            let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let milliseconds_since_epoch = duration_since_epoch.as_millis();
            Ok(milliseconds_since_epoch.try_into().unwrap())
        })
        .unwrap();
    lua.globals().set("GetTime", get_time)?;

    let set_window_title = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("SetWindowTitle", set_window_title)?;

    let bit = lua.create_table()?;
    bit.set(
        "rshift",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x >> y) })?,
    )?;
    bit.set(
        "band",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x.bitand(y)) })?,
    )?;
    bit.set(
        "bor",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x.bitor(y)) })?,
    )?;
    bit.set(
        "bxor",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x ^ y) })?,
    )?;
    bit.set(
        "bnot",
        lua.create_function(|_, x: i32| -> LuaResult<i32> { Ok(!x) })?,
    )?;
    lua.globals().set("bit", bit)?;

    let require = lua.create_function(|lua, module: LuaString| -> LuaResult<Option<LuaTable>> {
        let source = match module.to_str()? {
            "xml" => include_str!("../PathOfBuilding/runtime/lua/xml.lua"),
            "base64" => include_str!("../PathOfBuilding/runtime/lua/base64.lua"),
            "sha1" => include_str!("../PathOfBuilding/runtime/lua/sha1/init.lua"),
            "sha1.common" => include_str!("../PathOfBuilding/runtime/lua/sha1/common.lua"),

            "bit" => "return bit",
            "sha1.bit_ops" => include_str!("../PathOfBuilding/runtime/lua/sha1/bit_ops.lua"),

            "lua-profiler" => "return nil",

            // "bit32" => "return { }",
            _ => unimplemented!("Require: {} is not yet implemented", module.to_str()?),
        };

        let chunk = lua.load(source);
        let func = chunk.into_function().unwrap();
        let table: Option<LuaTable> = func.call(()).unwrap();
        Ok(table)
    })?;
    lua.globals().set("require", require)?;

    let pload_module = lua.create_function(|lua, module: LuaString| -> LuaResult<LuaTable> {
        let source = match module.to_str()? {
            "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
            _ => unimplemented!("{} is not yet implemented", module.to_str()?),
        };

        let chunk = lua.load(source);
        let module_function = chunk.into_function().unwrap();
        let module_table: LuaTable = module_function.call(()).unwrap();
        Ok(module_table)
    })?;
    lua.globals().set("PLoadModule", pload_module)?;

    let load_module = lua.create_function(
        |lua, (module, lmv): (LuaString, LuaMultiValue)| -> LuaResult<Option<LuaValue>> {
            let source = match module.to_str()? {
                "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
                "GameVersions" => {
                    include_str!("../PathOfBuilding/src/GameVersions.lua")
                }
                "Modules/Common" => include_str!("../PathOfBuilding/src/Modules/Common.lua"),
                "Modules/Data" => include_str!("../PathOfBuilding/src/Modules/Data.lua"),
                "Modules/ModTools" => include_str!("../PathOfBuilding/src/Modules/ModTools.lua"),
                "Modules/ItemTools" => include_str!("../PathOfBuilding/src/Modules/ItemTools.lua"),
                "Modules/CalcTools" => include_str!("../PathOfBuilding/src/Modules/CalcTools.lua"),
                "Modules/PantheonTools" => {
                    include_str!("../PathOfBuilding/src/Modules/PantheonTools.lua")
                }
                "Modules/BuildSiteTools" => {
                    include_str!("../PathOfBuilding/src/Modules/BuildSiteTools.lua")
                }
                "Modules/StatDescriber" => {
                    include_str!("../PathOfBuilding/src/Modules/StatDescriber.lua")
                }
                "Data/Global" => {
                    include_str!("../PathOfBuilding/src/Data/Global.lua")
                }
                "Data/Misc" => {
                    include_str!("../PathOfBuilding/src/Data/Misc.lua")
                }
                "Data/ModItem" => {
                    include_str!("../PathOfBuilding/src/Data/ModItem.lua")
                }
                "Data/ModFlask" => {
                    include_str!("../PathOfBuilding/src/Data/ModFlask.lua")
                }
                "Data/ModTincture" => {
                    include_str!("../PathOfBuilding/src/Data/ModTincture.lua")
                }
                "Data/ModJewel" => {
                    include_str!("../PathOfBuilding/src/Data/ModJewel.lua")
                }
                "Data/ModJewelAbyss" => {
                    include_str!("../PathOfBuilding/src/Data/ModJewelAbyss.lua")
                }
                "Data/ModJewelCluster" => {
                    include_str!("../PathOfBuilding/src/Data/ModJewelCluster.lua")
                }
                "Data/ModJewelCharm" => {
                    include_str!("../PathOfBuilding/src/Data/ModJewelCharm.lua")
                }
                "Data/ModMaster" => {
                    include_str!("../PathOfBuilding/src/Data/ModMaster.lua")
                }
                "Data/EnchantmentHelmet" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentHelmet.lua")
                }
                "Data/EnchantmentBoots" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentBoots.lua")
                }
                "Data/EnchantmentGloves" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentGloves.lua")
                }
                "Data/EnchantmentBelt" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentBelt.lua")
                }
                "Data/EnchantmentBody" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentBody.lua")
                }
                "Data/EnchantmentWeapon" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentWeapon.lua")
                }
                "Data/EnchantmentFlask" => {
                    include_str!("../PathOfBuilding/src/Data/EnchantmentFlask.lua")
                }

                "Data/Essence" => {
                    include_str!("../PathOfBuilding/src/Data/Essence.lua")
                }
                "Data/ModVeiled" => {
                    include_str!("../PathOfBuilding/src/Data/ModVeiled.lua")
                }
                "Data/ModNecropolis" => {
                    include_str!("../PathOfBuilding/src/Data/ModNecropolis.lua")
                }
                "Data/Crucible" => {
                    include_str!("../PathOfBuilding/src/Data/Crucible.lua")
                }
                "Data/Pantheons" => {
                    include_str!("../PathOfBuilding/src/Data/Pantheons.lua")
                }
                "Data/Costs" => {
                    include_str!("../PathOfBuilding/src/Data/Costs.lua")
                }
                "Data/ModMap" => {
                    include_str!("../PathOfBuilding/src/Data/ModMap.lua")
                }
                "Data/ClusterJewels" => {
                    include_str!("../PathOfBuilding/src/Data/ClusterJewels.lua")
                }
                "Data/TimelessJewelData/LegionTradeIds" => {
                    include_str!("../PathOfBuilding/src/Data/TimelessJewelData/LegionTradeIds.lua")
                }
                "Data/TimelessJewelData/NodeIndexMapping" => {
                    include_str!("../PathOfBuilding/src/Data/TimelessJewelData/NodeIndexMapping.lua")
                }
                "Modules/DataLegionLookUpTableHelper" => {
                    include_str!("../PathOfBuilding/src/Modules/DataLegionLookUpTableHelper.lua")
                }
                "Data/Bosses" => {
                    include_str!("../PathOfBuilding/src/Data/Bosses.lua")
                }
                "Data/BossSkills" => {
                    include_str!("../PathOfBuilding/src/Data/BossSkills.lua")
                }
                "Data/SkillStatMap" => {
                    include_str!("../PathOfBuilding/src/Data/SkillStatMap.lua")
                }
                "Data/Skills/act_str" => {
                    include_str!("../PathOfBuilding/src/Data/Skills/act_str.lua")
                }
                _ => unimplemented!("{} is not yet implemented", module.to_str()?),
            };

            let chunk = lua.load(source);
            let chunk = chunk.set_name(module.to_str().unwrap());
            let module_function = chunk.into_function().unwrap();

            let module_results: Option<LuaTable> = match module_function.call(lmv.clone()) {
                Ok(module) => return Ok(module),
                Err(e) => panic!("Failed to include module {}, error: {}", module.to_str().unwrap(), e),
            };
            // let module_table: Option<LuaTable> = match module_function.call(lmv.clone()) {
            //     Ok(x) => x,
            //     Err(e) => {
            //         let module_table: LuaResult<Option<LuaFunction>> =
            //             module_function.call(lmv);
            //             match module_table {
            //                 Ok(Some(x)) => return Ok(Some(LuaValue::Function(x))),
            //                 Ok(None) => unreachable!("more so curious if this is ever triggered"),
            //                 Err(e) => panic!(""),
            //             }
            //         // panic!(
            //         //     "Failed to import module {}: {}",
            //         //     module.to_str().unwrap(),
            //         //     e.to_string()
            //         // );
            //     }
            // };

            // Ok(module_table.map(|mt| LuaValue::Table(mt)))
        },
    )?;
    lua.globals().set("LoadModule", load_module)?;

    let con_execute = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("ConExecute", con_execute)?;

    let render_init = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("RenderInit", render_init)?;

    let con_printf = lua.create_function(|_, val: LuaValue| -> LuaResult<()> {
        println!("con_printf: {:#?}", val);
        Ok(())
    })?;
    lua.globals().set("ConPrintf", con_printf)?;

    let set_main_object = lua
        .create_function(|_, obj: mlua::Table| -> LuaResult<()> {
            // state.inspect_stack(0).unwrap().stack().num_ups
            // state.inspect_stack(0).unwrap().

            // dbg!("It passed in a table", &obj);
            Ok(())
        })
        .unwrap();
    lua.globals().set("SetMainObject", set_main_object)?;

    let c = lua.load(
        include_str!("../PathOfBuilding/src/Launch.lua")
            .replace("#@ SimpleGraphic", "-- #@ SimpleGraphic"),
    );
    c.exec()?;

    // lua.globals()
    //     .get::<_, LuaTable>("package")?
    //     .get::<_, LuaTable>("preload")?
    //     .set("xml", lua.create_table().unwrap())?;

    let launch: LuaTable = lua.globals().get("launch")?;
    let on_init: LuaFunction = launch.get("OnInit")?;

    let this = lua.create_table()?;
    on_init.call::<_, ()>(this)?;

    Ok(())
}

fn main() {
    println!("Hello emscripten");

    match run() {
        Ok(_) => println!("Ran and exited successfully"),
        Err(LuaError::RuntimeError(e)) => {
            println!("Runtime Error: {}", e.replace("\\\\n", "\n"))
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
