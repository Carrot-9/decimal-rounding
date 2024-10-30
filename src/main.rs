use rand::prelude::*;

fn _get_random_number() -> i32{
   let r = random::<i32>();
   return r;
}

fn _equation(r: i32) {
    if r == 0 || r == 1 {
        println!("This number cannot be split, the result is {}", r);
    }
    else if r % 2 == 0 {
        let a1 = r/ 2;
        let a2 = &a1;
        println!("The equation is {} + {} = {}",a1, a2, r);
    }
    else if r % 2 != 0 {
        let divide_r = r as f32 / 2.0;
        let r_string = divide_r.to_string();
        // Contains the Option<usize> of '.'
        // when '.' is removed, the usize will point to the first decimal number instead
        let find_dot = r_string.find('.').unwrap();
        let mut v1: Vec<char> = r_string.chars().collect();
        v1.remove(find_dot);

        let v2 = v1.clone();

        //Convert Vec<char> to Vec<i32>
    }
}

fn main() {

}
