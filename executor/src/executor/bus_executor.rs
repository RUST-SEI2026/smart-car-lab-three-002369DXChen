use crate::bus_state::BusState;
use crate::pose::Pose;
use crate::Executor;

pub struct BusExecutor;

impl BusExecutor {
    pub fn with_pose(pose: Pose) -> Executor {
        Executor {
            pose,
            state: Box::new(BusState::default()),
        }
    }
}
