use std::cell::RefCell;

use wgpu::TextureView;

use crate::rendering::{sprites::load_level_sprites::helper_functions::{load_sprite_from_memory, load_sprite_from_memory_uncropped}, wgpurenderer::Renderer};

pub(crate) fn load_initial_level_sprites(renderer: &Renderer) ->  Vec<TextureView>{
    println!("Loading initial level sprites");

    //consider moving these to a file to specify which textures to load and where to load them from, based on the level selected
    let diffuse_bytes = include_bytes!("../../../../textures/Dwarf_BaseHouse.png");
    let dwarf_base_house_texture_view = load_sprite_from_memory(&diffuse_bytes.as_slice(), 135,45, 380, 517,  renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/Dwarf_BaseHouse_px9.png");
    let dwarf_base_house_texture_view2 = load_sprite_from_memory(&diffuse_bytes.as_slice(), 135,45, 380, 517, renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/black_pixel.png");
    let black_pixel_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes.as_slice(), renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/Solid_blue_pixel_1x1.png");
    let solid_blue_pixel_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes.as_slice(), renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/Solid_green_pixel_1x1.png");
    let solid_green_pixel_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes.as_slice(), renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/bordered_triangle_upside_down_14px.png");
    let worker_base_triangle_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes.as_slice(), renderer);
    let diffuse_bytes = include_bytes!("../../../../textures/basic_2_cropped.png");
    let base_large_house_texture_view = load_sprite_from_memory_uncropped(&diffuse_bytes.as_slice(), renderer);


    let mut texture_view_array = Vec::new();



    texture_view_array.push(dwarf_base_house_texture_view);
    texture_view_array.push(dwarf_base_house_texture_view2);
    texture_view_array.push(black_pixel_texture_view);
    texture_view_array.push(solid_blue_pixel_texture_view);
    texture_view_array.push(solid_green_pixel_texture_view);
    texture_view_array.push(worker_base_triangle_texture_view);
    texture_view_array.push(base_large_house_texture_view);

    texture_view_array
}