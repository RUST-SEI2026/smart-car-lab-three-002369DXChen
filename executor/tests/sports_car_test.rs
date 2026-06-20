use executor::{Pose, SportsCarExecutor};

mod sports_car_move_tests {
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

mod sports_car_turn_left_tests {
    use super::*;

    #[test]
    fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("L");
        assert_eq!(executor.query(), Pose::new(0, 1, 'N'));
    }

    #[test]
    fn should_return_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BL");
        assert_eq!(executor.query(), Pose::new(0, 1, 'S'));
    }

    #[test]
    fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FL");
        assert_eq!(executor.query(), Pose::new(1, 1, 'N'));
    }

    #[test]
    fn should_return_x_minus_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBL");
        assert_eq!(executor.query(), Pose::new(-1, 1, 'S'));
    }
}

mod sports_car_turn_right_tests {
    use super::*;

    #[test]
    fn should_return_y_minus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("R");
        assert_eq!(executor.query(), Pose::new(0, -1, 'S'));
    }

    #[test]
    fn should_return_y_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BR");
        assert_eq!(executor.query(), Pose::new(0, -1, 'N'));
    }

    #[test]
    fn should_return_x_plus_1_y_minus_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FR");
        assert_eq!(executor.query(), Pose::new(1, -1, 'S'));
    }

    #[test]
    fn should_return_x_minus_1_y_minus_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
        let mut executor = SportsCarExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBR");
        assert_eq!(executor.query(), Pose::new(-1, -1, 'N'));
    }
}
