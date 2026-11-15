//! Gravestone Core Logic and Mod Implementation

pub const MOD_ID: &str = "gravestone_core";
pub const VERSION: &str = "1.0.0";
pub const AUTHOR: &str = "wareal-nah";
pub const LICENSE: &str = "MIT";
pub const DESCRIPTION: &str = "Ported from Gravestone Mod by henkelmax (MIT)";

pub const GRAVESTONE_BLOCK: &str = "gravestone:gravestone";
pub const OBITUARY_ITEM: &str = "gravestone:obituary";

pub const MIN_BUILD_HEIGHT: i32 = -64;
pub const MAX_BUILD_HEIGHT: i32 = 320;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraveItem {
    pub slot: usize,
    pub item: String,
    pub count: u32,
    pub damage: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraveData {
    pub id: String,
    pub player_uuid: u128,
    pub player_name: String,
    pub dimension: String,
    pub pos: [i32; 3],
    pub timestamp: u64,
    pub items: Vec<GraveItem>,
}

/// Checks whether a block is considered replaceable for grave placement.
pub fn is_replaceable_block(block: &str) -> bool {
    matches!(
        block,
        "" | "minecraft:air"
            | "minecraft:cave_air"
            | "minecraft:void_air"
            | "minecraft:water"
            | "minecraft:flowing_water"
            | "minecraft:lava"
            | "minecraft:flowing_lava"
            | "minecraft:short_grass"
            | "minecraft:tall_grass"
            | "minecraft:grass"
            | "minecraft:seagrass"
            | "minecraft:tall_seagrass"
            | "minecraft:kelp"
            | "minecraft:dandelion"
            | "minecraft:poppy"
            | "minecraft:blue_orchid"
            | "minecraft:allium"
            | "minecraft:azure_bluet"
            | "minecraft:red_tulip"
            | "minecraft:orange_tulip"
            | "minecraft:white_tulip"
            | "minecraft:pink_tulip"
            | "minecraft:oxeye_daisy"
            | "minecraft:cornflower"
            | "minecraft:lily_of_the_valley"
            | "minecraft:sunflower"
            | "minecraft:lilac"
            | "minecraft:rose_bush"
            | "minecraft:peony"
            | "minecraft:dead_bush"
            | "minecraft:fern"
            | "minecraft:large_fern"
            | "minecraft:vine"
            | "minecraft:snow"
            | "minecraft:fire"
    ) || block.ends_with("_sapling")
        || block.ends_with("_mushroom")
}

/// Finds the nearest safe position for grave placement upwards from the death origin.
pub fn find_safe_grave_location(
    origin: [f64; 3],
    mut is_replaceable: impl FnMut([i32; 3]) -> bool,
) -> Option<[i32; 3]> {
    let x = origin[0].floor() as i32;
    let mut y = origin[1].floor() as i32;
    let z = origin[2].floor() as i32;

    // Void fallback: if player fell into void, clamp to safe bottom layer (min + 1)
    if y <= MIN_BUILD_HEIGHT {
        y = MIN_BUILD_HEIGHT + 1;
    }

    // Traverse upwards until a replaceable block is found
    while y < MAX_BUILD_HEIGHT {
        let pos = [x, y, z];
        if is_replaceable(pos) {
            return Some(pos);
        }
        y += 1;
    }

    None
}

/// Computes the horizontal facing direction for the gravestone from the player's yaw.
/// The grave faces towards the player, opposite to the player's look direction.
pub fn yaw_to_grave_facing(yaw: f32) -> &'static str {
    // Normalize yaw to 0..360
    let mut deg = yaw % 360.0;
    if deg < 0.0 {
        deg += 360.0;
    }

    // Standard Minecraft yaw:
    // 0 / 360 = South (+Z)
    // 90 = West (-X)
    // 180 = North (-Z)
    // 270 = East (+X)
    // Grave faces towards player (opposite to look direction):
    if (45.0..135.0).contains(&deg) {
        "east"
    } else if (135.0..225.0).contains(&deg) {
        "south"
    } else if (225.0..315.0).contains(&deg) {
        "west"
    } else {
        "north"
    }
}

/// Restores items into player's inventory:
/// - Fits items into their original slot if empty;
/// - If slot is occupied, pushes item to overflow;
/// - Returns list of overflow items that must be placed in any free slot or dropped.
pub fn sort_restore_inventory(
    player_slots: &mut [Option<GraveItem>],
    grave_items: &[GraveItem],
) -> Vec<GraveItem> {
    let mut overflow = Vec::new();

    for item in grave_items {
        if item.slot < player_slots.len() && player_slots[item.slot].is_none() {
            player_slots[item.slot] = Some(item.clone());
        } else {
            overflow.push(item.clone());
        }
    }

    overflow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replaceable() {
        assert!(is_replaceable_block("minecraft:air"));
        assert!(is_replaceable_block("minecraft:water"));
        assert!(is_replaceable_block("minecraft:lava"));
        assert!(is_replaceable_block("minecraft:short_grass"));
        assert!(is_replaceable_block("minecraft:oak_sapling"));
        assert!(!is_replaceable_block("minecraft:stone"));
        assert!(!is_replaceable_block("minecraft:dirt"));
        assert!(!is_replaceable_block("gravestone:gravestone"));
    }

    #[test]
    fn test_void_clamp() {
        let origin = [10.5, -70.0, 20.5];
        let loc = find_safe_grave_location(origin, |pos| pos[1] >= -63);
        assert_eq!(loc, Some([10, -63, 20]));
    }

    #[test]
    fn test_facing() {
        assert_eq!(yaw_to_grave_facing(0.0), "north");
        assert_eq!(yaw_to_grave_facing(90.0), "east");
        assert_eq!(yaw_to_grave_facing(180.0), "south");
        assert_eq!(yaw_to_grave_facing(270.0), "west");
    }

    #[test]
    fn test_inventory_restore() {
        let mut slots = vec![None; 36];
        slots[0] = Some(GraveItem {
            slot: 0,
            item: "minecraft:apple".into(),
            count: 1,
            damage: 0,
        });

        let grave_items = vec![
            GraveItem {
                slot: 0,
                item: "minecraft:diamond_sword".into(),
                count: 1,
                damage: 0,
            },
            GraveItem {
                slot: 1,
                item: "minecraft:iron_pickaxe".into(),
                count: 1,
                damage: 0,
            },
        ];

        let overflow = sort_restore_inventory(&mut slots, &grave_items);
        assert_eq!(slots[1].as_ref().unwrap().item, "minecraft:iron_pickaxe");
        assert_eq!(overflow.len(), 1);
        assert_eq!(overflow[0].item, "minecraft:diamond_sword");
    }
}

