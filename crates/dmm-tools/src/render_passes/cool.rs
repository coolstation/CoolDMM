/*******************************************************************************
* Render Passes for Coolstation
* (may be compatible with other Gooncode based servers)
*
* SpacingNevada 2026
*
* If they're not working for you give me a ping on the Coolstation Discord
* and I'll see what I can do for ya
*******************************************************************************/
use dm::dmi::Dir;

use super::*;

use rand::seq::SliceRandom;

// Helpers for connecting sprites
const N_NORTH: usize = 1;
const N_SOUTH: usize = 2;
const N_EAST: usize = 4;
const N_WEST: usize = 8;

const N_NORTHWEST: usize = 16;
const N_NORTHEAST: usize = 32;
const N_SOUTHWEST: usize = 64;
const N_SOUTHEAST: usize = 128;

const CONNECTS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
];

const R_CONNECTS: &[&str] = &[
    "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14",
    "R15",
];

const G_CONNECTS: &[&str] = &[
    "g0", "g1", "g2", "g3", "g4", "g5", "g6", "g7", "g8", "g9", "g10", "g11", "g12", "g13", "g14",
    "g15",
];

//const D_CONNECTS: &[&str] = &["", "NW", "NE", "SW", "SE"];

// Planes
//const PLANE_UNDERFLOOR: i32 = -120; Only used by Cluwnes
const PLANE_SPACE: i32 = -115;
const PLANE_FLOOR: i32 = -110;
const PLANE_WALL: i32 = -105;
const PLANE_NOSHADOW_BELOW: i32 = -101;
const PLANE_DEFAULT: i32 = -100;
const PLANE_NOSHADOW_ABOVE: i32 = -99;
//const PLANE_EXAMINE: i32 = -96;
//const PLANE_HIDDENGAME: i32 = -95;
//const PLANE_LIGHTING: i32 = -90;
//const PLANE_SELFILLUM: i32 = -80;
//const PLANE_BLACKNESS: i32 = 0;
//const PLANE_MASTER_GAME: i32 = 10;
//const PLANE_FLOCKVISION: i32 = 22;
//const PLANE_OVERLAY_EFFECTS: i32 = 25;
//const PLANE_HUD: i32 = 30;
//const PLANE_SCREEN_OVERLAYS: i32 = 40;

// Layers
const TURF_LAYER: f32 = 2.0;
const PLATING_LAYER: f32 = TURF_LAYER - 0.2;
//const BETWEEN_FLOORS_LAYER: f32 = TURF_LAYER - 0.1; Only used by Cluwnes
//const TURF_OVERLAY_LAYER: f32 = TURF_LAYER + 0.05;
const LATTICE_LAYER: f32 = TURF_LAYER + 0.10;
const DISPOSAL_PIPE_LAYER: f32 = TURF_LAYER + 0.11;
const PIPE_LAYER: f32 = TURF_LAYER + 0.12;
const CATWALK_LAYER: f32 = TURF_LAYER + 0.13;
const CABLE_LAYER: f32 = TURF_LAYER + 0.14;
const CATWALK_OVERPIPE: f32 = TURF_LAYER + 0.15;
//const CLEANABLE_DECAL_LAYER: f32 = TURF_LAYER + 0.155;
const PIPE_MACHINE_LAYER: f32 = TURF_LAYER + 0.16;
const PIPE_OVERCAT: f32 = TURF_LAYER + 0.17;
const DECAL_LAYER: f32 = TURF_LAYER + 0.2;
//const FLUID_LAYER: f32 = TURF_LAYER + 0.25;
//const FLUID_AIR_LAYER: f32 = TURF_LAYER + 0.26;
//const FIREFLASH_LAYER: f32 = TURF_LAYER + 0.27;
const FLOOR_EQUIP_LAYER1: f32 = TURF_LAYER + 0.3;
//const FLOOR_EQUIP_LAYER2: f32 = TURF_LAYER + 0.31;
//const AI_RAIL_LAYER: f32 = TURF_LAYER + 0.4;
//const TURF_EFFECTS_LAYER: f32 = TURF_LAYER + 0.8;
//const GRILLE_LAYER: f32 = TURF_LAYER + 0.9;
//const COG2_WINDOW_LAYER: f32 = TURF_LAYER + 0.95;

const EFFECTS_LAYER_BASE: f32 = 30.0;

/// Turns wingrille spawners into windows and grilles and sets their colour.
#[derive(Default)]
pub struct WinGrilles;
impl RenderPass for WinGrilles {
    fn expand<'a>(
        &self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        output: &mut Vec<Atom<'a>>,
    ) -> bool {
        if !atom.istype("/obj/wingrille_spawn/") {
            return true;
        };
        let win_path = atom.get_var("win_path", objtree).as_str().unwrap();
        let grille_path = atom.get_var("grille_path", objtree).as_str().unwrap();
        let window = Atom::from(objtree.expect(win_path));
        let grille = Atom::from(objtree.expect(grille_path));

        output.push(window);
        output.push(grille);
        false
    }

    fn adjust_sprite<'a>(
        &self,
        atom: &Atom<'a>,
        sprite: &mut Sprite<'a>,
        _objtree: &'a ObjectTree,
        _bump: &'a bumpalo::Bump,
    ) {
        if atom.istype("/obj/grille/steel") {
            sprite.color = [140, 140, 140, 255]
        }

        if atom.istype("/obj/window/crystal") {
            sprite.color = [161, 20, 255, 180]
        } else if atom.istype("/obj/window/") {
            sprite.color = [163, 220, 255, 180]
        }
    }
}

/// Hides all the holiday decorations.
#[derive(Default)]
pub struct HideHolidays;
impl RenderPass for HideHolidays {
    fn early_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        if atom.istype("/obj/decal/garland")
            || atom.istype("/obj/decal/tinsel")
            || atom.istype("/obj/decal/wreath")
            || atom.istype("/obj/decal/mistletoe")
            || atom.istype("/obj/decal/xmas_lights")
            || atom.istype("/obj/decal/cleanable/grinch_graffiti")
            || atom.istype("/obj/stocking")
        {
            return false;
        }
        true
    }
}

/// Hides disposal pipes.
#[derive(Default)]
pub struct HidePipes;
impl RenderPass for HidePipes {
    fn early_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        !atom.istype("/obj/disposalpipe/")
    }
}

// Hides power cables.
#[derive(Default)]
pub struct HideWires;
impl RenderPass for HideWires {
    fn early_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        !atom.istype("/obj/cable/")
    }
}

/// Converts random item spawners into a random selection from their item lists.
#[derive(Default)]
pub struct CoolRandom;
impl RenderPass for CoolRandom {
    fn expand<'a>(
        &self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        output: &mut Vec<Atom<'a>>,
    ) -> bool {
        if atom.istype("/obj/random_item_spawner/") {
            let mut rng = rand::thread_rng();
            let items: &mut Vec<Atom<'a>> = &mut Vec::new();
            match atom.get_var("items2spawn", objtree) {
                Constant::List(elements) => {
                    for (key, _) in elements.iter() {
                        let type_key;
                        let reference = match *key {
                            Constant::String(ref s) => s,
                            Constant::Prefab(ref fab) => {
                                type_key = dm::ast::FormatTreePath(&fab.path).to_string();
                                type_key.as_str()
                            },
                            _ => continue,
                        };
                        items.push(Atom::from(objtree.expect(reference)));
                    }
                    if !items.is_empty() {
                        let rand_item = items.choose(&mut rng).unwrap();
                        output.push(rand_item.to_owned());
                    }
                    false
                },
                _ => true,
            }
        } else {
            true
        }
    }

    fn adjust_sprite<'a>(
        &self,
        atom: &Atom<'a>,
        sprite: &mut Sprite<'a>,
        objtree: &'a ObjectTree,
        _bump: &'a bumpalo::Bump,
    ) {
        let mut rng = rand::thread_rng();
        let states = &mut Vec::new();
        if let Constant::List(elements) = atom.get_var("random_icon_states", objtree) {
            for (key, _) in elements.iter() {
                let reference = match *key {
                    Constant::String(ref s) => s.as_str(),
                    _ => continue,
                };
                states.push(reference);
            }
            if !states.is_empty() {
                let rand_state = *states.choose(&mut rng).unwrap();
                //eprintln!("{:#?}", rand_state);
                sprite.icon_state = rand_state;
            }
        }
    }

    fn overlays<'a>(
        &self,
        atom: &Atom<'a>,
        _objtree: &'a ObjectTree,
        _underlays: &mut Vec<Sprite<'a>>,
        overlays: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump, // TODO: kind of a hacky way to pass this
    ) {
        if atom.istype("/obj/landmark/random_room") {
            if atom.istype("/obj/landmark/random_room/size5x3") {
                overlays.push(Sprite {
                    icon: "icons/map-editing/random-rooms/5x3.dmi",
                    icon_state: "",
                    ..Default::default()
                })
            } else if atom.istype("/obj/landmark/random_room/size3x5") {
                overlays.push(Sprite {
                    icon: "icons/map-editing/random-rooms/3x5.dmi",
                    icon_state: "",
                    ..Default::default()
                })
            } else if atom.istype("/obj/landmark/random_room/size3x3") {
                overlays.push(Sprite {
                    icon: "icons/map-editing/random-rooms/3x3.dmi",
                    icon_state: "",
                    ..Default::default()
                })
            } else if atom.istype("/obj/landmark/random_room/size5x4") {
                overlays.push(Sprite {
                    icon: "icons/map-editing/random-rooms/5x4.dmi",
                    icon_state: "",
                    ..Default::default()
                })
            }
        }
    }

    fn late_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        !atom.istype("/obj/random_item_spawner/")
    }
}

/// Catch-all for any overlays.
#[derive(Default)]
pub struct CoolOverlays;
impl RenderPass for CoolOverlays {
    fn neighborhood_appearance<'a>(
        &self,
        atom: &Atom<'a>,
        _objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        output: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump, // TODO: kind of a hacky way to pass this
    ) -> bool {
        if atom.istype("/obj/machinery/phone") {
            let mut phone = atom.sprite.clone();
            for inner_atom in neighborhood.center() {
                if inner_atom.istype("/area/station/security") {
                    phone.color = parse_hex("A76A6A").unwrap()
                } else if inner_atom.istype("/area/station/bridge") {
                    phone.color = parse_hex("769C76").unwrap()
                } else if inner_atom.istype("/area/station/engine")
                    || inner_atom.istype("/area/station/quartermaster")
                    || inner_atom.istype("/area/station/mining")
                {
                    phone.color = parse_hex("AA9867").unwrap()
                } else if inner_atom.istype("/area/station/science") {
                    phone.color = parse_hex("9A84B1").unwrap()
                } else if inner_atom.istype("/area/station/medical") {
                    phone.color = parse_hex("6395B8").unwrap()
                }
            }
            output.push(phone);
            false
        } else {
            true
        }
    }

    fn adjust_sprite<'a>(
        &self,
        atom: &Atom<'a>,
        sprite: &mut Sprite<'a>,
        objtree: &'a ObjectTree,
        _bump: &'a bumpalo::Bump,
    ) {
        use crate::dmi::Dir;

        if atom.istype("/obj/machinery/power/apc/") {
            if atom.istype("/obj/machinery/power/apc/autoname/") {
                let y_offset = atom.get_var("pixel_y", objtree).to_int();
                let x_offset = atom.get_var("pixel_x", objtree).to_int();
                // auto-set pixel location
                match atom
                    .get_var("dir", objtree)
                    .to_int()
                    .and_then(Dir::from_int)
                {
                    Some(Dir::North) => sprite.ofs_y = y_offset.unwrap(),
                    Some(Dir::South) => sprite.ofs_y = y_offset.unwrap(),
                    Some(Dir::East) => sprite.ofs_x = x_offset.unwrap(),
                    Some(Dir::West) => sprite.ofs_x = x_offset.unwrap(),
                    _ => {},
                }
            } else {
                let y_offset = atom.get_var_inner("pixel_y", objtree);
                let x_offset = atom.get_var_inner("pixel_x", objtree);

                if let Some(offset) = y_offset {
                    sprite.ofs_y = offset.to_int().unwrap()
                };
                if let Some(offset) = x_offset {
                    sprite.ofs_x = offset.to_int().unwrap()
                };
                sprite.icon_state = "apc0";
            }
        } else if atom.istype("/obj/item/device/radio/intercom/") {
            if sprite.icon_state == "intercom-map" {
                sprite.icon_state = "intercom"
            }
            // auto-set pixel location
            match sprite.dir {
                Dir::North => sprite.ofs_y = -21,
                Dir::South => sprite.ofs_y = 24,
                Dir::East => sprite.ofs_x = -21,
                Dir::West => sprite.ofs_x = 21,
                _ => {},
            }
        } else if sprite.icon_state == "celltop-P" {
            sprite.icon_state = "cryo_bottom_1";
        } else if sprite.icon_state == "turret" {
            sprite.icon_state = "turretCover";
        } else if sprite.icon_state == "f_spawn" {
            sprite.icon_state = "door_open";
        } else if sprite.icon_state == "coolerbase" {
            sprite.icon_state = "water_fountain1"
        }
    }

    fn overlays<'a>(
        &self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        underlays: &mut Vec<Sprite<'a>>,
        overlays: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump,
    ) {
        // APC lights
        if atom.istype("/obj/machinery/power/apc/") {
            // status overlays
            for &each in ["apcox-1", "apco3-2", "apco0-3", "apco1-3", "apco2-3"].iter() {
                add_to(overlays, atom, each);
            }

            // A terminal for every APC
            let mut terminal =
                Sprite::from_vars(objtree, &objtree.expect("/obj/machinery/power/terminal"));
            terminal.dir = atom.sprite.dir;
            terminal.layer = Layer::from(FLOOR_EQUIP_LAYER1);
            terminal.plane = PLANE_NOSHADOW_BELOW;
            underlays.push(terminal);

        // Cryo tube-tops
        } else if atom.istype("/obj/machinery/atmospherics/unary/cryo_cell") {
            let mut rng = rand::thread_rng();
            let cryo_top = Sprite {
                icon_state: ["celltop", "celltop_1"].choose(&mut rng).unwrap(),
                ofs_y: 32,
                ..atom.sprite
            };
            overlays.push(cryo_top);

        // Intercom color overlays
        } else if atom.istype("/obj/item/device/radio/intercom/") {
            let screen = Sprite {
                icon_state: "intercom-screen",
                color: parse_hex(
                    atom.get_var("device_color", objtree)
                        .as_str()
                        .unwrap_or_default()
                        .trim_matches('#'),
                )
                .unwrap_or([0, 0, 0, 0]),
                ..atom.sprite
            };
            overlays.push(screen);

        // Storage lock lights
        } else if atom.istype("/obj/storage/secure/") {
            if atom.istype("/obj/storage/secure/closet/fridge") {
                add_to(overlays, atom, "fridge-redlight")
            } else if atom.istype("/obj/storage/secure/closet/") {
                add_to(overlays, atom, "redlight")
            } else if atom.istype("/obj/storage/secure/crate/") {
                add_to(overlays, atom, "securecrater")
            }

        // Cabinet Slots
        } else if atom.istype("/obj/cabinet/") {
            let slots = Vec::from([(3, 5), (17, 5), (3, 16), (17, 16), (3, 27), (17, 27)]);
            let mut rng = rand::thread_rng();
            for slot in slots {
                overlays.push(Sprite {
                    icon_state: ["slot_empty", "slot_bottle"].choose(&mut rng).unwrap(),
                    ofs_x: slot.0,
                    ofs_y: slot.1,
                    ..atom.sprite
                });
            }
        } else if atom.istype("/obj/machinery/sleeper") {
            overlays.push(Sprite {
                icon_state: "sleeperlid0",
                ..atom.sprite
            })
        } else if atom.istype("/obj/machinery/portable_atmospherics/canister/") {
            overlays.push(Sprite {
                icon_state: "can-o3",
                ..atom.sprite
            })
        }
    }
}

#[derive(Default)]
pub struct CoolLayers;
impl RenderPass for CoolLayers {
    fn adjust_sprite<'a>(
        &self,
        atom: &Atom<'a>,
        sprite: &mut Sprite<'a>,
        _objtree: &'a ObjectTree,
        _bump: &'a bumpalo::Bump,
    ) {
        self.apply_cool_plane(atom.get_path(), sprite);
        self.apply_cool_layer(atom.get_path(), sprite);
    }
}

/// Applies planes and layers to sprites so they're drawn in the right order.
impl CoolLayers {
    fn cool_layer_for_path(&self, p: &str) -> Option<Layer> {
        if ispath(p, "/turf/floor/plating/") {
            Some(Layer::from(PLATING_LAYER))
        } else if ispath(p, "/turf/") {
            Some(Layer::from(TURF_LAYER))
        } else if ispath(p, "/obj/effect/turf_decal/") {
            Some(Layer::from(DECAL_LAYER))
        } else if ispath(p, "/obj/disposalpipe/") {
            Some(Layer::from(DISPOSAL_PIPE_LAYER))
        } else if ispath(p, "/obj/machinery/atmospherics/pipe/overfloor/")
            || ispath(p, "/obj/machinery/atmospherics/pipe/tank/")
        {
            Some(Layer::from(PIPE_OVERCAT))
        } else if ispath(p, "/obj/machinery/power/terminal/")
            || ispath(p, "/obj/machinery/power/data_terminal/")
        {
            Some(Layer::from(FLOOR_EQUIP_LAYER1))
        } else if ispath(p, "/obj/machinery/atmospherics/binary/pump") {
            Some(Layer::from(PIPE_MACHINE_LAYER))
        } else if ispath(p, "/obj/machinery/atmospherics/unary/cryo_cell") {
            Some(Layer::from(EFFECTS_LAYER_BASE))
        } else if ispath(p, "/obj/machinery/atmospherics/") {
            Some(Layer::from(PIPE_LAYER))
        } else if ispath(p, "/obj/cable/") || ispath(p, "/obj/machinery/navbeacon") {
            Some(Layer::from(CABLE_LAYER))
        } else if ispath(p, "/obj/structure/lattice/") {
            Some(Layer::from(LATTICE_LAYER))
        } else if ispath(p, "/obj/grille/catwalk/jen") || ispath(p, "/obj/grille/catwalk/bob") {
            Some(Layer::from(CATWALK_OVERPIPE))
        } else if ispath(p, "/obj/grille/catwalk") {
            Some(Layer::from(CATWALK_LAYER))
        } else if ispath(p, "/obj/decal/tile_edge") {
            Some(Layer::from(TURF_LAYER + 0.1))
        } else {
            None
        }
    }

    fn cool_plane_for_path(&self, p: &str) -> Option<i32> {
        if ispath(p, "/turf/wall") || ispath(p, "/obj/indestructible/shuttle_corner") {
            Some(PLANE_WALL)
        } else if ispath(p, "/turf/floor") || ispath(p, "/turf/") {
            Some(PLANE_FLOOR)
        } else if ispath(p, "/obj/effects/background_objects") {
            Some(PLANE_SPACE)
        } else if ispath(p, "/obj/item/device/radio/intercom")
            || ispath(p, "/obj/item/storage/wall")
            || ispath(p, "/obj/shitty_radio/ceiling")
            || ispath(p, "/obj/submachine/ATM")
            || ispath(p, "/obj/machinery/drainage/stand_pipe")
            || ispath(p, "/obj/item/device/radio/intercom/")
        {
            Some(PLANE_NOSHADOW_ABOVE)
        } else if ispath(p, "/obj/cable/")
            || ispath(p, "/obj/machinery/atmospherics/pipe/")
            || ispath(p, "/obj/machinery/atmospherics/binary/dp_vent_pump")
            || ispath(p, "/obj/machinery/atmospherics/binary/pump")
            || ispath(p, "/obj/machinery/power/terminal/")
            || ispath(p, "/obj/machinery/power/data_terminal/")
            || ispath(p, "/obj/decal/aliencomputer")
            || ispath(p, "/obj/crevice")
            || ispath(p, "/obj/grille/catwalk/bob")
            || ispath(p, "/obj/grille/catwalk/jen")
            || ispath(p, "/obj/decal/snowbits")
            || ispath(p, "/obj/river")
            || ispath(p, "/obj/decoration/bullethole")
            || ispath(p, "/obj/decoration/plasmabullethole")
            || ispath(p, "/obj/securearea")
        {
            Some(PLANE_NOSHADOW_BELOW)
        } else if ispath(p, "/obj/machinery/drainage/")
            || ispath(p, "/obj/grille/catwalk")
            || ispath(p, "/obj/disposalpipe/")
            || ispath(p, "/obj/lattice")
            || ispath(p, "/obj/decal/tile_edge")
        {
            Some(PLANE_FLOOR)
        } else {
            Some(PLANE_DEFAULT)
        }
    }

    fn apply_cool_layer(&self, path: &str, sprite: &mut Sprite) {
        if let Some(layer) = self.cool_layer_for_path(path) {
            sprite.layer = layer;
        }
    }

    fn apply_cool_plane(&self, path: &str, sprite: &mut Sprite) {
        if let Some(plane) = self.cool_plane_for_path(path) {
            sprite.plane = plane;
        }
    }
}

/// Hide ocean surface sprites.
#[derive(Default)]
pub struct HideOcean;
impl RenderPass for HideOcean {
    fn late_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        !atom.istype("/turf/space/magindara")
    }
}

/// Don't show pipes if they're under floor tiles.
#[derive(Default)]
pub struct OccludePipes;
impl RenderPass for OccludePipes {
    fn neighborhood_appearance<'a>(
        &self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        output: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump,
    ) -> bool {
        if atom.istype("/obj/disposalpipe") {
            let mut under_tile = false;
            for atom in neighborhood.center() {
                if atom.get_var("intact", objtree).to_bool() {
                    under_tile = true;
                }
            }
            !under_tile
        } else if !atom.istype("/obj/machinery/atmospherics/pipe/vent/")
            && !atom.istype("/obj/machinery/atmospherics/pipe/vertical_pipe/")
            && atom.istype("/obj/machinery/atmospherics/pipe/")
        {
            let mut under_tile = false;
            for inner_atom in neighborhood.center() {
                if inner_atom.get_var("intact", objtree).to_bool()
                    && atom.get_var("level", objtree).to_int().unwrap_or(2) == 1
                {
                    under_tile = true;
                }
            }
            !under_tile
        } else if atom.istype("/obj/machinery/atmospherics/unary/vent_pump")
            || atom.istype("/obj/machinery/atmospherics/unary/vent_scrubber")
        {
            let mut under_tile = false;
            for inner_atom in neighborhood.center() {
                if inner_atom.istype("/obj/grille/catwalk")
                    || inner_atom.get_var("intact", objtree).to_bool()
                        && atom.get_var("level", objtree).to_int().unwrap_or(2) == 1
                {
                    under_tile = true;
                    if atom.sprite.icon_state == "out" {
                        output.push(Sprite {
                            icon_state: "hout",
                            ..atom.sprite
                        });
                    } else if atom.sprite.icon_state == "in" {
                        output.push(Sprite {
                            icon_state: "hin",
                            ..atom.sprite
                        });
                    } else if atom.sprite.icon_state == "off" {
                        output.push(Sprite {
                            icon_state: "hoff",
                            ..atom.sprite
                        });
                    } else if atom.sprite.icon_state == "on" {
                        output.push(Sprite {
                            icon_state: "hon",
                            ..atom.sprite
                        });
                    }
                }
            }
            !under_tile
        } else {
            true
        }
    }
}

/// Don't show power cables if they're under floor tiles.
#[derive(Default)]
pub struct OccludeWires;
impl RenderPass for OccludeWires {
    fn neighborhood_appearance<'a>(
        &self,
        atom: &Atom<'a>,
        objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        _output: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump,
    ) -> bool {
        if atom.istype("/obj/cable")
            || atom.istype("/obj/machinery/navbeacon")
            || atom.istype("/obj/machinery/power/data_terminal")
        {
            let mut under_tile = false;
            for atom in neighborhood.center() {
                if atom.get_var("intact", objtree).to_bool() {
                    under_tile = true;
                }
            }
            !under_tile
        } else {
            true
        }
    }
}

/// Hides objects listed in the config toml or in icons/map-editing/mark.dmi
#[derive(Default)]
pub struct CoolInvisible {
    overrides: Vec<String>,
}

impl RenderPass for CoolInvisible {
    fn configure(&mut self, renderer_config: &dm::config::MapRenderer) {
        self.overrides.clone_from(&renderer_config.hide_invisible);
        // Put longer typepaths earlier in the list so that `/foo/bar` can override `/foo`.
        self.overrides
            .sort_unstable_by_key(|k| usize::MAX - k.len());
        // Append `/` to each typepath for faster starts_with later.
        for key in self.overrides.iter_mut() {
            if !key.ends_with('/') {
                key.push('/');
            }
        }
    }

    fn early_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        // Remove it if it is in our list of atoms to hide
        for pathtype in self.overrides.iter() {
            // Note: You *cannot* just `return !atom.istype(pathtype)`
            // If you do that, you skip the rest of the loop iterations
            if atom.istype(pathtype) && !atom.istype("/obj/landmark/random_room") {
                return false;
            }
        }
        true
    }

    fn sprite_filter(&self, sprite: &Sprite) -> bool {
        sprite.icon != "icons/map-editing/mark.dmi"
    }
}

/// Renders only power cables of any kind.
#[derive(Default)]
pub struct CoolWires;
impl RenderPass for CoolWires {
    fn late_filter(&self, atom: &Atom, _: &ObjectTree) -> bool {
        atom.istype("/obj/cable/")
    }
}

/// Renders only sewage disposal pipes.
#[derive(Default)]
pub struct CoolSewage;
impl RenderPass for CoolSewage {
    fn early_filter(&self, atom: &Atom, _: &ObjectTree) -> bool {
        atom.istype("/obj/disposalpipe/segment/sewage")
            || atom.istype("/obj/disposalpipe/trunk/sewage")
        //|| atom.istype("/obj/disposalpipe/segment/bent")
    }
}

/// Renders only disposal pipes of any kind.
#[derive(Default)]
pub struct CoolPipes;
impl RenderPass for CoolPipes {
    fn early_filter(&self, atom: &Atom, _: &ObjectTree) -> bool {
        atom.istype("/obj/disposalpipe")
    }
}

/// Renders only power cables and disposal pipes of any kind.
#[derive(Default)]
pub struct CoolWiresAndPipes;
impl RenderPass for CoolWiresAndPipes {
    fn early_filter(&self, atom: &Atom, _: &ObjectTree) -> bool {
        atom.istype("/obj/disposalpipe") || atom.istype("/obj/cable/")
    }
}

/// Renders connecting sprites correctly.
#[derive(Default)]
pub struct CoolConnects;
impl RenderPass for CoolConnects {
    fn neighborhood_appearance<'a>(
        &self,
        atom: &Atom<'a>,
        _objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        output: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump,
    ) -> bool {
        if atom.istype("/obj/window/") {
            let mut linked_dirs: usize = 0;
            for item in neighborhood.center() {
                if item.istype("/obj/grille/steel") {
                    for &check_dir in Dir::CARDINALS {
                        let turf = neighborhood.offset(check_dir);
                        for inner_atom in turf {
                            if inner_atom.istype("/obj/grille/steel") {
                                match check_dir {
                                    Dir::North => linked_dirs |= N_NORTH,
                                    Dir::South => linked_dirs |= N_SOUTH,
                                    Dir::East => linked_dirs |= N_EAST,
                                    Dir::West => linked_dirs |= N_WEST,
                                    _ => linked_dirs += 0,
                                }
                            }
                        }
                    }
                    let mut state = CONNECTS[0];
                    if atom.istype("/obj/window/reinforced")
                        || atom.istype("/obj/window/crystal/reinforced")
                    {
                        state = R_CONNECTS[linked_dirs];
                    } else if atom.istype("/obj/window") {
                        state = CONNECTS[linked_dirs];
                    }
                    let sprite = Sprite {
                        icon_state: state,
                        ..atom.sprite
                    };
                    output.push(sprite);
                    return false;
                } else {
                    continue;
                }
            }
            true
        } else if atom.istype("/obj/table/")
            && !atom.istype("/obj/table/folding")
            && !atom.istype("/obj/table/glass")
            && !atom.istype("/obj/table/surgery_tray")
        {
            let mut linked_dirs: usize = 0;
            for &check_dir in Dir::ALL {
                let turf = neighborhood.offset(check_dir);
                for inner_atom in turf {
                    if inner_atom.istype("/obj/table/")
                        && !atom.istype("/obj/table/folding")
                        && !atom.istype("/obj/table/glass")
                        && !atom.istype("/obj/table/surgery_tray")
                    {
                        match check_dir {
                            Dir::North => linked_dirs |= N_NORTH,
                            Dir::South => linked_dirs |= N_SOUTH,
                            Dir::East => linked_dirs |= N_EAST,
                            Dir::West => linked_dirs |= N_WEST,
                            Dir::Northwest => linked_dirs |= N_NORTHWEST,
                            Dir::Northeast => linked_dirs |= N_NORTHEAST,
                            Dir::Southwest => linked_dirs |= N_SOUTHWEST,
                            Dir::Southeast => linked_dirs |= N_SOUTHEAST,
                        }
                    }
                }
            }
            let state = CONNECTS[linked_dirs & 15];
            let sprite = Sprite {
                icon_state: state,
                ..atom.sprite
            };
            output.push(sprite);
            if linked_dirs & N_NORTHWEST != 0
                && linked_dirs & N_NORTH != 0
                && linked_dirs & N_WEST != 0
            {
                add_to(output, atom, "NW");
            }
            if linked_dirs & N_NORTHEAST != 0
                && linked_dirs & N_NORTH != 0
                && linked_dirs & N_EAST != 0
            {
                add_to(output, atom, "NE");
            }
            if linked_dirs & N_SOUTHWEST != 0
                && linked_dirs & N_SOUTH != 0
                && linked_dirs & N_WEST != 0
            {
                add_to(output, atom, "SW");
            }
            if linked_dirs & N_SOUTHEAST != 0
                && linked_dirs & N_SOUTH != 0
                && linked_dirs & N_EAST != 0
            {
                add_to(output, atom, "SE");
            }
            false
        } else if atom.istype("/obj/table/glass") {
            let mut linked_dirs: usize = 0;
            for &check_dir in Dir::ALL {
                let turf = neighborhood.offset(check_dir);
                for inner_atom in turf {
                    if inner_atom.istype("/obj/table/glass") {
                        match check_dir {
                            Dir::North => linked_dirs |= N_NORTH,
                            Dir::South => linked_dirs |= N_SOUTH,
                            Dir::East => linked_dirs |= N_EAST,
                            Dir::West => linked_dirs |= N_WEST,
                            Dir::Northwest => linked_dirs |= N_NORTHWEST,
                            Dir::Northeast => linked_dirs |= N_NORTHEAST,
                            Dir::Southwest => linked_dirs |= N_SOUTHWEST,
                            Dir::Southeast => linked_dirs |= N_SOUTHEAST,
                        }
                    }
                }
            }
            let state = CONNECTS[linked_dirs & 15];
            let sprite = Sprite {
                icon_state: state,
                ..atom.sprite
            };
            output.push(sprite);
            add_to(output, atom, G_CONNECTS[linked_dirs & 15]);
            if linked_dirs & N_NORTHWEST != 0 {
                if linked_dirs & N_NORTH != 0 && linked_dirs & N_WEST != 0 {
                    add_to(output, atom, "gNWs");
                } else {
                    add_to(output, atom, "gNW");
                }
            }
            if linked_dirs & N_NORTHEAST != 0 {
                if linked_dirs & N_NORTH != 0 && linked_dirs & N_EAST != 0 {
                    add_to(output, atom, "gNEs");
                } else {
                    add_to(output, atom, "gNE");
                }
            }
            if linked_dirs & N_SOUTHWEST != 0 {
                if linked_dirs & N_SOUTH != 0 && linked_dirs & N_WEST != 0 {
                    add_to(output, atom, "gSWs");
                } else {
                    add_to(output, atom, "gSW");
                }
            }
            if linked_dirs & N_SOUTHEAST != 0 {
                if linked_dirs & N_SOUTH != 0 && linked_dirs & N_EAST != 0 {
                    add_to(output, atom, "gSEs");
                } else {
                    add_to(output, atom, "gSE");
                }
            }
            false
        } else {
            true
        }
    }
}

/// Hide items on the same tile as a container.
#[derive(Default)]
pub struct CoolContainers;
impl RenderPass for CoolContainers {
    fn neighborhood_appearance<'a>(
        &self,
        atom: &Atom<'a>,
        _objtree: &'a ObjectTree,
        neighborhood: &Neighborhood<'a, '_>,
        _output: &mut Vec<Sprite<'a>>,
        _bump: &'a bumpalo::Bump, // TODO: kind of a hacky way to pass this
    ) -> bool {
        if atom.istype("/obj/item/") {
            let mut show = true;
            for atom in neighborhood.center() {
                if atom.istype("/obj/storage/") {
                    show = false;
                }
            }
            show
        } else {
            true
        }
    }
}

/// Late filter for area sprites. Allows access to area data in earlier passes.
#[derive(Default)]
pub struct CoolHideAreas;
impl RenderPass for CoolHideAreas {
    fn late_filter(&self, atom: &Atom, _objtree: &ObjectTree) -> bool {
        !atom.istype("/area/")
    }
}

/// Parses hex color codes into RGBA arrays
/// Yoinked directly from dm-langserver/src/color.rs
fn parse_hex(hex: &str) -> Option<[u8; 4]> {
    let mut sum = 0;
    for ch in hex.chars() {
        sum = 16 * sum + ch.to_digit(16).unwrap_or(0);
    }

    if hex.len() == 8 {
        // #rrggbbaa
        Some([
            (sum >> 24) as u8,
            (sum >> 16) as u8,
            (sum >> 8) as u8,
            sum as u8,
        ])
    } else if hex.len() == 6 {
        // #rrggbb
        Some([(sum >> 16) as u8, (sum >> 8) as u8, sum as u8, 255])
    } else if hex.len() == 4 {
        // #rgba
        Some([
            (0x11 * ((sum >> 12) & 0xf)) as u8,
            (0x11 * ((sum >> 8) & 0xf)) as u8,
            (0x11 * ((sum >> 4) & 0xf)) as u8,
            (0x11 * (sum & 0xf)) as u8,
        ])
    } else if hex.len() == 3 {
        // #rgb
        Some([
            (0x11 * ((sum >> 8) & 0xf)) as u8,
            (0x11 * ((sum >> 4) & 0xf)) as u8,
            (0x11 * (sum & 0xf)) as u8,
            255,
        ])
    } else {
        None
    }
}
