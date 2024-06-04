//Модуль базы данных
pub mod db {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    pub struct Data {
        pub name: String,
        pub login: String,
        pub password: String,
    }

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
        // Подключение к базе данных
        let conn = Connection::open("Keys.db").unwrap();

        let mut stmt = conn
            .prepare("SELECT name, login, password FROM Passwords")
            .unwrap();
        let person_iter = stmt
            .query_map([], |row| {
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
    }

    pub fn print_certain(name: &String) {
        // Подключение к базе данных
        let conn = Connection::open("Keys.db").unwrap();

        let mut stmt = conn
            .prepare("SELECT * FROM Passwords WHERE name = ?")
            .unwrap();
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
    }
}
