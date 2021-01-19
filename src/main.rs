mod fizzbuzz;

#[cfg(test)]
mod test {
    use crate::fizzbuzz::FizzBuzz;
    use crate::fizzbuzz::say_it_external;

    #[test]
    fn should_return_1_given_input_is_1_try_external() {
        let result = say_it_external(1);

        assert_eq!("1", result);
    }

    #[test]
    fn should_return_string_of_number_given_input_is_normal_number() {
        let result = FizzBuzz::say_it(1);

        assert_eq!("1", result);
    }

    #[test]
    fn should_return_fizz_given_input_can_be_divided_by_3() {
        let result = FizzBuzz::say_it(6);

        assert_eq!("Fizz", result);
    }


}

fn main() {}