use executor::{Pose, SportsCarExecutor};

mod sports_car_move_tasts {
    use super::*;

    #[test]
    fn shoule_return_x_plus_2_given_command_is_m_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("M");
        assert_eq!(executor.query(), Pose::new(2, 0, 'E'));
    }

    #[test]
    fn should_return_x_minus_2_given_command_is_bm_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BM");
        assert_eq!(executor.query(), Pose::new(-2, 0, 'E'));
    }

    #[test]
    fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FM");
        assert_eq!(executor.query(), Pose::new(4, 0, 'E'));
    }

    #[test]
    fn should_return_x_minus_4_given_command_is_fbm_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBM");
        assert_eq!(executor.query(), Pose::new(-4, 0, 'E'));
    }
    
}
