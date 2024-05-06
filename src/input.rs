pub mod input {
    use std::io;

    //Функция создает переменную из типа String в тип u8
    pub fn transform(string: &String) -> u8 {
        let number: u8 = string.trim().parse().expect(
            "Не удалось распознать число. Пожалуйста, убедитесь, что вы ввели число от 0 до 255",
        );

        number
    }

    //Функция вводит данные с клавиатуры и записывает в переменню типа String
    fn stdinput(mut string: &mut String) -> String {
        //Ввод значения с клавитуры в переменную String
        io::stdin()
            .read_line(&mut string)
            .expect("Не удалось ввести значение");
        let result = String::new();

        result
    }

    //Функция вводит данные с клавиатуры и записывает их в тип u8
    pub fn int_input(mut string: &mut String) -> u8 {
        *string = stdinput(&mut string);
        let number: u8 = transform(&string);

        //Возвращает number:u8
        number
    }
}
