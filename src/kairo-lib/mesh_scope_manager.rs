//! mesh_scope_manager.rs
//! Manages mesh scope transitions based on WAU scores.

use crate::wau_config::WauThresholds;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Scope {
    Personal,
    Family,
    Group,
    Community,
    World,
}

pub struct MeshScopeManager {
    thresholds: WauThresholds,
    current_scope: Scope,
}

impl MeshScopeManager {
    /// Creates a new manager from provided thresholds.
    pub fn new(thresholds: WauThresholds) -> Self {
        Self {
            thresholds,
            current_scope: Scope::Personal,
        }
    }

    /// Returns the current scope.
    pub fn current_scope(&self) -> Scope {
        self.current_scope
    }

    /// Updates the current scope based on a WAU score applying hysteresis.
    pub fn update_scope(&mut self, score: f32) {
        let t = &self.thresholds;
        let up = t.hysteresis.up_margin;
        let down = t.hysteresis.down_margin;

        use Scope::*;
        self.current_scope = match self.current_scope {
            Personal => {
                if score >= t.family + up {
                    Family
                } else {
                    Personal
                }
            }
            Family => {
                if score >= t.group + up {
                    Group
                } else if score < t.family - down {
                    Personal
                } else {
                    Family
                }
            }
            Group => {
                if score >= t.community + up {
                    Community
                } else if score < t.group - down {
                    Family
                } else {
                    Group
                }
            }
            Community => {
                if score >= t.world + up {
                    World
                } else if score < t.community - down {
                    Group
                } else {
                    Community
                }
            }
            World => {
                if score < t.world - down {
                    Community
                } else {
                    World
                }
            }
        };
    }
}
