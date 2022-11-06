use dfu_schema_define::define_schema;
use dfu_structures::types::{ConversionFunc, Types};
use dfu_structures::SchemaInfo;

pub fn define(types: &mut Types<&'static ConversionFunc>) {
    let info = SchemaInfo {
        version: 99,
        references: types,
    };
    define_schema!(info, {
        register player {
            opt("Inventory", list(refer(item_stack))),
            opt("EnderItems", list(refer(item_stack)))
        },
        register chunk {
            req("Level", all(
                opt("Entities", list(refer(entity))),
                opt("TileEntitites", list(refer(tile_entity))),
                opt("TileTicks", list(req("i", refer(block_name))))
            ))
        },
        register entity_chunk {
            opt("Entities", list(refer(entity)))
        },
        register saved_data {
            opt("data", all(
                opt("Features", map_values(refer(structure_feature))),
                opt("Objectives", list(refer(objective))),
                opt("Teams", list(refer(team)))
            ))
        }
    });
}
