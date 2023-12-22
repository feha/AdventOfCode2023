#![allow(dead_code)] // Disables warnings about unused code.
#![allow(unused_macros)] // Disables warnings about unused macros.

use std::collections::{HashMap, HashSet};

// mod utils; // imports utils.rs (not needed as its done in main.rs. not even possible for unknown reason)
use crate::utils::*; // needs to use crate, refers to crate populated by main.rs

use itertools::Itertools;
// Creates struct for Day#, implementing Day trait.
// Thus links passed functions to main.rs (as it has macros instantiating Day# structs for each Day#.rs file).
use proc_macro_aoc;
proc_macro_aoc::impl_day!(part_1, part_2);


// Implementations
fn part_1(input: &str) -> Result<isize, String> {
    let input = input
        .lines()
        .map(|s| s.trim())
        .map(|line| line.chars());
        // .map(|line| line.chars().map(|c| match c {
        //     'S' => Node::Start,
        //     '|' => Node::Ver,
        //     '-' => Node::Hor,
        //     'L' => Node::NE,
        //     'J' => Node::NW,
        //     '7' => Node::SW,
        //     'F' => Node::SE,
        //     '.' => Node::Ground,
        // }));

    let mut grid: HashMap<Coord, char> = HashMap::new();
    let mut start = None;
    input.enumerate()
        .for_each(|(y, row)| row
            .enumerate()
            .for_each(|(x, c)| {
                let pos: Coord = (x as isize, y as isize);
                if c == 'S' {
                    start = Some(pos);
                }
                grid.insert(pos, c);
            })
        );
    let start = start.unwrap();
    
    let mut grid: HashMap<Coord, Node> = grid.into_iter()
        .map(|((x, y), c)| {
            let node = match c {
                'S' => {
                    // Don't bother with this node yet, as we would need to go through all neighbours
                    let left: Coord = (x,y);
                    let right: Coord = (x,y);
                    Node::Pipe(left, right)
                },
                '|' => Node::Pipe((x+0, y-1), (x+0, y+1)),
                '-' => Node::Pipe((x-1, y+0), (x+1, y+0)),
                'L' => Node::Pipe((x+0, y-1), (x+1, y+0)),
                'J' => Node::Pipe((x+0, y-1), (x-1, y+0)),
                '7' => Node::Pipe((x+0, y+1), (x-1, y+0)),
                'F' => Node::Pipe((x+0, y+1), (x+1, y+0)),
                '.' => Node::Ground,
                _ => panic!("Unknown char discovered!"),
            };
            ((x,y), node.clone())
        })
        .collect();

    // Conenct Start-node by finding nodes linking to it.
    let mut start_node = *grid.get(&start).unwrap();
    let mut b = false;
        grid.clone().into_iter().filter(|&(pos, _)| pos != start)
            .for_each(|(pos, node)| {
                match node {
                    Node::Pipe(left, right) => {
                        if let Node::Pipe(start_left, start_right) = start_node {
                            if left == start {
                                if !b {
                                    b = true;
                                    start_node = Node::Pipe(pos, start_right);
                                } else {
                                    start_node = Node::Pipe(start_left, pos);
                                }
                            } else if right == start{
                                if !b {
                                    b = true;
                                    start_node = Node::Pipe(pos, start_right);
                                } else {
                                    start_node = Node::Pipe(start_left, pos);
                                }
                            }
                        }
                    },
                    _ => (),
                }
            });
    grid.insert(start, start_node.clone());

    let mut dists: HashMap<Node, usize> = HashMap::new();
    let mut candidates = Vec::<Node>::new();
    candidates.push(start_node.clone());
    dists.insert(start_node.clone(), 0);
    while !candidates.is_empty() {
        let node = candidates.remove(0);
        if let Node::Pipe(left, right) = node.clone() {
            let dist = dists.get(&node.clone()).unwrap() + 1;
            vec![
                grid.get(&left).unwrap().clone(),
                grid.get(&right).unwrap().clone(),
            ].iter()
                .for_each(|&next_node| {
                    if !dists.contains_key(&next_node) {
                        candidates.push(next_node.clone());
                        dists.insert(next_node.clone(), dist);
                    }
                });
        } else {
            panic!("Pipe somehow led to Ground")
        }
    }
    

    let solution = *dists.values()
        .max().unwrap();

    return Ok(solution as isize);
}

type Coord = (isize, isize);
// impl std::ops::Add<Coord> for Coord {
//     type Output = Coord;
// 
//     fn add(self, other: Coord) -> Coord {
//         (self.0 + other.0, self.1 + other.1)
//     }
// }
// 
// impl std::ops::Sub<Coord> for Coord {
//     type Output = Coord;
// 
//     fn sub(self, other: Coord) -> Coord {
//         (self.0 - other.0, self.1 - other.1)
//     }
// }

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Node {
    // Start(Coord, Coord),
    // Ver(Coord, Coord),
    // Hor(Coord, Coord),
    // NE(Coord, Coord),
    // NW(Coord, Coord),
    // SW(Coord, Coord),
    // SE(Coord, Coord),
    Pipe(Coord, Coord),
    Ground,
}

fn part_2(input: &str) -> Result<isize, String> {
    let pad_lines = "#".repeat(
        input.lines()
            .map(|s| s.trim())
            .next().unwrap().len()
        );
    let pad_line = "#".to_owned();
    let padded_input = pad_lines.clone() + "\n" + &input + "\n" + &pad_lines;

    let input = padded_input
        .lines()
        .map(|s| s.trim())
        .map(|line| pad_line.clone() + line + &pad_line)
        .map(|line| line.chars().collect::<Vec<_>>().into_iter());

    let mut grid: HashMap<Coord, char> = HashMap::new();
    let mut start = None;
    input.enumerate()
        .for_each(|(y, row)| row
            .enumerate()
            .for_each(|(x, c)| {
                let pos: Coord = (x as isize, y as isize);
                if c == 'S' {
                    start = Some(pos);
                }
                grid.insert(pos, c);
            })
        );
    let start = start.unwrap();
    
    let mut grid: HashMap<Coord, Node2> = grid.into_iter()
        .map(|((x, y), c)| {
            let node = match c {
                'S' => {
                    // Don't bother with this node yet, as we would need to go through all neighbours
                    let left: Coord = (x,y);
                    let right: Coord = (x,y);
                    Node2::Pipe(left, right, vec![], vec![])
                },
                '|' => Node2::Pipe((x+0, y-1), (x+0, y+1), vec![(x-1, y+0),], vec![(x+1, y+0),]),
                '-' => Node2::Pipe((x+1, y+0), (x-1, y+0), vec![(x+0, y-1),], vec![(x+0, y+1),]),
                'L' => Node2::Pipe((x+0, y-1), (x+1, y+0), vec![(x-1, y), (x, y+1)], vec![]),
                'J' => Node2::Pipe((x+0, y-1), (x-1, y+0), vec![], vec![(x+1, y), (x, y+1)]),
                '7' => Node2::Pipe((x+0, y+1), (x-1, y+0), vec![(x+1, y), (x, y-1)], vec![]),
                'F' => Node2::Pipe((x+0, y+1), (x+1, y+0), vec![], vec![(x-1, y), (x, y-1)]),
                '.' => Node2::Ground(Ground::Unknown),
                '#' => Node2::Ground(Ground::Outside),
                _ => panic!("Unknown char discovered!"),
            };
            ((x,y), node.clone())
        })
        .collect();

    print_pretty_grid(&grid, Some(&start));

    // Conenct Start-node by finding nodes linking to it.
    let mut start_node = grid.get(&start).unwrap().clone();
    let mut b = false;
    grid.iter().filter(|&(&pos, _)| pos != start)
        .for_each(|(pos, node)| {
            let pos = pos.clone();
            let node = node.clone();
            match node {
                Node2::Pipe(forward, back, _, _) => {
                    if let Node2::Pipe(start_forward, start_back, _, _) = start_node {
                        if forward == start {
                            if !b {
                                b = true;
                                start_node = Node2::Pipe(pos, start_back, vec![], vec![]);
                            } else {
                                start_node = Node2::Pipe(start_forward, pos, vec![], vec![]);
                            }
                        } else if back == start{
                            if !b {
                                b = true;
                                start_node = Node2::Pipe(pos, start_back, vec![], vec![]);
                            } else {
                                start_node = Node2::Pipe(start_forward, pos, vec![], vec![]);
                            }
                        }
                    }
                },
                _ => (),
            }
        });
    grid.insert(start, start_node.clone());
    // ! currently ignoring start nodes left/right. potentially breaking, but likely not (input probabilities).

    print_pretty_grid(&grid, Some(&start));

    // walk cycle, aligning pipe directions/polarity
    // also remember Coord's in collection `cycle`
    let mut cycle = Vec::<Coord>::new();
    let mut last_pos: Option<Coord> = None;
    let mut node_pos = start;
    loop {
        cycle.push(node_pos);
        // cycle.insert(node_pos);
        if let Some(&Node2::Pipe(forward, back, ref left, ref right)) = grid.get(&node_pos) {
            let mut next_pos = forward;
            if Some(next_pos) == last_pos {
                next_pos = back;
                let flipped = Node2::Pipe(back.clone(), forward.clone(), right.clone(), left.clone());
                grid.insert(node_pos, flipped);
            }
            last_pos = Some(node_pos);
            node_pos = next_pos;
            if next_pos == start {
                break;
            }
        } else {
            break;
        }
    }

    // println!("cycle: {:?}", cycle);
    // println!("--------");

    // make all other cells be Ground (detached cycles and pipes don't block)
    grid = grid.iter()
        .map(|(pos, node)|
            if node.is_pipe() && cycle.contains(&pos) || node.is_ground() && node.clone() != Node2::Ground(Ground::Unknown) {
                (pos.clone(), node.clone())
            } else {
                (pos.clone(), Node2::Ground(Ground::Unknown))
            }
        )
        .collect();

    print_pretty_grid(&grid, Some(&start));

    // flood known Outside nodes until they but up against the cycle
    flood_all_known_ground_nodes(&mut grid);
    // grid.iter().for_each(|(node_pos, _)| { flood(grid, &node_pos.clone()); });

    print_pretty_grid(&grid, Some(&start));

    // walking forwards along a cycle, either all right nodes are inside, or all left nodes are
    // As grid is padded AND Outside is butting up against cycle,
    //  there will be some nodes that can see Outside. This allows us to know which side is Inside
    // Color those nodes the cycle can see as Inside or Outside, depending on which side it sees it at.
    // ? Diagonals?
    let left_outside = cycle.iter().any(|node_pos|
            if let Node2::Pipe(_, _, left, _) = grid.get(&node_pos).unwrap() {
                left.iter().any(|node_pos2|
                    Some(Node2::Ground(Ground::Outside)) == grid.get(&node_pos2.clone()).cloned()
                )
            } else {
                panic!("Expected cycle-pipe was not a pipe");
            }
        );
    cycle.iter().for_each(|node_pos| {
            if let Node2::Pipe(_, _, left, right) = grid.get(&node_pos.clone()).unwrap() {
                let mut sides = vec![left.clone(), right.clone()];
                if !left_outside {
                    sides.reverse();
                }
                let colors = vec![Node2::Ground(Ground::Outside), Node2::Ground(Ground::Inside)];
                let mut wrong_color = colors.clone();
                wrong_color.reverse();
                let sides = sides
                    .iter()
                    .zip(colors.iter().zip(wrong_color.iter()))
                    .map(|(arr, color)|
                        arr.iter().map(move |pos| (pos.clone(), color.clone()))
                    )
                    .collect::<Vec<_>>();

                sides.iter().for_each(|side|
                    side.clone().for_each(|(node_pos2, (color, wrong_color))| {
                        let node_pos2 = node_pos2.clone();
                        if Some(Node2::Ground(Ground::Unknown)) == grid.get(&node_pos2).cloned() {
                            grid.insert(node_pos2, color.clone());
                        } else if Some(wrong_color.clone()) == grid.get(&node_pos2).cloned() {
                            panic!("Saw unexpected color!");
                        }
                    })
                );
                // let outside = sides[0];
                // let inside = sides[1];
                // inside.for_each(|(&node_pos2, color)| {
                //     if Some(Node2::Ground(Ground::Unknown)) == grid.get(&node_pos2).cloned() {
                //         grid.insert(node_pos2, color);
                //     } else if Some(Node2::Ground(Ground::Outside)) == grid.get(&node_pos2).cloned() {
                //         panic!("Saw Outside colors on the Inside!");
                //     }
                // });
            } else {
                panic!("Expected cycle-pipe was not a pipe");
            }
        });

    print_pretty_grid(&grid, Some(&start));
    
    // As all nodes butting up against cycle is now known to be colored,
    //  any remaining nodes can simply flood to find which colour they are surrounded by.
    flood_all_known_ground_nodes(&mut grid);

    print_pretty_grid(&grid, Some(&start));

    let solution = grid.values()
        .filter(|&n| n.clone() == Node2::Ground(Ground::Inside))
        .count();

    return Ok(solution as isize);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Ground {
    Unknown,
    Outside,
    Inside,
    Highlight,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node2 {
    Pipe(Coord, Coord, Vec<Coord>, Vec<Coord>), // forward, back, left, right
    Ground(Ground),
}

impl Node2 {
    fn is_pipe(&self) -> bool {
        matches!(self, Node2::Pipe(_, _, _, _))
    }
    fn is_ground(&self) -> bool {
        matches!(self, Node2::Ground(_))
    }
}

fn flood_all_known_ground_nodes(grid: &mut HashMap<Coord, Node2>) {
    let keys: Vec<Coord> = grid.keys().cloned().collect();
    for node_pos in keys {
        flood(grid, &node_pos);
    }
    // grid.clone().iter().for_each(|(node_pos, _)| {
    //     flood(grid, node_pos);
    // });
}
fn flood(grid: &mut HashMap<Coord, Node2>, node_pos: &Coord) {
    let node = grid.get(node_pos).unwrap().clone();
    // can't flood pipe nodes
    if node.is_pipe() {
        return;
    }
    // can't flood unknown colors
    if !(node == Node2::Ground(Ground::Outside) || node == Node2::Ground(Ground::Inside)) {
        return;
    }

    let color = if node == Node2::Ground(Ground::Outside) { Node2::Ground(Ground::Outside) } else { Node2::Ground(Ground::Inside) };
    let mut locked: HashSet<Coord> = HashSet::new();
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut candidates: Vec<Coord> = Vec::new();

    candidates.push(node_pos.clone());
    seen.insert(node_pos.clone());
    while !candidates.is_empty() {
        let node_pos = candidates.remove(0);
        if !locked.insert(node_pos) {
            panic!("Somehow encountered a locked value!")
        }

        let valid_node = grid.get(&node_pos).is_some();
        if valid_node {
            grid.insert(node_pos, color.clone());

            let neighbours = vec![
                    (-1isize,0),
                    (1isize,0),
                    (0,-1isize),
                    (0,1isize),
                ];
            let neighbours = neighbours
                .into_iter()
                .map(|dir| (node_pos.0 + dir.0, node_pos.1 + dir.1));
            let neighbours = neighbours
                .filter(|pos| {
                    let seen = seen.contains(pos);
                    let unknown = grid.get(pos).is_some_and(|neighbour|
                        neighbour.is_ground() && *neighbour == Node2::Ground(Ground::Unknown)
                    );
                    !seen && unknown
                })
                .collect::<Vec<_>>();
            neighbours.iter().for_each(|pos| {
                seen.insert(*pos);
            });
            candidates.extend(neighbours);
        } else {
            panic!("Somehow encountered an invalid node!")
        }
    }
}

fn print_pretty_grid(grid: &HashMap<Coord, Node2>, start: Option<&Coord>) {
    grid.iter()
        .collect::<Vec<_>>()
        .iter()
        .sorted_by(|((x1, y1), _), ((x2, y2), _)|
            Ord::cmp(&y1, &y2).then(Ord::cmp(&x1, &x2))
        )
        .group_by(|((_, y), _)| *y)
        .into_iter()
        .for_each(|(_y, row)| {
            let _line = row.map(|(pos, node)|
                if start.is_some() && (*pos).clone() == start.unwrap().clone() {
                    'S'
                } else {
                    match node {
                        Node2::Pipe(forward, back, _, _) => {
                            let diff_forward = (forward.0 - pos.0, forward.1 - pos.1);
                            let diff_back = (back.0 - pos.0, back.1 - pos.1);
                            match (diff_forward, diff_back) {
                                ((0, -1), (0, 1)) | ((0, 1), (0, -1)) => '|',
                                ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => '-',
                                ((0, -1), (1, 0)) | ((1, 0), (0, -1)) => 'L',
                                ((0, -1), (-1, 0)) | ((-1, 0), (0, -1)) => 'J',
                                ((0, 1), (-1, 0)) | ((-1, 0), (0, 1)) => '7',
                                ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => 'F',
                                _ => panic!("unknown linkage!")
                            }
                        },
                        Node2::Ground(Ground::Outside) => 'O',
                        Node2::Ground(Ground::Inside) => 'I',
                        Node2::Ground(Ground::Unknown) => '.',
                        Node2::Ground(Ground::Highlight) => '#',
                    }
                }
            ).join("");
            // println!("{}", line);
        });
    // println!("--------");
}


// Tests
#[cfg(test)]
mod tests
{
    use super :: * ;

    fn test_helper_1(s : & str, v : isize) {
        assert_eq!(part_1(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_1() {
        // assert_eq!("", "");
        test_helper_1(r#".....
        .S-7.
        .|.|.
        .L-J.
        ....."#, 4);
        test_helper_1(r#"..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ..."#, 8);
    }

    fn test_helper_2(s : & str, v : isize)
    {
        assert_eq!(part_2(s).unwrap(), v, "input: {}", s);
    }

    #[test]
    fn test_2() {
        test_helper_2(r#"...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."#, 4);
        test_helper_2(r#".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."#, 8);
        test_helper_2(r#"FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"#, 10);
    }
}


// fn main() -> Result<(), ()> {
//     println!("hello day!");
//     return Ok(());
// }