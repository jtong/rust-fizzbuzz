pub struct FizzBuzz {}

impl FizzBuzz {
    pub fn say_it(input: u32) -> String {
        let result = FizzBuzz::divided_by357(input);
        if result != "" {
            return result;
        }
        return input.to_string();
    }

    fn divided_by357_new(input: u32) -> String {
        let mut result: String = String::from("");
        if input % 3 == 0{
            result.push_str("Fizz");
        }
        if input % 5 == 0{
            result.push_str("Buzz");
        }
        if input % 7 == 0 {
            result.push_str("Whizz");
        }
        return result;

    }
    fn divided_by357(input: u32) -> String {
        let mut result: String = String::from("");
        if input % 3 == 0 && input % 5 == 0 && input % 7 == 0 {
            result = String::from("FizzBuzzWhizz");
        } else if input % 5 == 0 && input % 7 == 0 {
            result = String::from("BuzzWhizz");
        } else if input % 3 == 0 && input % 7 == 0 {
            result = String::from("FizzWhizz");
        } else if input % 3 == 0 && input % 5 == 0 {
            result = String::from("FizzBuzz");
        } else if input % 3 == 0 {
            result = String::from("Fizz");
        } else if input % 5 == 0 {
            result = String::from("Buzz");
        } else if input % 7 == 0 {
            result = String::from("Whizz");
        }
        return result;
    }
}


pub fn say_it_external(input: u32) -> String {
    return input.to_string();
}
