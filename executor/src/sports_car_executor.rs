use crate::assembler::Assembler;
use crate::pose::Pose;
use crate::sports_car_state::SportsCarState;
use crate::Executor;

pub struct SportsCarExecutor;

impl SportsCarExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor { 
            pose,
            state: Box::new(SportsCarState::default()),
        }
    }
}