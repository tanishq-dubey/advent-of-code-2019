fn main() {
    let start: i32 = 372304;
    let end: i32 = 847060;
//    let start: i32 = 111100;
//    let end: i32 = 123445;
    // Part 1
    let mut count = 0;
    for x in start..end {
        let s: String = x.to_string();

        // Check for doubles
        let mut double: bool = false;
        for y in 0..10 {
            let f_string = format!("{}{}", y, y);
            double = s.contains(&f_string);
            if double {
                break
            }
        }

        // strictly increasing
        let char_vec: Vec<char> = s.chars().collect();
        let mut increasing: bool = true;
        for y in 0..char_vec.len() - 1 {
            if char_vec[y] > char_vec[y + 1] {
                increasing = false;
                break
            }
        }
        if double && increasing {
            count = count + 1;
        }
    }
    println!("combos: {}\n\n", count);

    // part 2
    count = 0;
    for x in start..end {
        let s: String = x.to_string();

        // Check for doubles
        let mut double: bool = false;
        let char_vec: Vec<char> = s.chars().collect();
        let mut c_len:i32 = 1;
        for z in 0..char_vec.len() - 1  {
            if char_vec[z] == char_vec[z + 1] {
                c_len = c_len + 1;
                if z == (char_vec.len() - 2) && c_len == 2 {
                    double = true;
                }
            } else {
                if c_len == 2 {
                    double = true;
                }
                c_len = 1;
            }
        }

        // strictly increasing
        let mut increasing: bool = true;
        for y in 0..char_vec.len() - 1 {
            if char_vec[y] > char_vec[y + 1] {
                increasing = false;
                break
            }
        }
        if double && increasing {
            count = count + 1;
        }
    }

    println!("combos new: {}", count);
}
