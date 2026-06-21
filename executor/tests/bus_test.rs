use executor::{BusExecutor, Pose};

mod bus_move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("M");
        assert_eq!(executor.query(), Pose::new(1, 0, 'E'));
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_bm_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BM");
        assert_eq!(executor.query(), Pose::new(-1, 0, 'E'));
    }

    #[test]
    fn should_return_x_plus_2_given_command_is_fm_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FM");
        assert_eq!(executor.query(), Pose::new(2, 0, 'E'));
    }

    #[test]
    fn should_return_x_minus_2_given_command_is_fbm_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBM");
        assert_eq!(executor.query(), Pose::new(-2, 0, 'E'));
    }
}

mod bus_turn_left_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("L");
        assert_eq!(executor.query(), Pose::new(1, 0, 'N'));
    }

    #[test]
    fn should_return_x_minus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BL");
        assert_eq!(executor.query(), Pose::new(-1, 0, 'S'));
    }

    #[test]
    fn should_return_x_plus_2_and_facing_n_given_command_is_fl_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FL");
        assert_eq!(executor.query(), Pose::new(2, 0, 'N'));
    }

    #[test]
    fn should_return_x_minus_2_and_facing_s_given_command_is_fbl_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBL");
        assert_eq!(executor.query(), Pose::new(-2, 0, 'S'));
    }
}

mod bus_turn_right_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("R");
        assert_eq!(executor.query(), Pose::new(1, 0, 'S'));
    }

    #[test]
    fn should_return_x_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("BR");
        assert_eq!(executor.query(), Pose::new(-1, 0, 'N'));
    }

    #[test]
    fn should_return_x_plus_2_and_facing_s_given_command_is_fr_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FR");
        assert_eq!(executor.query(), Pose::new(2, 0, 'S'));
    }

    #[test]
    fn should_return_x_minus_2_and_facing_n_given_command_is_fbr_and_facing_is_e() {
        let mut executor = BusExecutor::with_pose(Pose::new(0, 0, 'E'));
        executor.execute("FBR");
        assert_eq!(executor.query(), Pose::new(-2, 0, 'N'));
    }
}
