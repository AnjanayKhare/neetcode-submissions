use std::collections::HashSet;
fn is_valid_row(arr: &Vec<Vec<char>>, i:usize) -> bool{
    let mut set = HashSet::new();
    for j in 0..9{
        if arr[i][j] == '.'{
            continue;
        }
        if let Some(_) = set.get(&arr[i][j]){
            println!("failed at row {} {}", i, j);
            return false;
        }
        set.insert(arr[i][j]);
    }

    true
}
fn is_valid_col(arr: &Vec<Vec<char>>, j:usize) -> bool{
    let mut set = HashSet::new();
    for i in 0..9{
        if arr[i][j] == '.'{
            continue;
        }if let Some(_) = set.get(&arr[i][j]){
            println!("failed at col {} {}", i, j);
            return false;
        }
        set.insert(arr[i][j]);
    }

    true
}
fn is_valid_sec(arr: &Vec<Vec<char>>, a:usize, b:usize) -> bool{
    let mut set = HashSet::new();
    for i in (a*3)..(a*3 + 3){
            for j in (b*3)..(b*3 + 3){
                if arr[i][j] == '.'{
                continue;
            }if let Some(_) = set.get(&arr[i][j]){
                println!("failed at sec {} {}", i, j);
                return false;
            }
            set.insert(arr[i][j]);
        }
    }

    true
}


impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

        for i in 0..9{
            if !is_valid_row(&board, i) || !is_valid_col(&board, i){
                return false;
            }
        }

        for i in 0..3{
            for j in 0..3{
                if !is_valid_sec(&board, i, j){
                    return false;
                }
            }
        }

        true
    }
}
