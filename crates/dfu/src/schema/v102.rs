use crate::util::v102_item_name;
use dfu_schema_define::define_schema;
use dfu_structures::types::{ConversionFunc, Types};
use dfu_structures::SchemaInfo;
use quartz_nbt::{NbtCompound, NbtTag};

pub fn define(types: &mut Types<&'static ConversionFunc>) {
    let info = SchemaInfo {
        version: 102,
        references: types,
    };
    define_schema!(info, {
        register item_name {
            custom(|value, _from, _to| {
                let id = match value {
                    NbtTag::Int(val) => val,
                    _ => return Ok(None)
                };
                if let Some(name) = v102_item_name::ITEM_NAMES.get(id) {
                    Ok(Some(name))
                } else {
                    println!("(v102) Unknown legacy id {}", id);
                    return Ok(Some(v102_item_name::ITEM_NAMES.get(&0).unwrap()));
                }
            })
        },
        register item_stack {
            custom(|value, _from, _to| {
                let compound = match value {
                    NbtTag::Compound(compound) => compound,
                    _ => return Ok(None::<()>)
                };
                let id: i32 = compound.get("id")?;

                let name = if let Some(name) = v102_item_name::ITEM_NAMES.get(&id) {
                    name
                } else {
                    println!("(v102) Unknown legacy id {}", id);
                    v102_item_name::ITEM_NAMES.get(&0).unwrap()
                };
                compound.insert("id", NbtTag::String(name.to_string()));

                Ok(None)
            })
        },
        register item_stack { // FIXME: register only for potion items, not all items
            custom(|value, _from, _to| {
                let compound = match value {
                    NbtTag::Compound(compound) => compound,
                    _ => return Ok(None::<()>)
                };
                let damage: i16 = compound.get("Damage")?;
                if damage != 0 {
                    compound.insert("Damage", NbtTag::Short(damage));
                }

                let tag: Option<&mut NbtTag> = compound.inner_mut().get_mut("tag");
                let tag_compound: &mut NbtCompound;
                if tag.is_none() {
                    compound.insert("tag", NbtCompound::new());
                    tag_compound = compound.get_mut::<_, &mut NbtCompound>("tag")?;
                } else {
                    tag_compound = tag.unwrap().try_into()?
                };

                if tag_compound.get::<_, &String>("Potion").is_err() {
                    let name = v102_item_name::POTION_NAMES[damage as usize];
                    tag_compound.insert("Potion", name.unwrap_or("minecraft:water"));
                    if damage & 16384 == 16384 {
                        compound.insert("id", "minecraft:splash_potion");
                    }
                }
                Ok(None)
            })
        }
    });
}
