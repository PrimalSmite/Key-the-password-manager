//Модуль ввода
pub mod input {
    use std::io;

    pub fn input_line() -> String {
        let mut string = String::new();
        io::stdin().read_line(&mut string);

        string
    }

    pub fn to_u8(string: &String) -> u8 {
        let y = string.trim().parse().expect("Input not an integer");

        y
    }

    pub fn read_int() -> u8 {
        let string = input_line();
        let x: u8 = to_u8(&string);

        x
    }
}
