impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let n = matrix.len();
        let m = matrix[0].len();

        let mut l = 0;
        let mut h = n*m - 1;

        while l <= h{
            let mid = (l+h) / 2;
            let i = mid / m;
            let j = mid % m;
            println!("{} {}", i, j);
            let curr = matrix[i][j];
            if curr == target{
                return true;
            }else if target < curr{
                if mid == 0{
                    return false;
                }
                h = mid - 1;
            }else{
                l = mid+1;
            }
        }

        false
    }
}
