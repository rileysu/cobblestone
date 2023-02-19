use std::collections::HashMap;

use crate::{simulation::{data::dimension::{Dimension, DimensionEffect}, resources::dimensions::Dimensions}, codec_data::compound::nbt::{NBTValue, NBT_COMPOUND_ID}};

pub fn generate_registry_type_entry(index: i32, named_dimension: &(&String, &Dimension)) -> NBTValue {
    let name = &named_dimension.0;
    let config = &named_dimension.1.config;

    NBTValue::Compound(HashMap::from([
        ("name".into(), NBTValue::String((*name).clone())),
        ("id".into(), NBTValue::Int(index)),
        ("element".into(), NBTValue::Compound(HashMap::from([
            ("piglin_safe".into(), NBTValue::Byte(config.piglin_safe as i8)),
            ("has_raids".into(), NBTValue::Byte(config.has_raids as i8)),
            ("monster_spawn_light_level".into(), NBTValue::Int(config.monster_spawn_light_level)),
            ("monster_spawn_block_light_limit".into(), NBTValue::Int(config.monster_spawn_block_light_limit)),
            ("natural".into(), NBTValue::Byte(config.natural as i8)),
            ("ambient_light".into(), NBTValue::Float(config.ambient_light)),
            //("fixed_time".into(), NBTValue::Long(config.fixed_time)),
            ("infiniburn".into(), NBTValue::String(config.infiniburn.clone())),
            ("respawn_anchor_works".into(), NBTValue::Byte(config.respawn_anchor_works as i8)),
            ("has_skylight".into(), NBTValue::Byte(config.has_skylight as i8)),
            ("bed_works".into(), NBTValue::Byte(config.bed_works as i8)),
            ("effects".into(), NBTValue::String(match config.effects {
                DimensionEffect::Overworld => "minecraft:overworld".into(),
                DimensionEffect::TheNether => "minecraft:the_nether".into(),
                DimensionEffect::TheEnd => "minecraft:the_end".into(),
            })),
            ("min_y".into(), NBTValue::Int(config.min_y)),
            ("height".into(), NBTValue::Int(config.height)),
            ("logical_height".into(), NBTValue::Int(config.logical_height)),
            ("coordinate_scale".into(), NBTValue::Double(config.coordinate_scale)),
            ("ultrawarm".into(), NBTValue::Byte(config.ultrawarm as i8)),
            ("has_ceiling".into(), NBTValue::Byte(config.has_ceiling as i8)),
        ]))),
    ]))
}

pub fn generate_registry_codec(dimensions: &Dimensions) -> NBTValue {
    NBTValue::Compound(HashMap::from([
        ("".into(),
            NBTValue::Compound(HashMap::from([
                ("minecraft:dimension_type".into(), 
                    NBTValue::Compound(HashMap::from([
                        ("type".into(), NBTValue::String("minecraft:dimension_type".into())),
                        ("value".into(), NBTValue::List(NBT_COMPOUND_ID, dimensions.0.iter().enumerate().map(|(index, dim)| 
                            generate_registry_type_entry(index as i32, &dim)).collect())
                        ),
                    ]))),
                ("minecraft:worldgen/biome".into(), 
                    NBTValue::Compound(HashMap::from([
                        ("type".into(), NBTValue::String("minecraft:worldgen/biome".into())),
                        ("value".into(), NBTValue::List(NBT_COMPOUND_ID, vec![
                            NBTValue::Compound(HashMap::from([
                                ("name".into(), NBTValue::String("minecraft:plains".into())),
                                ("id".into(), NBTValue::Int(0)),
                                ("element".into(), NBTValue::Compound(HashMap::from([
                                    ("temperature".into(), NBTValue::Float(0.8)),
                                    ("precipitation".into(), NBTValue::String("rain".into())),
                                    ("effects".into(), NBTValue::Compound(HashMap::from([
                                        ("water_fog_color".into(), NBTValue::Int(329011)),
                                        ("fog_color".into(), NBTValue::Int(12638463)),
                                        ("water_color".into(), NBTValue::Int(4159204)),
                                        ("mood_sound".into(), NBTValue::Compound(HashMap::from([
                                            ("offset".into(), NBTValue::Double(2.0)),
                                            ("block_search_extent".into(), NBTValue::Int(8)),
                                            ("tick_delay".into(), NBTValue::Int(6000)),
                                            ("sound".into(), NBTValue::String("minecraft:ambient.cave".into())),
                                        ]))),
                                        ("sky_color".into(), NBTValue::Int(7907327)),
                                    ]))),
                                    ("downfall".into(), NBTValue::Float(0.4)),
                                ]))),
                            ]))
                        ])),
                    ]))),
                ("minecraft:chat_type".into(), 
                    NBTValue::Compound(HashMap::from([
                        ("type".into(), NBTValue::String("minecraft:chat_type".into())),
                        ("value".into(), NBTValue::List(NBT_COMPOUND_ID, vec![]),
                        ),
                    ]))),
            ]))
        )
    ]))
}