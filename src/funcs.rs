//Модуль консоли
pub mod consl {
    //Очистка консоли
    pub fn clean() {
        let term = console::Term::stdout();
        term.clear_screen().expect("Не удалось очистить консоль");
    }
}

pub mod password {
    use crate::iomod::input::input_line;
    use rand::Rng;

    //Массив символов
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

    /*
    //Функция меню
    pub fn menu() -> u8 {
        println!("Выберите действие:\n(1) Сгенирировать пароль\n(2) Сохранить пароль\n(3) Показать пароль\n(4) Сменить логин\n(5) Сменить пароль\n(6) Удалить пароль\n(0) Выход");

        let mut str_act = String::new();
        let action: u8;

        action = int_input(&mut str_act);

        action
    }
    */

    //Генерация пароля
    pub fn generate(length: i8) -> String {
        let mut rng = rand::thread_rng();

        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }
}
