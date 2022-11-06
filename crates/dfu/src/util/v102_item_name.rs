use lazy_static::lazy_static;
use std::collections::HashMap;

// This class is responsible for mapping the id -> string update in itemstacks and potions

lazy_static! {
    pub static ref ITEM_NAMES: HashMap<i32, &'static str> = create_item_names();
    pub static ref POTION_NAMES: [Option<&'static str>; 128] = create_potion_names();
}

fn create_item_names() -> HashMap<i32, &'static str> {
    let mut names = HashMap::new();
    names.insert(0, "minecraft:air");
    names.insert(1, "minecraft:stone");
    names.insert(2, "minecraft:grass");
    names.insert(3, "minecraft:dirt");
    names.insert(4, "minecraft:cobblestone");
    names.insert(5, "minecraft:planks");
    names.insert(6, "minecraft:sapling");
    names.insert(7, "minecraft:bedrock");
    names.insert(8, "minecraft:flowing_water");
    names.insert(9, "minecraft:water");
    names.insert(10, "minecraft:flowing_lava");
    names.insert(11, "minecraft:lava");
    names.insert(12, "minecraft:sand");
    names.insert(13, "minecraft:gravel");
    names.insert(14, "minecraft:gold_ore");
    names.insert(15, "minecraft:iron_ore");
    names.insert(16, "minecraft:coal_ore");
    names.insert(17, "minecraft:log");
    names.insert(18, "minecraft:leaves");
    names.insert(19, "minecraft:sponge");
    names.insert(20, "minecraft:glass");
    names.insert(21, "minecraft:lapis_ore");
    names.insert(22, "minecraft:lapis_block");
    names.insert(23, "minecraft:dispenser");
    names.insert(24, "minecraft:sandstone");
    names.insert(25, "minecraft:noteblock");
    names.insert(27, "minecraft:golden_rail");
    names.insert(28, "minecraft:detector_rail");
    names.insert(29, "minecraft:sticky_piston");
    names.insert(30, "minecraft:web");
    names.insert(31, "minecraft:tallgrass");
    names.insert(32, "minecraft:deadbush");
    names.insert(33, "minecraft:piston");
    names.insert(35, "minecraft:wool");
    names.insert(37, "minecraft:yellow_flower");
    names.insert(38, "minecraft:red_flower");
    names.insert(39, "minecraft:brown_mushroom");
    names.insert(40, "minecraft:red_mushroom");
    names.insert(41, "minecraft:gold_block");
    names.insert(42, "minecraft:iron_block");
    names.insert(43, "minecraft:double_stone_slab");
    names.insert(44, "minecraft:stone_slab");
    names.insert(45, "minecraft:brick_block");
    names.insert(46, "minecraft:tnt");
    names.insert(47, "minecraft:bookshelf");
    names.insert(48, "minecraft:mossy_cobblestone");
    names.insert(49, "minecraft:obsidian");
    names.insert(50, "minecraft:torch");
    names.insert(51, "minecraft:fire");
    names.insert(52, "minecraft:mob_spawner");
    names.insert(53, "minecraft:oak_stairs");
    names.insert(54, "minecraft:chest");
    names.insert(56, "minecraft:diamond_ore");
    names.insert(57, "minecraft:diamond_block");
    names.insert(58, "minecraft:crafting_table");
    names.insert(60, "minecraft:farmland");
    names.insert(61, "minecraft:furnace");
    names.insert(62, "minecraft:lit_furnace");
    names.insert(65, "minecraft:ladder");
    names.insert(66, "minecraft:rail");
    names.insert(67, "minecraft:stone_stairs");
    names.insert(69, "minecraft:lever");
    names.insert(70, "minecraft:stone_pressure_plate");
    names.insert(72, "minecraft:wooden_pressure_plate");
    names.insert(73, "minecraft:redstone_ore");
    names.insert(76, "minecraft:redstone_torch");
    names.insert(77, "minecraft:stone_button");
    names.insert(78, "minecraft:snow_layer");
    names.insert(79, "minecraft:ice");
    names.insert(80, "minecraft:snow");
    names.insert(81, "minecraft:cactus");
    names.insert(82, "minecraft:clay");
    names.insert(84, "minecraft:jukebox");
    names.insert(85, "minecraft:fence");
    names.insert(86, "minecraft:pumpkin");
    names.insert(87, "minecraft:netherrack");
    names.insert(88, "minecraft:soul_sand");
    names.insert(89, "minecraft:glowstone");
    names.insert(90, "minecraft:portal");
    names.insert(91, "minecraft:lit_pumpkin");
    names.insert(95, "minecraft:stained_glass");
    names.insert(96, "minecraft:trapdoor");
    names.insert(97, "minecraft:monster_egg");
    names.insert(98, "minecraft:stonebrick");
    names.insert(99, "minecraft:brown_mushroom_block");
    names.insert(100, "minecraft:red_mushroom_block");
    names.insert(101, "minecraft:iron_bars");
    names.insert(102, "minecraft:glass_pane");
    names.insert(103, "minecraft:melon_block");
    names.insert(106, "minecraft:vine");
    names.insert(107, "minecraft:fence_gate");
    names.insert(108, "minecraft:brick_stairs");
    names.insert(109, "minecraft:stone_brick_stairs");
    names.insert(110, "minecraft:mycelium");
    names.insert(111, "minecraft:waterlily");
    names.insert(112, "minecraft:nether_brick");
    names.insert(113, "minecraft:nether_brick_fence");
    names.insert(114, "minecraft:nether_brick_stairs");
    names.insert(116, "minecraft:enchanting_table");
    names.insert(119, "minecraft:end_portal");
    names.insert(120, "minecraft:end_portal_frame");
    names.insert(121, "minecraft:end_stone");
    names.insert(122, "minecraft:dragon_egg");
    names.insert(123, "minecraft:redstone_lamp");
    names.insert(125, "minecraft:double_wooden_slab");
    names.insert(126, "minecraft:wooden_slab");
    names.insert(127, "minecraft:cocoa");
    names.insert(128, "minecraft:sandstone_stairs");
    names.insert(129, "minecraft:emerald_ore");
    names.insert(130, "minecraft:ender_chest");
    names.insert(131, "minecraft:tripwire_hook");
    names.insert(133, "minecraft:emerald_block");
    names.insert(134, "minecraft:spruce_stairs");
    names.insert(135, "minecraft:birch_stairs");
    names.insert(136, "minecraft:jungle_stairs");
    names.insert(137, "minecraft:command_block");
    names.insert(138, "minecraft:beacon");
    names.insert(139, "minecraft:cobblestone_wall");
    names.insert(141, "minecraft:carrots");
    names.insert(142, "minecraft:potatoes");
    names.insert(143, "minecraft:wooden_button");
    names.insert(145, "minecraft:anvil");
    names.insert(146, "minecraft:trapped_chest");
    names.insert(147, "minecraft:light_weighted_pressure_plate");
    names.insert(148, "minecraft:heavy_weighted_pressure_plate");
    names.insert(151, "minecraft:daylight_detector");
    names.insert(152, "minecraft:redstone_block");
    names.insert(153, "minecraft:quartz_ore");
    names.insert(154, "minecraft:hopper");
    names.insert(155, "minecraft:quartz_block");
    names.insert(156, "minecraft:quartz_stairs");
    names.insert(157, "minecraft:activator_rail");
    names.insert(158, "minecraft:dropper");
    names.insert(159, "minecraft:stained_hardened_clay");
    names.insert(160, "minecraft:stained_glass_pane");
    names.insert(161, "minecraft:leaves2");
    names.insert(162, "minecraft:log2");
    names.insert(163, "minecraft:acacia_stairs");
    names.insert(164, "minecraft:dark_oak_stairs");
    names.insert(170, "minecraft:hay_block");
    names.insert(171, "minecraft:carpet");
    names.insert(172, "minecraft:hardened_clay");
    names.insert(173, "minecraft:coal_block");
    names.insert(174, "minecraft:packed_ice");
    names.insert(175, "minecraft:double_plant");
    names.insert(256, "minecraft:iron_shovel");
    names.insert(257, "minecraft:iron_pickaxe");
    names.insert(258, "minecraft:iron_axe");
    names.insert(259, "minecraft:flint_and_steel");
    names.insert(260, "minecraft:apple");
    names.insert(261, "minecraft:bow");
    names.insert(262, "minecraft:arrow");
    names.insert(263, "minecraft:coal");
    names.insert(264, "minecraft:diamond");
    names.insert(265, "minecraft:iron_ingot");
    names.insert(266, "minecraft:gold_ingot");
    names.insert(267, "minecraft:iron_sword");
    names.insert(268, "minecraft:wooden_sword");
    names.insert(269, "minecraft:wooden_shovel");
    names.insert(270, "minecraft:wooden_pickaxe");
    names.insert(271, "minecraft:wooden_axe");
    names.insert(272, "minecraft:stone_sword");
    names.insert(273, "minecraft:stone_shovel");
    names.insert(274, "minecraft:stone_pickaxe");
    names.insert(275, "minecraft:stone_axe");
    names.insert(276, "minecraft:diamond_sword");
    names.insert(277, "minecraft:diamond_shovel");
    names.insert(278, "minecraft:diamond_pickaxe");
    names.insert(279, "minecraft:diamond_axe");
    names.insert(280, "minecraft:stick");
    names.insert(281, "minecraft:bowl");
    names.insert(282, "minecraft:mushroom_stew");
    names.insert(283, "minecraft:golden_sword");
    names.insert(284, "minecraft:golden_shovel");
    names.insert(285, "minecraft:golden_pickaxe");
    names.insert(286, "minecraft:golden_axe");
    names.insert(287, "minecraft:string");
    names.insert(288, "minecraft:feather");
    names.insert(289, "minecraft:gunpowder");
    names.insert(290, "minecraft:wooden_hoe");
    names.insert(291, "minecraft:stone_hoe");
    names.insert(292, "minecraft:iron_hoe");
    names.insert(293, "minecraft:diamond_hoe");
    names.insert(294, "minecraft:golden_hoe");
    names.insert(295, "minecraft:wheat_seeds");
    names.insert(296, "minecraft:wheat");
    names.insert(297, "minecraft:bread");
    names.insert(298, "minecraft:leather_helmet");
    names.insert(299, "minecraft:leather_chestplate");
    names.insert(300, "minecraft:leather_leggings");
    names.insert(301, "minecraft:leather_boots");
    names.insert(302, "minecraft:chainmail_helmet");
    names.insert(303, "minecraft:chainmail_chestplate");
    names.insert(304, "minecraft:chainmail_leggings");
    names.insert(305, "minecraft:chainmail_boots");
    names.insert(306, "minecraft:iron_helmet");
    names.insert(307, "minecraft:iron_chestplate");
    names.insert(308, "minecraft:iron_leggings");
    names.insert(309, "minecraft:iron_boots");
    names.insert(310, "minecraft:diamond_helmet");
    names.insert(311, "minecraft:diamond_chestplate");
    names.insert(312, "minecraft:diamond_leggings");
    names.insert(313, "minecraft:diamond_boots");
    names.insert(314, "minecraft:golden_helmet");
    names.insert(315, "minecraft:golden_chestplate");
    names.insert(316, "minecraft:golden_leggings");
    names.insert(317, "minecraft:golden_boots");
    names.insert(318, "minecraft:flint");
    names.insert(319, "minecraft:porkchop");
    names.insert(320, "minecraft:cooked_porkchop");
    names.insert(321, "minecraft:painting");
    names.insert(322, "minecraft:golden_apple");
    names.insert(323, "minecraft:sign");
    names.insert(324, "minecraft:wooden_door");
    names.insert(325, "minecraft:bucket");
    names.insert(326, "minecraft:water_bucket");
    names.insert(327, "minecraft:lava_bucket");
    names.insert(328, "minecraft:minecart");
    names.insert(329, "minecraft:saddle");
    names.insert(330, "minecraft:iron_door");
    names.insert(331, "minecraft:redstone");
    names.insert(332, "minecraft:snowball");
    names.insert(333, "minecraft:boat");
    names.insert(334, "minecraft:leather");
    names.insert(335, "minecraft:milk_bucket");
    names.insert(336, "minecraft:brick");
    names.insert(337, "minecraft:clay_ball");
    names.insert(338, "minecraft:reeds");
    names.insert(339, "minecraft:paper");
    names.insert(340, "minecraft:book");
    names.insert(341, "minecraft:slime_ball");
    names.insert(342, "minecraft:chest_minecart");
    names.insert(343, "minecraft:furnace_minecart");
    names.insert(344, "minecraft:egg");
    names.insert(345, "minecraft:compass");
    names.insert(346, "minecraft:fishing_rod");
    names.insert(347, "minecraft:clock");
    names.insert(348, "minecraft:glowstone_dust");
    names.insert(349, "minecraft:fish");
    names.insert(350, "minecraft:cooked_fish"); // Fix typo, the game never recognized cooked_fished
    names.insert(351, "minecraft:dye");
    names.insert(352, "minecraft:bone");
    names.insert(353, "minecraft:sugar");
    names.insert(354, "minecraft:cake");
    names.insert(355, "minecraft:bed");
    names.insert(356, "minecraft:repeater");
    names.insert(357, "minecraft:cookie");
    names.insert(358, "minecraft:filled_map");
    names.insert(359, "minecraft:shears");
    names.insert(360, "minecraft:melon");
    names.insert(361, "minecraft:pumpkin_seeds");
    names.insert(362, "minecraft:melon_seeds");
    names.insert(363, "minecraft:beef");
    names.insert(364, "minecraft:cooked_beef");
    names.insert(365, "minecraft:chicken");
    names.insert(366, "minecraft:cooked_chicken");
    names.insert(367, "minecraft:rotten_flesh");
    names.insert(368, "minecraft:ender_pearl");
    names.insert(369, "minecraft:blaze_rod");
    names.insert(370, "minecraft:ghast_tear");
    names.insert(371, "minecraft:gold_nugget");
    names.insert(372, "minecraft:nether_wart");
    names.insert(373, "minecraft:potion");
    names.insert(374, "minecraft:glass_bottle");
    names.insert(375, "minecraft:spider_eye");
    names.insert(376, "minecraft:fermented_spider_eye");
    names.insert(377, "minecraft:blaze_powder");
    names.insert(378, "minecraft:magma_cream");
    names.insert(379, "minecraft:brewing_stand");
    names.insert(380, "minecraft:cauldron");
    names.insert(381, "minecraft:ender_eye");
    names.insert(382, "minecraft:speckled_melon");
    names.insert(383, "minecraft:spawn_egg");
    names.insert(384, "minecraft:experience_bottle");
    names.insert(385, "minecraft:fire_charge");
    names.insert(386, "minecraft:writable_book");
    names.insert(387, "minecraft:written_book");
    names.insert(388, "minecraft:emerald");
    names.insert(389, "minecraft:item_frame");
    names.insert(390, "minecraft:flower_pot");
    names.insert(391, "minecraft:carrot");
    names.insert(392, "minecraft:potato");
    names.insert(393, "minecraft:baked_potato");
    names.insert(394, "minecraft:poisonous_potato");
    names.insert(395, "minecraft:map");
    names.insert(396, "minecraft:golden_carrot");
    names.insert(397, "minecraft:skull");
    names.insert(398, "minecraft:carrot_on_a_stick");
    names.insert(399, "minecraft:nether_star");
    names.insert(400, "minecraft:pumpkin_pie");
    names.insert(401, "minecraft:fireworks");
    names.insert(402, "minecraft:firework_charge");
    names.insert(403, "minecraft:enchanted_book");
    names.insert(404, "minecraft:comparator");
    names.insert(405, "minecraft:netherbrick");
    names.insert(406, "minecraft:quartz");
    names.insert(407, "minecraft:tnt_minecart");
    names.insert(408, "minecraft:hopper_minecart");
    names.insert(417, "minecraft:iron_horse_armor");
    names.insert(418, "minecraft:golden_horse_armor");
    names.insert(419, "minecraft:diamond_horse_armor");
    names.insert(420, "minecraft:lead");
    names.insert(421, "minecraft:name_tag");
    names.insert(422, "minecraft:command_block_minecart");
    names.insert(2256, "minecraft:record_13");
    names.insert(2257, "minecraft:record_cat");
    names.insert(2258, "minecraft:record_blocks");
    names.insert(2259, "minecraft:record_chirp");
    names.insert(2260, "minecraft:record_far");
    names.insert(2261, "minecraft:record_mall");
    names.insert(2262, "minecraft:record_mellohi");
    names.insert(2263, "minecraft:record_stal");
    names.insert(2264, "minecraft:record_strad");
    names.insert(2265, "minecraft:record_ward");
    names.insert(2266, "minecraft:record_11");
    names.insert(2267, "minecraft:record_wait");
    // https://github.com/starlis/empirecraft/commit/2da59d1901407fc0c135ef0458c0fe9b016570b3
    // It's likely that this is a result of old CB/Spigot behavior still writing ids into items as ints.
    // These ids do not appear to be used by regular MC anyways, so I do not see the harm of porting it here.
    // Extras can be added if needed
    let mut extra = [None; 4000];
    // EMC start
    extra[409] = Some("minecraft:prismarine_shard");
    extra[410] = Some("minecraft:prismarine_crystals");
    extra[411] = Some("minecraft:rabbit");
    extra[412] = Some("minecraft:cooked_rabbit");
    extra[413] = Some("minecraft:rabbit_stew");
    extra[414] = Some("minecraft:rabbit_foot");
    extra[415] = Some("minecraft:rabbit_hide");
    extra[416] = Some("minecraft:armor_stand");
    extra[423] = Some("minecraft:mutton");
    extra[424] = Some("minecraft:cooked_mutton");
    extra[425] = Some("minecraft:banner");
    extra[426] = Some("minecraft:end_crystal");
    extra[427] = Some("minecraft:spruce_door");
    extra[428] = Some("minecraft:birch_door");
    extra[429] = Some("minecraft:jungle_door");
    extra[430] = Some("minecraft:acacia_door");
    extra[431] = Some("minecraft:dark_oak_door");
    extra[432] = Some("minecraft:chorus_fruit");
    extra[433] = Some("minecraft:chorus_fruit_popped");
    extra[434] = Some("minecraft:beetroot");
    extra[435] = Some("minecraft:beetroot_seeds");
    extra[436] = Some("minecraft:beetroot_soup");
    extra[437] = Some("minecraft:dragon_breath");
    extra[438] = Some("minecraft:splash_potion");
    extra[439] = Some("minecraft:spectral_arrow");
    extra[440] = Some("minecraft:tipped_arrow");
    extra[441] = Some("minecraft:lingering_potion");
    extra[442] = Some("minecraft:shield");
    extra[443] = Some("minecraft:elytra");
    extra[444] = Some("minecraft:spruce_boat");
    extra[445] = Some("minecraft:birch_boat");
    extra[446] = Some("minecraft:jungle_boat");
    extra[447] = Some("minecraft:acacia_boat");
    extra[448] = Some("minecraft:dark_oak_boat");
    extra[449] = Some("minecraft:totem_of_undying");
    extra[450] = Some("minecraft:shulker_shell");
    extra[452] = Some("minecraft:iron_nugget");
    extra[453] = Some("minecraft:knowledge_book");
    // EMC end

    // dump extra into map
    for i in 0..extra.len() {
        if let Some(value) = extra[i] {
            names.insert(i as i32, value);
        }
    }

    // Add block ids into conversion as well
    // Very old versions of the game handled them, but it seems 1.8.8 did not parse them at all, so no conversion
    // was written.
    // block ids are only skipped (set to AIR) if there is no 1-1 replacement item.
    names.insert(26, "minecraft:bed"); // bed block
    names.insert(34, names.get(&0).unwrap()); // skip (piston head block)
    names.insert(55, "minecraft:redstone"); // redstone wire block
    names.insert(59, names.get(&0).unwrap()); // skip (wheat crop block)
    names.insert(63, "minecraft:sign"); // standing sign
    names.insert(64, "minecraft:wooden_door"); // wooden door block
    names.insert(68, "minecraft:sign"); // wall sign
    names.insert(71, "minecraft:iron_door"); // iron door block
    names.insert(74, "minecraft:redstone_ore"); // lit redstone ore block
    names.insert(75, "minecraft:redstone_torch"); // unlit redstone torch
    names.insert(83, "minecraft:reeds"); // sugar cane block
    names.insert(92, "minecraft:cake"); // cake block
    names.insert(93, "minecraft:repeater"); // unpowered repeater block
    names.insert(94, "minecraft:repeater"); // powered repeater block
    names.insert(104, names.get(&0).unwrap()); // skip (pumpkin stem)
    names.insert(105, names.get(&0).unwrap()); // skip (melon stem)
    names.insert(115, "minecraft:nether_wart"); // nether wart block
    names.insert(117, "minecraft:brewing_stand"); // brewing stand block
    names.insert(118, "minecraft:cauldron"); // cauldron block
    names.insert(124, "minecraft:redstone_lamp"); // lit redstone lamp block
    names.insert(132, names.get(&0).unwrap()); // skip (tripwire wire block)
    names.insert(140, "minecraft:flower_pot"); // flower pot block
    names.insert(144, "minecraft:skull"); // skull block
    names.insert(149, "minecraft:comparator"); // unpowered comparator block
    names.insert(150, "minecraft:comparator"); // powered comparator block
                                               // there are technically more, but at some point even older versions pre id -> name conversion didn't even load them.
                                               // (all I know is 1.7.10 does not load them)
                                               // and so given even the vanilla game wouldn't load them, there's no conversion path for them - they were never valid.

    names
}

fn create_potion_names() -> [Option<&'static str>; 128] {
    let mut names = [None; 128];
    names[0] = Some("minecraft:water");
    names[1] = Some("minecraft:regeneration");
    names[2] = Some("minecraft:swiftness");
    names[3] = Some("minecraft:fire_resistance");
    names[4] = Some("minecraft:poison");
    names[5] = Some("minecraft:healing");
    names[6] = Some("minecraft:night_vision");
    names[7] = None;
    names[8] = Some("minecraft:weakness");
    names[9] = Some("minecraft:strength");
    names[10] = Some("minecraft:slowness");
    names[11] = Some("minecraft:leaping");
    names[12] = Some("minecraft:harming");
    names[13] = Some("minecraft:water_breathing");
    names[14] = Some("minecraft:invisibility");
    names[15] = None;
    names[16] = Some("minecraft:awkward");
    names[17] = Some("minecraft:regeneration");
    names[18] = Some("minecraft:swiftness");
    names[19] = Some("minecraft:fire_resistance");
    names[20] = Some("minecraft:poison");
    names[21] = Some("minecraft:healing");
    names[22] = Some("minecraft:night_vision");
    names[23] = None;
    names[24] = Some("minecraft:weakness");
    names[25] = Some("minecraft:strength");
    names[26] = Some("minecraft:slowness");
    names[27] = Some("minecraft:leaping");
    names[28] = Some("minecraft:harming");
    names[29] = Some("minecraft:water_breathing");
    names[30] = Some("minecraft:invisibility");
    names[31] = None;
    names[32] = Some("minecraft:thick");
    names[33] = Some("minecraft:strong_regeneration");
    names[34] = Some("minecraft:strong_swiftness");
    names[35] = Some("minecraft:fire_resistance");
    names[36] = Some("minecraft:strong_poison");
    names[37] = Some("minecraft:strong_healing");
    names[38] = Some("minecraft:night_vision");
    names[39] = None;
    names[40] = Some("minecraft:weakness");
    names[41] = Some("minecraft:strong_strength");
    names[42] = Some("minecraft:slowness");
    names[43] = Some("minecraft:strong_leaping");
    names[44] = Some("minecraft:strong_harming");
    names[45] = Some("minecraft:water_breathing");
    names[46] = Some("minecraft:invisibility");
    names[47] = None;
    names[48] = None;
    names[49] = Some("minecraft:strong_regeneration");
    names[50] = Some("minecraft:strong_swiftness");
    names[51] = Some("minecraft:fire_resistance");
    names[52] = Some("minecraft:strong_poison");
    names[53] = Some("minecraft:strong_healing");
    names[54] = Some("minecraft:night_vision");
    names[55] = None;
    names[56] = Some("minecraft:weakness");
    names[57] = Some("minecraft:strong_strength");
    names[58] = Some("minecraft:slowness");
    names[59] = Some("minecraft:strong_leaping");
    names[60] = Some("minecraft:strong_harming");
    names[61] = Some("minecraft:water_breathing");
    names[62] = Some("minecraft:invisibility");
    names[63] = None;
    names[64] = Some("minecraft:mundane");
    names[65] = Some("minecraft:long_regeneration");
    names[66] = Some("minecraft:long_swiftness");
    names[67] = Some("minecraft:long_fire_resistance");
    names[68] = Some("minecraft:long_poison");
    names[69] = Some("minecraft:healing");
    names[70] = Some("minecraft:long_night_vision");
    names[71] = None;
    names[72] = Some("minecraft:long_weakness");
    names[73] = Some("minecraft:long_strength");
    names[74] = Some("minecraft:long_slowness");
    names[75] = Some("minecraft:long_leaping");
    names[76] = Some("minecraft:harming");
    names[77] = Some("minecraft:long_water_breathing");
    names[78] = Some("minecraft:long_invisibility");
    names[79] = None;
    names[80] = Some("minecraft:awkward");
    names[81] = Some("minecraft:long_regeneration");
    names[82] = Some("minecraft:long_swiftness");
    names[83] = Some("minecraft:long_fire_resistance");
    names[84] = Some("minecraft:long_poison");
    names[85] = Some("minecraft:healing");
    names[86] = Some("minecraft:long_night_vision");
    names[87] = None;
    names[88] = Some("minecraft:long_weakness");
    names[89] = Some("minecraft:long_strength");
    names[90] = Some("minecraft:long_slowness");
    names[91] = Some("minecraft:long_leaping");
    names[92] = Some("minecraft:harming");
    names[93] = Some("minecraft:long_water_breathing");
    names[94] = Some("minecraft:long_invisibility");
    names[95] = None;
    names[96] = Some("minecraft:thick");
    names[97] = Some("minecraft:regeneration");
    names[98] = Some("minecraft:swiftness");
    names[99] = Some("minecraft:long_fire_resistance");
    names[100] = Some("minecraft:poison");
    names[101] = Some("minecraft:strong_healing");
    names[102] = Some("minecraft:long_night_vision");
    names[103] = None;
    names[104] = Some("minecraft:long_weakness");
    names[105] = Some("minecraft:strength");
    names[106] = Some("minecraft:long_slowness");
    names[107] = Some("minecraft:leaping");
    names[108] = Some("minecraft:strong_harming");
    names[109] = Some("minecraft:long_water_breathing");
    names[110] = Some("minecraft:long_invisibility");
    names[111] = None;
    names[112] = None;
    names[113] = Some("minecraft:regeneration");
    names[114] = Some("minecraft:swiftness");
    names[115] = Some("minecraft:long_fire_resistance");
    names[116] = Some("minecraft:poison");
    names[117] = Some("minecraft:strong_healing");
    names[118] = Some("minecraft:long_night_vision");
    names[119] = None;
    names[120] = Some("minecraft:long_weakness");
    names[121] = Some("minecraft:strength");
    names[122] = Some("minecraft:long_slowness");
    names[123] = Some("minecraft:leaping");
    names[124] = Some("minecraft:strong_harming");
    names[125] = Some("minecraft:long_water_breathing");
    names[126] = Some("minecraft:long_invisibility");
    names[127] = None;

    names
}
