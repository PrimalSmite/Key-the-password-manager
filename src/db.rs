//Модуль базы данных
pub mod db {
    use postgres::{Client, NoTls};

    pub fn save() -> Result {
        let mut client = Client::connect("host=localhost user=postgres", NoTls)?;

        client.batch_execute(
            "
        CREATE TABLE IF NOT EXISTS passwords (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            login   TEXT NOT NULL,
            password TEXT NOT NULL
        )
        ",
        )?;
    }
}
