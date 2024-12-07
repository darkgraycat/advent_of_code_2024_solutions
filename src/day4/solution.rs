#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
    fn from_index(i: usize, scanline: usize) -> Self {
        let x = (i as i32) % (scanline as i32);
        let y = (i as i32) / (scanline as i32);
        Vec2 { x, y }
    }
    fn add(&mut self, vec2: &Vec2) -> &mut Self {
        self.x += vec2.x;
        self.y += vec2.y;
        self
    }
}

pub fn task1(input: String) {
    let map = parse(&input);
    let scanline = map[0].len();

    let dir_up = Vec2::new(0, -1);
    let dir_down = Vec2::new(0, 1);
    let dir_left = Vec2::new(-1, 0);
    let dir_right = Vec2::new(1, 0);
    let dir_up_left = Vec2::new(-1, -1);
    let dir_up_right = Vec2::new(1, -1);
    let dir_down_left = Vec2::new(-1, 1);
    let dir_down_right = Vec2::new(1, 1);

    let chars = "XMAS".chars().collect();
    let mut result = 0;

    input.chars().enumerate().for_each(|(i, c)| {
        let position = Vec2::from_index(i, scanline);
        if search(&chars, &map, &position, &dir_up) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_down) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_left) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_right) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_up_left) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_up_right) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_down_left) {
            result += 1;
        }
        if search(&chars, &map, &position, &dir_down_right) {
            result += 1;
        }
    });

    println!("{:?}", result);
}

fn search(chars: &Vec<char>, map: &Vec<Vec<char>>, start: &Vec2, direction: &Vec2) -> bool {
    let mut cursor = start.clone();
    for &ch in chars.iter() {
        if let Some(c) = get_char_from_map(map, &cursor) {
            if ch != c {
                return false;
            }
        } else {
            return false;
        }
        cursor.add(direction);
    }

    true
}

fn get_char_from_map(map: &Vec<Vec<char>>, point: &Vec2) -> Option<char> {
    let Vec2 { x, y } = point;
    if *x < 0 || *y < 0 {
        return None;
    };
    let x = *x as usize;
    let y = *y as usize;

    if y < map.len() && x < map[y].len() {
        Some(map[y][x])
    } else {
        None
    }
}

pub fn task2(input: String) {}

fn parse(input: &String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
