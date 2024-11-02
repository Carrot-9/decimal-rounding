// Takes a random number, splits it, and turns it into an equation
// If number is odd, decimals are eliminated and the first addend is rounded down while the second is rounded up

// PLaceholder number
fn get_number() -> i32  {
    let r = 9876235;
    return r;
}

fn main() {

    let r = get_number();

    if r == 0 || r < 0 {
        println!("This number cannot be split, the result is {}.", r);
    }
    else if r % 2 == 0 {
        let (a1,a2) = (r/ 2, r/2);
        println!("The equation is {} + {} = {}.",a1, a2, r);
    }
    else if r % 2 != 0 {
        let divide_r: f32 = r as f32 / 2.0;
        let r_string: String = divide_r.to_string();

        if let Some(find_dot) = r_string.find('.') {

        let mut v: Vec<char> = r_string.chars().collect();

        // change find_dot to index for better readability 
        // Also allows us to manipulate the position given to us by find_dot without actually changing its value
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

        let r_len = r.to_string().len();

        // Moves backwards until the first decimal number is reached
        // Uses find_dot as the value still points to the first decimal number, unlike index
        while index >= find_dot {
        
            while v2_i32[index] <= 9 {
                v2_i32[index] += 1;
            }
            // Pushes index backwards in the vector
            index -= 1;

            // index + 1 so that if index = 0 after the last decrement, the decimal number can still be checked
            if v2_i32[index + 1] >= 10 {
                // Sets decimal number to 0 and adds 1 to the first whole number to simulate rounding
                v2_i32[index + 1] = 0;
                // Handles edge cases in which the length of r > 1 and 9 is the last whole number
                // find_dot - 1 looks at the last whole number
                if v2_i32[find_dot -1] == 9 && r_len > 1   {
                    // Set every whole number other than the first to 0
                    for i in 1..r_len {
                        v2_i32[i] = 0;
                    }

                    // Round up by adding 1 to first whole number
                    v2_i32[0] += 1;
                }
                else {
                // If r is a single digit simply round up as usual
                v2_i32[find_dot - 1] += 1;
                }
            }
        }
        // Get totals 
        let v_total: String = v_i32.iter().map(|s| s.to_string()).collect();
        let v2_total: String = v2_i32.iter().map(|l|l.to_string()).collect();

        // Remove trailing 0's
        let addend1 = v_total.strip_suffix("0").unwrap();
        let addend2 = v2_total.strip_suffix("0").unwrap();

        // Print Equation 
        println!("The equation is {} + {} = {}.", addend1, addend2, r);
    }
    else {
        println!("Decimal Not Found, Equation Could Not Be Printed.");
        }
    }
}



