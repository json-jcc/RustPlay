
use std::path::Path;
use mlua::prelude::*;


// pub fn create_chunk(path: &str) -> LuaResult<LuaChunk> {
//     let mut file = File::open(path)?;
//     let mut chunk = vec![];
//     file.read_to_end(&mut chunk)?;
//     Ok(chunk.into())
// }


pub fn test_lua() -> LuaResult<()> {
    let lua = Lua::new();

    let map_table = lua.create_table()?;
    map_table.set(1, "one")?;
    map_table.set("two", 2)?;
    
    lua.globals().set("map_table", map_table)?;
    
    lua.load(Path::new("scripts/test.lua")).exec()?;

    Ok(())
}