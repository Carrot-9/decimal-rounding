use rand::prelude::*;

fn get_random_number() -> i32 {
   let r = random::<i32>();
    return r;
}

fn main() {

    let r: i32 = get_random_number();

    if r == 0 {
        println!("0 cannot be split, the result is {}.", r);
    }
    else if r % 2 == 0 {
        let (a1,a2) = (r/ 2, r/2);
        println!("The equation is {} + {} = {}.",a1, a2, r);
    }
    else if r % 2 != 0 {
        let divide_r: f32 = r as f32 / 2.0;
        let r_string: String = divide_r.to_string();
        println!("{:?}",r_string);
        let find_dot: usize = r_string.find('.').unwrap();
        let mut v: Vec<char> = r_string.chars().collect();
        // change find_dot to index for better readability 
        let mut index: usize = find_dot;
        // removes '.' from vector, leaving only numbers
        // NOTE: Now that the '.' is removed, 'index' points to the first decimal number in the vector
        v.remove(index);
        //Create new vector Vec<i32>
        // ('0' as i32) subtracts the unicode value of the characters, giving us the intended values
        let mut v_i32: Vec<i32> = v.iter().map(|i| (*i as i32) - ('0' as i32)).collect();
        // .len() starts at 1 while usize starts 0 so a -1 is needed
        let vi32_len = v_i32.len() - 1;
        // each decimal number will be decremented to 0 until the end of the vector is reached
        while index <= vi32_len {
            while v_i32[index] > 0 {
                v_i32[index] -= 1;
            }
            index += 1;
        }
        // subtract 1 from index so that it doesnt point outside the vector
        index -= 1;
        // Clone v_i32 to get second addend
        let mut v2_i32: Vec<i32> = v_i32.clone();

        // Moves backwards until the first decimal number is reached
        while index >= find_dot {
            // increments each decimal number to 10 
            while v2_i32[index] <= 9 {
                v2_i32[index] += 1;
            }
            // Subtracts 10 from the number so that its equal to 0 to simulate rounding
            v2_i32[index] -= 10;
            index -= 1;
        }
        // Once all Values are set to 10, increase the whole value by 1 to simulate rounding
        v2_i32[find_dot - 1] += 1;
        // Get totals 
        let v_total: String = v_i32.iter().map(|s| s.to_string()).collect();
        let v2_total: String = v2_i32.iter().map(|l|l.to_string()).collect();
        // Remove trailing 0's
        let addend1 = v_total.strip_suffix("0").unwrap();
        let addend2 = v2_total.strip_suffix("0").unwrap();
        // Print Equation 
        println!("The equation is {} + {} = {}.", addend1, addend2, r);
    }
}

