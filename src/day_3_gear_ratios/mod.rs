pub fn solve(path: String) -> String {
    let mut numbers: Vec<Number> = Vec::new();

    // Create the grid
    let grid: Vec<Vec<char>> = std::fs::read_to_string(path)
        .unwrap()
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();

    // Set up some variables to track the creation of a number
    let mut number = String::new();
    let mut start_point: GridPoint = GridPoint::new(-1, -1);
    let mut end_point: GridPoint = GridPoint::new(-1, -1);

    // Iterate through the grid and create numbers
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let current_char = grid[i][j];
            let current_char_is_digit = current_char.is_digit(10);

            if current_char_is_digit {
                number.push(current_char);
                end_point = GridPoint::new(i as i32, j as i32);

                if start_point.row == -1 {
                    start_point = GridPoint::new(i as i32, j as i32);
                }
            }

            if number.len() > 0 {
                // Store the number if the next character is not a digit or if we're at the end of the row
                if !current_char_is_digit || j == row.len() - 1 {
                    numbers.push(Number::new(
                        number.parse::<i32>().unwrap(),
                        GridPoint::new(start_point.row, start_point.column),
                        GridPoint::new(end_point.row, end_point.column),
                    ));

                    number = String::new();
                    start_point = GridPoint::new(-1, -1);
                    end_point = GridPoint::new(-1, -1);
                }
            }
        }
    }

    // For every number, we will check if any surrounding points are a symbol (not a '.' or a number)
    for number in &mut numbers {

        // Move from start point to end point
        for i in number.start_index.column..number.end_index.column + 1 {
            if check_neighbor_is_symbol(&grid, number.start_index.row, i) {
                number.is_part = true;
            }
        }
    }

    let answer = numbers
        .iter()
        .filter(|number| number.is_part)
        .fold(0, |acc, number| acc + number.value);

    return answer.to_string();
}

/// Represents a number in the grid
#[derive(Debug)]
struct Number {
    value: i32,
    start_index: GridPoint,
    end_index: GridPoint,
    is_part: bool,
}

impl Number {
    pub fn new(value: i32, start_index: GridPoint, end_index: GridPoint) -> Number {
        Number {
            value,
            start_index,
            end_index,
            is_part: false,
        }
    }
}

/// Represents a point in the grid
#[derive(Debug)]
struct GridPoint {
    row: i32,
    column: i32,
}

impl GridPoint {
    pub fn new(row: i32, column: i32) -> GridPoint {
        GridPoint { row, column }
    }
}

/// Returns the character at the given point in the grid or '.' if the point is out of bounds.
fn char_at(grid: &Vec<Vec<char>>, row: i32, column: i32) -> char {
    let rows: i32 = grid[row as usize].len() as i32;
    let columns: i32 = grid.len() as i32;

    if (column >= 0 && column < columns && row >= 0 && row < rows) {
        return grid[row as usize][column as usize];
    }

    return '.';
}

/// Returns true if any of the surrounding points are a symbol (not a '.' or a number)
fn check_neighbor_is_symbol(grid: &Vec<Vec<char>>, row: i32, column: i32) -> bool {
    // upper left
    if char_at_is_symbol(grid, row - 1, column - 1) {
        return true;
    }

    // upper middle
    if char_at_is_symbol(grid, row - 1, column) {
        return true;
    }

    // upper right
    if char_at_is_symbol(grid, row - 1, column + 1) {
        return true;
    }

    // left
    if char_at_is_symbol(grid, row, column - 1) {
        return true;
    }

    // right
    if char_at_is_symbol(grid, row, column + 1) {
        return true;
    }

    // lower left
    if char_at_is_symbol(grid, row + 1, column - 1) {
        return true;
    }

    // lower middle
    if char_at_is_symbol(grid, row + 1, column) {
        return true;
    }

    // lower right
    if char_at_is_symbol(grid, row + 1, column + 1) {
        return true;
    }

    return false;
}

/// Returns true if the character at the given point in the grid is a symbol (not a '.' or a number)
fn char_at_is_symbol(grid: &Vec<Vec<char>>, row: i32, column: i32) -> bool {
    if (row < 0 || column < 0)
        || (row >= grid.len() as i32 || column >= grid[row as usize].len() as i32)
    {
        return false;
    }
    let char_at = char_at(grid, row, column);

    return char_at != '.' && !char_at.is_digit(10);
}
