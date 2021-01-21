pub struct FizzBuzz {

}

impl FizzBuzz {
    pub fn say_it(input: u32) -> String {
        if input % 3 == 0 && input % 7 == 0 {
            return String::from("FizzWhizz");
        }
        if input % 3 == 0 && input % 5 == 0 {
            return String::from("FizzBuzz");
        }
        if input % 3 == 0 {
            return String::from("Fizz");
        }
        if input % 5 == 0 {
            return String::from("Buzz");
        }
        if input % 7 ==0{
            return String::from("Whizz");
        }
        return input.to_string();
    }
}


pub fn say_it_external(input: u32) -> String {
    return input.to_string();
}
