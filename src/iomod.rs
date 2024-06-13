//Модуль ввода
pub mod input {
    use std::io;

    // Ввод строки
    pub fn input_line() -> String {
        let mut string = String::new();
        io::stdin()
            .read_line(&mut string)
            .expect("Ошибка! Не удалось ввести строку!");

        string
    }

    // Перевод переменой типа String в тип u8
    pub fn to_u8(string: &String) -> u8 {
        let y:u8;
        if string == "exit"{
            y = 0;
        } else{
            y = string.trim().parse().expect("Input not an integer");
        }
        
        y
    }

    // Ввод числа типа u8
    pub fn read_u8() -> u8 {
        let string = input_line();
        let x: u8 = to_u8(&string);

        x
    }

    // Перевод переменной типа String в тип i8
    pub fn to_i8(string: &String) -> i8 {
        let y = string.trim().parse().expect("Input not an integer");

        y
    }

    // Ввод числа типа i8
    pub fn read_i8() -> i8 {
        let string = input_line();
        let x: i8 = to_i8(&string);

        x
    }
}
