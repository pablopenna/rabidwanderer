use godot::prelude::*;
use rand::Rng;

#[derive(GodotConvert, Var, Export, Clone)]
#[godot(via = GString)] 
pub(crate) enum EnemyDefinition {
    Wolf,
}

impl EnemyDefinition {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnemyDefinition::Wolf => "Wolf",
        }
    }

    const RANDOM_POOL: [EnemyDefinition; 1] = [
        EnemyDefinition::Wolf,
    ];

    pub(crate) fn random() -> Self {
        let mut rng = rand::rng();
        let idx = rng.random_range(0..Self::RANDOM_POOL.len());
        Self::RANDOM_POOL[idx].clone()
    }
}