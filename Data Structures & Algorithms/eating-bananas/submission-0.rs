fn valid(piles : &Vec<i32>, h:i32, k:i32) -> bool{
    let mut time_taken = 0;

    for &i in piles{
        time_taken += i/k;
        if i%k > 0{
            time_taken += 1;
        }
    }
    // println!("{} {}", time_taken, k);

    time_taken <= h
}

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, m: i32) -> i32 {
        let mut l = 1;
        let &(mut h) = piles.iter().max().unwrap();
        let mut ans = h;
        while l <= h{
            let mid = (l+h) / 2;
            let p = valid(&piles, m, mid);
            // println!("{} {}", mid, p);
            if p{
                ans = ans.min(mid);
                h = mid -1;
            }else{
                l = mid + 1;
            }
        }

        ans

    }
}
