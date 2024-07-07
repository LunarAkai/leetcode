fn main() {
    println!("{}", num_water_bottles(9, 3));
    println!("{}", num_water_bottles(15, 4));
}

fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
    let mut num_empty: i32 = 0;
    let mut num_total: i32 = num_bottles;
    let mut num_b: i32 = num_bottles;

    while num_b + num_empty >= num_exchange {
        num_empty += num_b;
        num_b = 0;

        num_b = num_empty / num_exchange;

        if num_empty % num_exchange == 0 {
            num_empty = 0;
        } else {
            num_empty = num_empty % num_exchange;
        }
        num_total += num_b;
    }   
    num_total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let test_num_bottles = 9;
        let test_num_exchange = 3;
        assert_eq!(num_water_bottles(test_num_bottles, test_num_exchange), 13);
    }

    #[test]
    fn test_two() {
        let test_num_bottles = 15;
        let test_num_exchange = 4;
        assert_eq!(num_water_bottles(test_num_bottles, test_num_exchange), 19);
    }
}
