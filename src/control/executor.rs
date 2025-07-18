use crate::control::executor::MapPlace::Player;
use rand::Rng;
use std::ops::Add;

pub const X_MAX: i32 = 6;
pub const Y_MAX: i32 = 5;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    // 在计算和计算机视觉领域，pose表示位置和方向
    pub x: i32,
    pub y: i32,
    pub heading: char,
    is_shoot: bool,
} // 定义结构体，便于数据的组织和传递
impl Pose {
    // 提供初始化到指定位置(x, y, heading)的能力
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose {
            x,
            y,
            heading,
            is_shoot: false,
        }
    }

    fn left(self) -> Self {
        Pose {
            x: self.x,
            y: self.y,
            heading: match self.heading {
                'E' => 'N',
                'N' => 'W',
                'W' => 'S',
                'S' => 'E',
                _ => 'N',
            },
            is_shoot: false,
        }
    }

    fn right(self) -> Self {
        Pose {
            x: self.x,
            y: self.y,
            heading: match self.heading {
                'E' => 'S',
                'S' => 'W',
                'W' => 'N',
                'N' => 'E',
                _ => 'N',
            },
            is_shoot: false,
        }
    }
}

impl Add for Pose {
    type Output = Pose;
    fn add(self, other: Pose) -> Pose {
        match self.heading {
            'E' => Pose::new(
                if self.x < X_MAX {
                    self.x + other.x
                } else {
                    self.x
                },
                self.y,
                self.heading,
            ),
            'W' => Pose::new(
                if self.x > -X_MAX {
                    self.x - other.x
                } else {
                    self.x
                },
                self.y,
                self.heading,
            ),
            'S' => Pose::new(
                self.x,
                if self.y < Y_MAX {
                    self.y + other.y
                } else {
                    self.y
                },
                self.heading,
            ),
            'N' => Pose::new(
                self.x,
                if self.y > -Y_MAX {
                    self.y - other.y
                } else {
                    self.y
                },
                self.heading,
            ),
            _ => Pose::new(self.x, self.y, self.heading),
        }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
            is_shoot: false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Executor {
    pose: Option<Pose>,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor { pose: Some(pose) }
    }

    pub fn execute(&mut self, cmds: &str) {
        if let Some(pose) = self.pose {
            self.pose = Some(match cmds {
                "M" => {
                    pose + Pose {
                        x: 1,
                        y: 1,
                        heading: 'N',
                        is_shoot: false,
                    }
                }
                "L" => pose.left(),
                "R" => pose.right(),
                _ => pose,
            });
        }
    }

    pub fn query(&self) -> (i32, i32, char) {
        let x = self.pose.unwrap().x;
        let y = self.pose.unwrap().y;
        let heading = self.pose.unwrap().heading;
        (x, y, heading)
    }
}

#[derive(Copy, Clone)]
pub enum MapPlace {
    Player(Executor),
    Enemy(Executor),
    Shoot(Executor),
    Place,
    Block
}

impl Default for MapPlace {
    fn default() -> Self {
        MapPlace::Place
    }
}

#[derive(Default)]
pub struct Executors {
    pub executors: [[MapPlace; (2 * X_MAX + 1) as usize]; (2 * Y_MAX + 1) as usize],
    player_x: i32,
    player_y: i32,
    enemy_place: Vec<(i32, i32)>,
    shoot_place: Vec<(i32, i32)>,
    pub point: i32,
    pub is_lose: bool,
}

impl Executors {
    pub fn new() -> Self {
        let mut executors = [[MapPlace::Place; (2 * X_MAX + 1) as usize]; (2 * Y_MAX + 1) as usize];
        let mut enemy_place = Vec::new();
        for _ in 0..10{
            let mut rng = rand::thread_rng();
            let block_x = rng.gen_range(0..13);
            let block_y = rng.gen_range(0..11);
            executors[block_y][block_x] = MapPlace::Block;
        }
        executors[(0 + Y_MAX + Y_MAX) as usize][(0 + X_MAX) as usize] =
            Player(Executor::with_pose(Pose::new(0, Y_MAX, 'N')));
        executors[(-Y_MAX + Y_MAX) as usize][(X_MAX - 2 + X_MAX) as usize] =
            MapPlace::Enemy(Executor::with_pose(Pose::new(X_MAX - 2, -Y_MAX, 'S')));
        enemy_place.push((X_MAX - 2, -Y_MAX));
        executors[(-Y_MAX + Y_MAX) as usize][(-X_MAX + 2 + X_MAX) as usize] =
            MapPlace::Enemy(Executor::with_pose(Pose::new(-X_MAX + 2, -Y_MAX, 'S')));
        enemy_place.push((-X_MAX + 2, -Y_MAX));
        Executors {
            executors,
            player_x: X_MAX,
            player_y: Y_MAX + Y_MAX,
            enemy_place,
            shoot_place: Vec::new(),
            point: 0,
            is_lose: false,
        }
    }

    pub fn spawn(&mut self) {
        if let MapPlace::Place =
            self.executors[(-Y_MAX + Y_MAX) as usize][(X_MAX - 2 + X_MAX) as usize]
        {
            self.executors[(-Y_MAX + Y_MAX) as usize][(X_MAX - 2 + X_MAX) as usize] =
                MapPlace::Enemy(Executor::with_pose(Pose::new(X_MAX - 2, -Y_MAX, 'S')));
            self.enemy_place.push((X_MAX - 2, -Y_MAX));
        }
        if let MapPlace::Place =
            self.executors[(-Y_MAX + Y_MAX) as usize][(-X_MAX + 2 + X_MAX) as usize]
        {
            self.executors[(-Y_MAX + Y_MAX) as usize][(-X_MAX + 2 + X_MAX) as usize] =
                MapPlace::Enemy(Executor::with_pose(Pose::new(-X_MAX + 2, -Y_MAX, 'S')));
            self.enemy_place.push((-X_MAX + 2, -Y_MAX));
        }
    }

    pub fn player_move(&mut self, cmds: &str) {
        if let Player(mut player) = self.executors[self.player_y as usize][self.player_x as usize] {
            match cmds {
                "M" => {
                    let (x_, y_, heading) = player.query();
                    let mut temp = Executor::with_pose(Pose::new(x_, y_, heading));
                    temp.execute("M");
                    let (x, y, _) = temp.query();
                    if let MapPlace::Place =
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize]
                    {
                        player.execute(cmds);
                        let (player_x, player_y, _) = player.query();
                        self.player_x = player_x + X_MAX;
                        self.player_y = player_y + Y_MAX;
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] = Player(player);
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                    }
                }
                _ => {
                    player.execute(cmds);
                    self.executors[self.player_y as usize][self.player_x as usize] = Player(player);
                }
            }
        }
    }

    pub fn enemy_move(&mut self) {
        let mut new_place = Vec::<(i32, i32)>::new();
        for (enemy_x, enemy_y) in &self.enemy_place {
            if let MapPlace::Enemy(mut enemy) =
                self.executors[(enemy_y + Y_MAX) as usize][(enemy_x + X_MAX) as usize]
            {
                let mut rng = rand::thread_rng();
                let behave = rng.gen_range(0..6);
                match behave {
                    0 | 3 | 4 | 5 => {
                        let (x_, y_, heading) = enemy.query();
                        let mut temp = Executor::with_pose(Pose::new(x_, y_, heading));
                        temp.execute("M");
                        let (x, y, _) = temp.query();
                        if let MapPlace::Place =
                            self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize]
                        {
                            enemy.execute("M");
                            self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                                MapPlace::Enemy(enemy);
                            new_place.push((x, y));
                            self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                                MapPlace::Place;
                        } else {
                            let behave = rng.gen_range(0..2);
                            match behave {
                                1 => enemy.execute("R"),
                                2 => enemy.execute("L"),
                                _ => {}
                            }
                            self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                                MapPlace::Enemy(enemy);
                            new_place.push((x_, y_));
                        }
                    }
                    1 => {
                        enemy.execute("R");
                        self.executors[(enemy_y + Y_MAX) as usize][(enemy_x + X_MAX) as usize] =
                            MapPlace::Enemy(enemy);
                        new_place.push((*enemy_x, *enemy_y));
                    }
                    2 => {
                        enemy.execute("L");
                        self.executors[(enemy_y + Y_MAX) as usize][(enemy_x + X_MAX) as usize] =
                            MapPlace::Enemy(enemy);
                        new_place.push((*enemy_x, *enemy_y));
                    }
                    _ => {}
                }
            }
        }
        self.enemy_place = new_place;
    }

    pub fn shoot(&mut self) {
        let mut shoot = self.enemy_place.clone();
        shoot.push((self.player_x - X_MAX, self.player_y - Y_MAX));
        for (enemy_x, enemy_y) in &shoot {
            if let MapPlace::Enemy(enemy) | Player(enemy) =
                self.executors[(enemy_y + Y_MAX) as usize][(enemy_x + X_MAX) as usize]
            {
                let (x_, y_, heading) = enemy.query();
                let mut temp = Executor::with_pose(Pose::new(x_, y_, heading));
                temp.execute("M");
                let (x, y, _) = temp.query();
                if let MapPlace::Place = self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize]
                {
                    let shoot = Executor::with_pose(Pose::new(x, y, heading));
                    shoot.pose.unwrap().is_shoot = true;
                    self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                        MapPlace::Shoot(shoot);
                    self.shoot_place.push((x, y));
                }
            }
        }
    }

    pub fn shoot_move(&mut self) {
        let mut new_place = Vec::<(i32, i32)>::new();
        for (shoot_x, shoot_y) in &self.shoot_place {
            if let MapPlace::Shoot(mut shoot) =
                self.executors[(shoot_y + Y_MAX) as usize][(shoot_x + X_MAX) as usize]
            {
                let (x_, y_, heading) = shoot.query();
                let mut temp = Executor::with_pose(Pose::new(x_, y_, heading));
                temp.execute("M");
                let (x, y, _) = temp.query();
                match self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] {
                    MapPlace::Place => {
                        shoot.execute("M");
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                            MapPlace::Shoot(shoot);
                        new_place.push((x, y));
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                    }
                    MapPlace::Shoot(_shoot) => {
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                            MapPlace::Place;
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                    }
                    MapPlace::Enemy(_enemy) => {
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                            MapPlace::Place;
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                        self.point += 1;
                    }
                    Player(_player) => {
                        self.executors[(y + Y_MAX) as usize][(x + X_MAX) as usize] =
                            MapPlace::Place;
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                        self.is_lose = true;
                    }
                    MapPlace::Block => {
                        self.executors[(y_ + Y_MAX) as usize][(x_ + X_MAX) as usize] =
                            MapPlace::Place;
                    }
                }
            }
        }
        self.shoot_place = new_place;
    }
}
