//Модуль базы данных
pub mod db {
    use rusqlite::{Connection, Result};

    #[derive(Debug)]
    pub struct Data {
        pub name: String,
        pub login: String,
        pub password: String,
    }

    pub fn save(name: String, login: String, password: String) -> Result<()> {
        // Подключение к базе данных
        let conn = Connection::open("Keys.db")?;

        // Создание таблицы
        conn.execute(
            "CREATE TABLE Passwords (
                id  INTEGER PRIMARY KEY,
                name    TEXT NOT NULL,
                login   TEXT NOT NULL,
                password    TEXT NOT NULL
                )",
            (),
        )?;

        // Запись данных в структуру Data
        let user = Data {
            name: name,
            login: login,
            password: password,
        };

        // Ввод данных в таблицу
        conn.execute(
            "INSERT INTO Passwords (name, login, password) VALUES (?1, ?2)",
            (&user.name, &user.login, &user.password),
        )?;

        Ok(())
    }

    pub fn print_all() -> Result<()> {
        // Подключение к базе данных
        let conn = Connection::open("Keys.db")?;

        let mut stmt = conn.prepare("SELECT id, name, login, password FROM Passwords")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(Data {
                name: row.get(0)?,
                login: row.get(1)?,
                password: row.get(2)?,
            })
        })?;

        for person in person_iter {
            println!("{:?}", person.unwrap());
        }

        Ok(())
    }
}
