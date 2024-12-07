use regex::Regex;

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

pub fn task2(input: String) {
    // IT TOOK 22seconds to run on input.txt
    let lines = input.lines().collect::<Vec<&str>>();

    let re_validator = Regex::new(r"[MS].[MS]").unwrap();

    let mut result = 0;

    for (ln, line) in lines.iter().enumerate() {
        for idx in 0..line.len() {
            let Some(first3) = line.get(idx..idx + 3) else {
                break;
            };
            if !re_validator.is_match(first3) {
                continue;
            }
            let Some(next_line) = lines.get(ln + 1) else {
                break;
            };
            let Some(last_line) = lines.get(ln + 2) else {
                break;
            };
            let next3 = next_line.get(idx..idx + 3).unwrap();
            let last3 = last_line.get(idx..idx + 3).unwrap();

            if has_x_mas(first3, next3, last3) {
                result += 1;
            }
        }
    }

    println!("Result - {}", result);
}

fn has_x_mas(line1: &str, line2: &str, line3: &str) -> bool {
    // println!("Checking:\n{}\n{}\n{}\n", line1, line2, line3);
    let re_center_a = Regex::new(r".A.").unwrap();
    let re_m_m = Regex::new(r"M.M").unwrap();
    let re_m_s = Regex::new(r"M.S").unwrap();
    let re_s_m = Regex::new(r"S.M").unwrap();
    let re_s_s = Regex::new(r"S.S").unwrap();
    if !re_center_a.is_match(line2) {
        return false;
    };
    if re_m_m.is_match(line1) && re_s_s.is_match(line3) {
        return true;
    }
    if re_s_s.is_match(line1) && re_m_m.is_match(line3) {
        return true;
    }
    if re_m_s.is_match(line1) && re_m_s.is_match(line3) {
        return true;
    }
    if re_s_m.is_match(line1) && re_s_m.is_match(line3) {
        return true;
    }
    return false;
}

fn parse(input: &String) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
