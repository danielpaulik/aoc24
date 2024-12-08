use std::collections::HashSet;

fn input() -> &'static str {
    include_str!("../inputs/1.txt")
}

// Simplest & fastest idea: given each 4x4 cell in the input matrix,
// check if it contains the XMAS or SAMX pattern in any direction
// manually, byte-by-byte.
//
// But that's boring.
//
// Let's do it the hard way instead, and learn something new.
// Basic idea: implement iterators for the letter matrix that can yield
// rows, columns, diagonals, etc. without copying the input data.
// Then, use these iterators to check for XMAS/SAMX occurrences.

enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
    AntiDiagonal
}

struct LetterMatrix<'a> {
    letters: &'a [u8],
    width: usize
}

struct LinearIterator<'a> {
    matrix: &'a LetterMatrix<'a>,
    direction: Direction,
    cursor: usize,
    did_wrap: bool
}

impl<'a> LetterMatrix<'a> {
    fn rows(&'a self) -> LinearIterator<'a> {
        LinearIterator {
            matrix: self,
            direction: Direction::Horizontal,
            cursor: 0,
            did_wrap: false
        }
    }

    fn columns(&'a self) -> LinearIterator<'a> {
        LinearIterator {
            matrix: self,
            direction: Direction::Vertical,
            cursor: 0,
            did_wrap: false
        }
    }

    fn diagonals(&'a self) -> LinearIterator<'a> {
        LinearIterator {
            matrix: self,
            direction: Direction::Diagonal,
            cursor: 0,
            did_wrap: false
        }
    }

    fn anti_diagonals(&'a self) -> LinearIterator<'a> {
        LinearIterator {
            matrix: self,
            direction: Direction::AntiDiagonal,
            cursor: 0,
            did_wrap: false
        }
    }
}

fn get_step(matrix: &LetterMatrix, direction: &Direction) -> usize {
    match direction {
        Direction::Horizontal => 1,
        Direction::Vertical => matrix.width + 1,
        Direction::Diagonal => matrix.width + 2,
        Direction::AntiDiagonal => matrix.width
    }
}

fn get_wrapping_step(direction: &Direction) -> usize {
    match direction {
        Direction::Horizontal => 0,
        Direction::Vertical => 1,
        Direction::Diagonal => 0,
        Direction::AntiDiagonal => 0
    }
}

fn get_newline_step(matrix: &LetterMatrix, direction: &Direction) -> usize {
    match direction {
        Direction::Horizontal => 1,
        Direction::Vertical => matrix.letters.len() - matrix.width,
        Direction::Diagonal => 1,
        Direction::AntiDiagonal => matrix.letters.len() - 1
    }
}

fn advance_cursor(matrix: &LetterMatrix, cursor: usize, direction: &Direction) -> (usize, bool) {
    let step = get_step(matrix, direction);
    let moved = cursor + step;
    let mut moved_wrapped = moved % matrix.letters.len();
    let mut did_wrap = moved >= matrix.letters.len();

    if did_wrap {
        moved_wrapped = (moved_wrapped + get_wrapping_step(direction)) % matrix.letters.len();
    }

    if matrix.letters[moved_wrapped] == b'\n' {
        moved_wrapped = (moved_wrapped + get_newline_step(matrix, direction)) % matrix.letters.len();
        did_wrap = true;
    }

    (moved_wrapped, did_wrap)
}

impl Iterator for LinearIterator<'_> {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.did_wrap && self.cursor == 0 {
            return None;
        }

        if self.did_wrap {
            self.did_wrap = false;
            Some((0, b'\n'))
        } else {
            let prev_cursor = self.cursor;
            (self.cursor, self.did_wrap) = advance_cursor(self.matrix, self.cursor, &self.direction);
            Some((prev_cursor, self.matrix.letters[prev_cursor]))
        }
    }
}

enum ReadDirection {
    Forward,
    Backward,
}

fn find_xmas_occurences(letters: LinearIterator) -> usize {
    let mut xmas_count = 0;

    let mut last_char = b'\0';
    let mut direction: Option<ReadDirection> = None;
    for (_, char) in letters {
        match char {
            b'X' => {
                if matches!(direction, Some(ReadDirection::Backward)) && last_char == b'M' {
                    xmas_count += 1;
                }
                direction = Some(ReadDirection::Forward);
            },
            b'M' => {
                match direction {
                    Some(ReadDirection::Forward) => {
                        if last_char != b'X' {
                            direction = None;
                        }
                    },
                    Some(ReadDirection::Backward) => {
                        if last_char != b'A' {
                            direction = None;
                        }
                    },
                    _ => {}
                }
            },
            b'A' => {
                match direction {
                    Some(ReadDirection::Forward) => {
                        if last_char != b'M' {
                            direction = None;
                        }
                    },
                    Some(ReadDirection::Backward) => {
                        if last_char != b'S' {
                            direction = None;
                        }
                    },
                    _ => {}
                }
            },
            b'S' => {
                if matches!(direction, Some(ReadDirection::Forward)) && last_char == b'A' {
                    xmas_count += 1;
                }
                direction = Some(ReadDirection::Backward);
            },
            _ => {
                direction = None;
            }
        }
        last_char = char;
    }

    xmas_count
}

fn day4_1() -> usize {
    let matrix = LetterMatrix {
        letters: input().as_bytes(),
        width: input().find('\n').unwrap(),
    };
    let xmas_count_in_rows = find_xmas_occurences(matrix.rows());
    let xmas_count_in_columns = find_xmas_occurences(matrix.columns());
    let xmas_count_in_diagonals = find_xmas_occurences(matrix.diagonals());
    let xmas_count_in_anti_diagonals = find_xmas_occurences(matrix.anti_diagonals());
    xmas_count_in_rows + xmas_count_in_columns + xmas_count_in_diagonals + xmas_count_in_anti_diagonals
}

fn find_mas_centers(letters: LinearIterator) -> HashSet<usize> {
    let mut mas_centers = HashSet::new();

    let mut last_char = b'\0';
    let mut direction: Option<ReadDirection> = None;
    let mut last_a: usize = 0;
    for (index, char) in letters {
        match char {
            b'M' => {
                if matches!(direction, Some(ReadDirection::Backward)) && last_char == b'A' {
                    mas_centers.insert(last_a);
                }
                direction = Some(ReadDirection::Forward);
            },
            b'A' => {
                match direction {
                    Some(ReadDirection::Forward) => {
                        if last_char != b'M' {
                            direction = None;
                        }
                    },
                    Some(ReadDirection::Backward) => {
                        if last_char != b'S' {
                            direction = None;
                        }
                    },
                    _ => {}
                }
                last_a = index;
            },
            b'S' => {
                if matches!(direction, Some(ReadDirection::Forward)) && last_char == b'A' {
                    mas_centers.insert(last_a);
                }
                direction = Some(ReadDirection::Backward);
            },
            _ => {
                direction = None;
            }
        }
        last_char = char;
    }

    mas_centers
}

fn day4_2() -> usize {
    let matrix = LetterMatrix {
        letters: input().as_bytes(),
        width: input().find('\n').unwrap(),
    };
    let mas_centers_in_diagonals = find_mas_centers(matrix.diagonals());
    let mas_centers_in_anti_diagonals = find_mas_centers(matrix.anti_diagonals());
    mas_centers_in_diagonals.intersection(&mas_centers_in_anti_diagonals).count()
}

fn main() {
    assert_eq!(day4_1(), 2517);
    assert_eq!(day4_2(), 1960);
}
