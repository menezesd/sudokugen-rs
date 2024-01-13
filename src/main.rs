use rand::seq::SliceRandom;
use rand::Rng;

fn main() {
    let mut sudoku = create_sudoku();
    let puzzle = remove_cells(&mut sudoku, 40);
    println!("Generated Sudoku Puzzle:");
    print_sudoku(&puzzle);
}

/**
 * Creates a new Sudoku grid and fills it.
 * @return A newly generated Sudoku grid.
 */
fn create_sudoku() -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; 9]; 9];
    fill(&mut grid);
    grid
}

/**
 * Fills the given Sudoku grid with numbers in a randomized order.
 *
 * This function generates a sequence of numbers from 1 to 9,
 * shuffles them randomly, and uses them to fill the Sudoku grid.
 *
 * @param grid The Sudoku grid to be filled (modified by reference)
 */
fn fill(grid: &mut Vec<Vec<i32>>) {
    let mut numbers: Vec<i32> = (1..=9).collect();
    let mut rng = rand::thread_rng();
    numbers.shuffle(&mut rng);
    fill_recursive(grid, &numbers);
}

/**
 * Recursively fills the Sudoku grid with valid numbers.
 * @param grid The Sudoku grid to fill.
 * @param numbers The list of numbers to fill the grid with.
 * @return True if the grid is successfully filled, false otherwise.
 */
fn fill_recursive(grid: &mut Vec<Vec<i32>>, numbers: &Vec<i32>) -> bool {
    if let Some((row, col)) = find_empty_location(grid) {
        for &num in numbers {
        if is_safe(grid, row, col, num) {
            grid[row][col] = num;
            if fill_recursive(grid, numbers) {
                return true;
            }
            grid[row][col] = 0;
        }
    }
    false
    }
    else {
        return true;
    }

 }

/**
 * Finds an empty cell (containing 0) in the Sudoku grid.
 * @param grid The Sudoku grid to search.
 * @return A pair of integers representing the coordinates (row, column) of the
 * empty cell, or {-1, -1} if no empty cell is found.
 */
fn find_empty_location(grid: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                return Some((i, j));
            }
        }
    }
    None
}

/**
 * Checks if it's safe to place a number in a given cell.
 * @param grid The Sudoku grid.
 * @param row The row index of the cell.
 * @param col The column index of the cell.
 * @param num The number to check.
 * @return True if it's safe to place the number, false otherwise.
 */
fn is_safe(grid: &Vec<Vec<i32>>, row: usize, col: usize, num: i32) -> bool {
    !used_in_row(grid, row, num) && !used_in_col(grid, col, num)
        && !used_in_box(grid, row - row % 3, col - col % 3, num)
}

/**
 * Checks if a number is used in a specific row.
 * @param grid The Sudoku grid.
 * @param row The row index to check.
 * @param num The number to check.
 * @return True if the number is used in the row, false otherwise.
 */
fn used_in_row(grid: &Vec<Vec<i32>>, row: usize, num: i32) -> bool {
    grid[row].contains(&num)
}

/**
 * Checks if a number is used in a specific column.
 * @param grid The Sudoku grid.
 * @param col The column index to check.
 * @param num The number to check.
 * @return True if the number is used in the column, false otherwise.
 */
fn used_in_col(grid: &Vec<Vec<i32>>, col: usize, num: i32) -> bool {
    for row in grid {
        if row[col] == num {
            return true;
        }
    }
    false
}

/**
 * Checks if a number is used in a specific sub-box (3x3 grid).
 * @param grid The Sudoku grid.
 * @param row The starting row index of the sub-box.
 * @param col The starting column index of the sub-box.
 * @param num The number to check.
 * @return True if the number is used in the sub-box, false otherwise.
 */
fn used_in_box(grid: &Vec<Vec<i32>>, row: usize, col: usize, num: i32) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if grid[i + row][j + col] == num {
                return true;
            }
        }
    }
    false
}

/**
 * Removes cells from the Sudoku grid to achieve the specified difficulty level.
 * Modifies the grid by removing cells until it meets the desired difficulty
 * level.
 * @param grid The Sudoku grid to modify.
 * @param difficulty The desired difficulty level (number of filled cells).
 * @return The modified Sudoku grid.
 */
fn remove_cells(grid: &mut Vec<Vec<i32>>, difficulty: i32) -> Vec<Vec<i32>> {
    let mut cells = 81;
    let mut old_cells = cells + 1;
    let mut rng = rand::thread_rng();
    while cells < old_cells || cells > difficulty {
        for _ in 0..100 {
            let row = rng.gen_range(0..=8);
            let col = rng.gen_range(0..=8);
            if grid[row][col] != 0 {
                let backup = grid[row][col];
                grid[row][col] = 0;

                let mut count = 0;
                let temp_grid = grid.clone();
                solve_count(&temp_grid, 0, 0, &mut count);

                if count != 1 {
                    grid[row][col] = backup;
                } else {
                    cells -= 1;
                }
            }
        }
        old_cells = cells;
    }
    grid.clone()
}

/**
 * Counts the number of solutions for a Sudoku grid.
 *
 * This function uses backtracking to solve the Sudoku grid and
 * increments the 'count' parameter for each valid solution found.
 *
 * @param grid The Sudoku grid represented as a 2D vector
 * @param row The current row being processed in the grid
 * @param col The current column being processed in the grid
 * @param count The count of valid solutions found in the grid (modified by
 * reference)
 */
fn solve_count(grid: &Vec<Vec<i32>>, row: usize, col: usize, count: &mut i32) {
    if row == 8 && col == 9 {
        *count += 1;
        return;
    }

    let (mut row, mut col) = (row, col);
    if col == 9 {
        row += 1;
        col = 0;
    }

    if grid[row][col] == 0 {
        for num in 1..=9 {
            if is_safe(grid, row, col, num) {
                let mut new_grid = grid.clone();
                new_grid[row][col] = num;
                solve_count(&new_grid, row, col + 1, count);
            }
        }
    } else {
        solve_count(grid, row, col + 1, count);
    }
}

/**
 * Prints the Sudoku grid to stdout
 *
 * @param grid The 2D vector representing the Sudoku grid
 *             where each inner vector represents a row
 *             and each element within the row represents a number in the Sudoku
 * grid.
 */
fn print_sudoku(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for &num in row {
            print!("{} ", num);
        }
        println!();
    }
}

