use bevy::prelude::*;
use rand::prelude::*;
use crate::lsystem::{LSystem,Rule};

/// ECS component representing an individual plant
#[derive(Component)]
pub struct Plant {
    /// Reference to the stateless L-System blueprint
    pub lsystem: LSystem,

    /// Current developmental state
    pub current_string: String,
    pub iteration: usize,
    pub max_iterations: usize,

    /// Per-plant growth/drawing parameters
    pub step_size: f32,
    pub thickness:f32,
}

impl Plant {
    /// Create a new plant from a given LSystem blueprint
    pub fn new(lsystem: LSystem, step_size: f32, max_iterations: usize, thickness: f32) -> Self {
        let axiom = lsystem.axiom.clone();
        Self {
            lsystem,
            current_string: axiom,
            iteration: 0,
            max_iterations,
            step_size,
            thickness,
        }
    }

    /// Reset the plant to its initial state
    pub fn reset(&mut self) {
        self.current_string = self.lsystem.axiom.clone();
        self.iteration = 0;
    }

    /// Advance the plant one step using its LSystem rules
    pub fn step(&mut self) {
        let mut next = String::new();
        let mut rng = rand::rng();

        for c in self.current_string.chars() {
            // collect all matching rules
            let matches: Vec<&Rule> = self.lsystem.rules.iter()
                .filter(|r| r.predecessor == c)
                .collect();

            if matches.is_empty() {
                next.push(c);
            } else {
                // pick one based on probability
                let mut picked = None;
                for rule in matches {
                if rng.random::<f32>() < rule.probability {
                        picked = Some(rule);
                        break; // stop at first successful probability check
                    }
                }

                if let Some(rule) = picked {
                    next.push_str(&rule.successor);
                } else {
                    // no rule triggered, copy original
                    next.push(c);
                }
            }
        }

        self.current_string = next;
        self.iteration += 1;
    }

    /// Check if the plant has finished growing
    pub fn finished(&self) -> bool {
        self.iteration >= self.max_iterations
    }
}
