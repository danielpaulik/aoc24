use core::panic;
use std::{cmp::{min, max}, collections::{HashMap, HashSet}};

fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

type ObstacleMap = HashMap<usize, Vec<usize>>;

#[derive(Clone)]
struct Obstacles {
    vertical: ObstacleMap,
    horizontal: ObstacleMap,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Obstacles {
    fn add(&mut self, obstacle: &Position) {
        let vertical = self.vertical.entry(obstacle.x).or_default();
        vertical.push(obstacle.y);
        vertical.sort_unstable();

        let horizontal = self.horizontal.entry(obstacle.y).or_default();
        horizontal.push(obstacle.x);
        horizontal.sort_unstable();
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Guard {
    position: Position,
    facing: Facing,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    obstacles: Obstacles,
    guard_start: Option<Guard>,
}

fn get_guard_facing_from_char(character: char) -> Facing {
    match character {
        '^' => Facing::Up,
        'v' => Facing::Down,
        '<' => Facing::Left,
        '>' => Facing::Right,
        _ => panic!("Invalid character"),
    }
}

fn read_map() -> Map {
    let mut map = Map {
        width: 0,
        height: 0,
        obstacles: Obstacles { vertical: HashMap::new(), horizontal: HashMap::new() },
        guard_start: None,
    };

    input().lines()
        .enumerate()
        .for_each(|(row, line)| {
            map.height = max(map.height, row + 1);
            line.char_indices()
                .for_each(|(column, character)| {
                    map.width = max(map.width, column + 1);
                    match character {
                        '#' => {
                            map.obstacles.add(&Position {x: column, y: row});
                        },
                        '^' | '>' | 'v' | '<' => {
                            assert!(map.guard_start.is_none(), "Multiple guards found");
                            map.guard_start = Some(Guard { position: Position { x: column, y: row }, facing: get_guard_facing_from_char(character) });
                        },
                        _ => {},
                    }
                });
    });

    map
}

fn get_position_before_position(pos: usize, facing: &Facing) -> usize {
    match facing {
        Facing::Up | Facing::Left => pos + 1,
        Facing::Down | Facing::Right => pos - 1,
    }
}

fn turn(facing: &Facing) -> Facing {
    match facing {
        Facing::Up => Facing::Right,
        Facing::Right => Facing::Down,
        Facing::Down => Facing::Left,
        Facing::Left => Facing::Up,
    }
}

fn find_position_before_next_obstacle(guard: &Guard, obstacles: &Obstacles, map_size: (usize, usize)) -> (Position, bool) {
    match guard.facing {
        Facing::Up => {
            obstacles.vertical.get(&guard.position.x)
                .and_then(|column|
                    column.iter().rfind(|&y| y < &guard.position.y ))
                .map(|y|
                    (Position { x: guard.position.x, y: get_position_before_position(*y, &guard.facing) }, true))
                .unwrap_or((Position { x: guard.position.x, y: 0 }, false))
        },
        Facing::Down => {
            obstacles.vertical.get(&guard.position.x)
                .and_then(|column|
                    column.iter().find(|&y| y > &guard.position.y ))
                .map(|y|
                    (Position { x: guard.position.x, y: get_position_before_position(*y, &guard.facing) }, true))
                .unwrap_or((Position { x: guard.position.x, y: map_size.1 - 1 }, false))
        },
        Facing::Left => {
            obstacles.horizontal.get(&guard.position.y)
                .and_then(|row|
                    row.iter().rfind(|&x| x < &guard.position.x ))
                .map(|x|
                    (Position { x: get_position_before_position(*x, &guard.facing), y: guard.position.y }, true))
                .unwrap_or((Position { x: 0, y: guard.position.y }, false))
        },
        Facing::Right => {
            obstacles.horizontal.get(&guard.position.y)
                .and_then(|row|
                    row.iter().find(|&x| x > &guard.position.x ))
                .map(|x|
                    (Position { x: get_position_before_position(*x, &guard.facing), y: guard.position.y }, true))
                .unwrap_or((Position { x: map_size.0 - 1, y: guard.position.y }, false))
        },
    }
}

fn range_between(start: usize, end: usize) -> std::ops::RangeInclusive<usize> {
    min(start, end)..=max(start, end)
}

fn get_positions_between(start: &Position, end: &Position) -> Vec<Position> {
    let mut positions = Vec::new();

    if start.x != end.x {
        range_between(start.x, end.x).for_each(|x| {
            positions.push(Position { x, y: start.y });
        });
    }
    if start.y != end.y {
        range_between(start.y, end.y).for_each(|y| {
            positions.push(Position { x: start.x, y });
        });
    }

    positions
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Waypoint {
    position: Position,
    facing: Facing,
}

fn route<F>(mut guard: Guard, obstacles: &Obstacles, map_size: (usize, usize), mut callback: F)
where F: FnMut(Waypoint) -> bool {
    loop {
        let (next_position, is_facing_obstacle) = find_position_before_next_obstacle(&guard, obstacles, map_size);
        guard.position = next_position;

        if !callback(Waypoint { position: guard.position.clone(), facing: guard.facing }) {
            return
        }

        if !is_facing_obstacle {
            return
        }

        guard.facing = turn(&guard.facing);
    }
}

fn collect_visited_positions(map: &Map) -> HashSet<Position> {
    let mut previous_position = map.guard_start.clone().unwrap().position;
    let mut visited_positions = HashSet::new();

    route(map.guard_start.clone().unwrap(), &map.obstacles, (map.width, map.height), |waypoint| {
        for position in get_positions_between(&previous_position, &waypoint.position) {
            visited_positions.insert(position);
        }
        previous_position = waypoint.position;
        true
    });

    visited_positions
}

fn day6_1() -> usize {
    let map = read_map();

    collect_visited_positions(&map).len()
}

fn day6_2() -> usize {
    let map = read_map();
    
    // The guard will only encounter additional obstacles if they are placed
    // along its route, so we only need to check the visited positions
    // let visited_positions = collect_visited_positions(&map);
    let visited_positions = collect_visited_positions(&map);
    
    let mut looping_obstacles = Vec::new();

    visited_positions.iter().for_each(| position| {
        let mut map_copy = map.clone();

        map_copy.obstacles.add(position);

        let mut visited_waypoints = HashSet::new();

        let mut is_loop = false;
        route(map_copy.guard_start.unwrap(), &map_copy.obstacles, (map_copy.width, map_copy.height), |waypoint| {
            let is_new_waypoint = visited_waypoints.insert(waypoint);
            is_loop |= !is_new_waypoint;
            is_new_waypoint
        });

        if is_loop {
            looping_obstacles.push(position);
        }
    });

    looping_obstacles.len()
}

fn main() {
    assert_eq!(day6_1(), 5145);
    assert_eq!(day6_2(), 1523);
}
