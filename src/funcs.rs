//Модуль консоли
pub mod consl {
    //Пауза консоли
    pub fn pause() {
        println!("Нажмите, чтобы продолжить... ");

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    }
}

pub mod password {
    use crate::iomod::input::read_u8;
    use rand::Rng;

    //Массив символов
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";

    //Функция меню
    pub fn menu() -> u8 {
        println!("Выберите действие:\n(1) Сгенирировать пароль\n(2) Сохранить пароль\n(3) Показать пароль\n(4) Сменить логин\n(5) Сменить пароль\n(6) Удалить пароль\n(0) Выход");
        let action: u8 = read_u8();

        action
    }

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
