pub mod control;

#[cfg(test)]
mod tests {
    use crate::control::executor::{Executor, Pose};

    #[test]
    fn go_straight() {
        //测试直走功能
        let mut car = Executor::with_pose(Pose::new(0, 0, 'N'));
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        car.execute("M");
        //前进会撞到边界，数轴边界设定为5，在control的executor.rs中修改边界
        assert_eq!(car, Executor::with_pose(Pose::new(0, 5, 'N')));
    }

    #[test]
    fn go_circle() {
        //测试转向功能
        let mut car = Executor::with_pose(Pose::new(0, 0, 'N'));
        car.execute("L");
        car.execute("L");
        car.execute("L");
        car.execute("L");
        car.execute("R");
        car.execute("R");
        car.execute("R");
        car.execute("R");
        car.execute("M");
        car.execute("L");
        assert_eq!(car, Executor::with_pose(Pose::new(0, 1, 'W')));
    }
}
