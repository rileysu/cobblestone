#[derive(Debug)]
pub enum DimensionEffect {
    Overworld,
    TheNether,
    TheEnd,
}

#[derive(Debug)]
pub struct DimensionConfig {
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: i32,
    pub monster_spawn_block_light_limit: i32,
    pub natural: bool,
    pub ambient_light: f32,
    pub fixed_time: Option<i64>,
    pub infiniburn: String,
    pub respawn_anchor_works: bool,
    pub has_skylight: bool,
    pub bed_works: bool,
    pub effects: DimensionEffect,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub coordinate_scale: f64,
    pub ultrawarm: bool,
    pub has_ceiling: bool,
}

#[derive(Debug)]
pub struct Dimension {
    pub config: DimensionConfig,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            config: DimensionConfig {
                piglin_safe: false,
                has_raids: true,
                monster_spawn_light_level: 0,
                monster_spawn_block_light_limit: 0,
                natural: true,
                ambient_light: 1.0,
                fixed_time: None,
                infiniburn: "#".to_string(),
                respawn_anchor_works: false,
                has_skylight: true,
                bed_works: true,
                effects: DimensionEffect::Overworld,
                min_y: 0,
                height: 256,
                logical_height: 256,
                coordinate_scale: 1.0,
                ultrawarm: false,
                has_ceiling: false,
            }
        }
    }
}