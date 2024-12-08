use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Size {
    width: usize,
    height: usize,
}

type Frequency = char;
type NodeMap = HashMap<Frequency, Vec<Position>>;

struct Map {
    nodes: NodeMap,
    size: Size,
}

fn read_map() -> Map {
    let mut map = Map {
        nodes: NodeMap::new(),
        size: Size { width: 0, height: 0 },
    };
    input().lines().enumerate().for_each(|(line_index, line)| {
        map.size.height = std::cmp::max(map.size.height, line_index + 1);
        map.size.width = line.len();
        line.chars().enumerate().for_each(|(char_index, character)| {
            if character == '.' {
                return;
            }
            map.nodes.entry(character).or_default().push(Position { x: char_index, y: line_index });
        });
    });
    map
}

fn is_within_map_bounds(position: &Position, map_bounds: &Size) -> bool {
    position.x < map_bounds.width && position.y < map_bounds.height
}

fn for_each_node_pairs<F>(nodes: &[Position], mut callback: F)
where F: FnMut(&Position, &Position) {
    nodes.iter().enumerate().cartesian_product(nodes.iter().enumerate()).for_each(|((index_a, node_a), (index_b, node_b))| {
        if index_a != index_b {
            callback(node_a, node_b);
        }
    });
}

fn get_equal_distance_antinodes(nodes: &[Position], map_bounds: &Size) -> Vec<Position> {
    let mut antinodes = Vec::new();

    for_each_node_pairs(nodes, |node_a, node_b| {
        let antinode = Position {
            x: node_b.x.wrapping_sub(node_a.x).wrapping_add(node_b.x),
            y: node_b.y.wrapping_sub(node_a.y).wrapping_add(node_b.y),
        };
        if is_within_map_bounds(&antinode, map_bounds) {
            antinodes.push(antinode);
        }
    });

    antinodes
}

fn day8_1() -> usize {
    let map = read_map();
    let mut unique_antinodes: HashSet<Position> = HashSet::new();
    map.nodes.values().for_each(|nodes| {
        unique_antinodes.extend(get_equal_distance_antinodes(nodes, &map.size).iter());
    });
    unique_antinodes.len()
}

fn greatest_common_divisor(a: isize, b: isize) -> isize {
    (1..=(std::cmp::min(a, b)/2))
    .filter(|i| { i % a == 0 && i % b == 0 })
    .max()
    .unwrap_or(1)
}

fn get_collinear_antinodes(nodes: &[Position], map_bounds: &Size) -> Vec<Position> {
    let mut antinodes = Vec::new();

    for_each_node_pairs(nodes, |node_a, node_b| {
        let x_diff = (node_b.x as isize) - (node_a.x as isize);
        let y_diff = (node_b.y as isize) - (node_a.y as isize);
        let gcd = greatest_common_divisor(x_diff, y_diff);
        let x_diff = x_diff / gcd;
        let y_diff = y_diff / gcd;
        let mut antinode = *node_b;

        loop {
            antinode = Position {
                x: antinode.x.wrapping_add_signed(x_diff),
                y: antinode.y.wrapping_add_signed(y_diff),
            };
            if !is_within_map_bounds(&antinode, map_bounds) {
                break;
            }
            antinodes.push(antinode);
        }
    });

    antinodes
}

fn day8_2() -> usize {
    let map = read_map();
    let mut unique_antinodes: HashSet<Position> = HashSet::new();
    map.nodes.values().for_each(|nodes| {
        unique_antinodes.extend(nodes);
        unique_antinodes.extend(get_collinear_antinodes(nodes, &map.size).iter());
    });
    unique_antinodes.len()
}

fn main() {
    assert_eq!(day8_1(), 214);
    assert_eq!(day8_2(), 809);
}
