// Takes a random number, splits it, and turns it into an equation
// If number is odd, decimals are eliminated and the first addend is rounded down while the second is rounded up

use rand::prelude::*;

fn get_random_number() -> i32  {
    let mut rng:  ThreadRng = thread_rng();
    let r = rng.gen_range(0..9999999);
    return r;
}

fn remove_dot(r: &i32) -> (Vec<char>, usize, usize, usize) {

    let r_len = r.to_string().len();
    let divide_r: f32 = *r as f32 / 2.0;

    // Converted to string so it can be later moved into a Vec<Char>
    let r_string: String = divide_r.to_string();

     // Create Vec<Char> which will later be converted back to i32
     let mut v: Vec<char> = r_string.chars().collect();

     let find_dot = r_string.find('.').unwrap();

     // change find_dot to index for better readability 
     // Also allows us to manipulate the position given to us by find_dot without actually changing its value
     let index: usize = find_dot;
 
    if find_dot != 0 {
        // removes '.' from vector, leaving only numbers
        // NOTE: Now that the '.' is removed, 'index' points to the first decimal number in the vector
        v.remove(index);
    }
    else {
        println!("Decimal Not Found.")
    }
    (v, index, find_dot, r_len)
}

fn create_equation(v: &Vec<char>, index: usize, find_dot: usize, r_len: usize) -> (Vec<i32>, Vec<i32>) {

        // index needs to be mutable
        let mut i = index;

     //Create new vector Vec<i32>
        // ('0' as i32) subtracts the unicode value of the characters, giving us the intended values
        let mut v_i32: Vec<i32> = v.iter().map(|i| (*i as i32) - ('0' as i32)).collect();

        // .len() starts at 1 while usize starts 0 so a -1 is needed
        let vi32_len = v_i32.len() - 1;

        // each decimal number will be decremented to 0 until the end of the vector is reached
        while i <= vi32_len {
            while v_i32[i] > 0 {
                v_i32[i] -= 1;
            }
            i += 1;
        }

        // subtract 1 from index so that it doesnt point outside the vector
        i -= 1;

        // Clone v_i32 to get second addend
        let mut v2_i32: Vec<i32> = v_i32.clone();

        // Moves backwards until the first decimal number is reached
        // Uses fd as the value still points to the first decimal number, unlike index
        while i >= find_dot {
        
            while v2_i32[i] <= 9 {
                v2_i32[i] += 1;
            }
            // Pushes index backwards in the vector
            i -= 1;

            // index + 1 so that if index = 0 after the last decrement, the decimal number can still be checked
            if v2_i32[i + 1] >= 10 {
                // Sets decimal number to 0 and adds 1 to the last whole number to simulate rounding
                v2_i32[i + 1] = 0;
                // Handles edge cases in which the length of r > 1 and 9 is the last whole number
                // fd - 1 looks at the last whole number
                if v2_i32[find_dot -1] == 9 && r_len > 1   {
                    // Set every whole number other than the first to 0
                    for n in 1..r_len {
                        v2_i32[n] = 0;
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
        (v_i32, v2_i32)
    
}

fn get_totals(r: &i32,v_i32: &Vec<i32>, v2_i32: &Vec<i32>) -> String {

    // Get totals 
    let v_total: String = v_i32.iter().map(|s| s.to_string()).collect();
    let v2_total: String = v2_i32.iter().map(|l|l.to_string()).collect();

    // Remove trailing 0's
    let addend1 = v_total.strip_suffix("0").unwrap();
    let addend2 = v2_total.strip_suffix("0").unwrap();

    // Print Equation 
   let equation = format!("The equation is {} + {} = {}.", addend1, addend2, r);

   return equation;
    }

fn main() {

    let r = get_random_number();

    if r == 0 || r < 0 {
        println!("This number cannot be split because it is negative, the result is {}.", r);
    }
    else if r % 2 == 0 {
        let (a1,a2) = (r/ 2, r/2);
        println!("The equation is {} + {} = {}.",a1, a2, r);
    }
    else if r % 2 != 0 {
    let (v, index, find_dot,r_len) = remove_dot(&r);
      let (v_i32, v2_i32) = create_equation(&v, index, find_dot, r_len);
      let result = get_totals(&r, &v_i32, &v2_i32);
      println!( "{:?}", result);
        }
    }




