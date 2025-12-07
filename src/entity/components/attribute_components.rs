use bevy::prelude::Component;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct PhysicalTraits {
    pub size: f32,
    pub sight: f32,
    pub speed: f32,
    pub reach: f32,

    pub metabolism: f32
}

impl PhysicalTraits {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let size: f32 = 1.0;

        let metabolism = rng.gen_range(0.1..1.0) / size.powf(0.25);
        
        let sight = rng.gen_range(500.0..700.0) * size.sqrt();
        let speed = rng.gen_range(1.0..3.0) / size.sqrt();

        let reach = rng.gen_range(0.5..2.0) * size;

        Self {
            size,
            sight,
            speed,
            reach,
            metabolism
        }
    } 
}


#[derive(Component)]
pub struct Vitals {
    pub life: f32,
    pub max_life: f32,

    pub thirst: f32,
    pub max_thirst: f32,

    /// Percentage of max thirst
    pub dehydration_threshold: f32,

    pub hunger: f32,
    pub max_hunger: f32,

    /// Percentage of max hunger
    pub starvation_threshold: f32
}

impl Vitals {
    pub fn new(traits: &PhysicalTraits) -> Self {
        let mut rng = thread_rng();

        let max_life = rng.gen_range(100.0..=150.0) * traits.size;
        let max_thirst = rng.gen_range(100.0..=150.0) * traits.size;
        let max_hunger = rng.gen_range(100.0..=150.0) * traits.size;

        Self {
            max_life: max_life,
            max_thirst: max_thirst,
            max_hunger: max_hunger,

            dehydration_threshold: max_thirst * 0.3,
            starvation_threshold: max_hunger * 0.3,

            life: max_life,
            thirst: max_thirst,
            hunger: max_hunger
        }
    } 
}

#[derive(Component)]
pub struct Starving;