//Модуль базы данных
pub mod db {
    extern crate rusqlite;
    use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    struct Data {
        name: String,
        login: String,
        password: String,
    }

    /*
    impl Data {
        pub fn decrypttt(&self) -> String {
            let mcrypt = new_magic_crypt!("magickey", 256);
            let decrypt_string = mcrypt.decrypt_base64_to_string(&self.password).unwrap();

            decrypt_string
        }
    }
    */

    pub fn save(name: String, login: String, password: String) {
        // Подключение к базе данных
        let conn = Connection::open("Keys.db").unwrap();

        // Создание таблицы
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Passwords (
                name    TEXT NOT NULL,
                login   TEXT NOT NULL,
                password    TEXT NOT NULL
                )",
            (),
        )
        .unwrap();

        // Запись данных в структуру Data
        let user = Data {
            name: name,
            login: login,
            password: password,
        };

        // Ввод данных в таблицу
        conn.execute(
            "INSERT INTO Passwords (name, login, password) VALUES (?1, ?2, ?3)",
            (&user.name, &user.login, &user.password),
        )
        .unwrap();
    }

    pub fn print_all() {
        impl Result<_, Error> {
            pub fn decrypttt(&self) -> String {
                let mcrypt = new_magic_crypt!("magickey", 256);
                let decrypt_string = mcrypt.decrypt_base64_to_string(&self.password).unwrap();

                decrypt_string
            }
        }
        // Подключение к базе данных
        let conn = Connection::open("Keys.db").unwrap();

        let mut stmt = conn.prepare("SELECT * FROM Passwords").unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(Data {
                    name: row.get(0).unwrap(),
                    login: row.get(1).unwrap(),
                    password: row.get(2).decrypttt(),
                })
            })
            .unwrap();

        for person in person_iter {
            println!("Найдены данные: {:?}", person.unwrap());
        }
    }

    pub fn print_certain(name: &String) -> Result<()> {
        // Подключение к базе данных
        let conn = Connection::open("Keys.db")?;

        let mut a = String::from("SELECT * FROM Passwords WHERE name = ");
        a.push_str(name);
        let mut stmt = conn.prepare(&a)?;
        let mut row = stmt.query_row((), |row| row.get(0))?;
        let result = row.get(0); // получаем результат
        println!("{}", result); // выводим результат
        Ok(())

        /*
        let mut stmt = conn
            .prepare("SELECT * FROM Passwords WHERE name = ?")
            .unwrap();
        let mut stmt = conn.prepare(a.as_str()).unwrap();
        let person_iter = stmt
            .query_map([name], |row| {
                Ok(Data {
                    name: row.get(0).unwrap(),
                    login: row.get(1).unwrap(),
                    password: row.get(2).unwrap(),
                })
            })
            .unwrap();

        for person in person_iter {
            println!("Найдены данные: {:?}", person.unwrap());
        }
        */

        //Ok(())
    }
}
