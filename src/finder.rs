pub mod file {
    extern crate walkdir;

    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};

    // Создание, запись и сохрание
    pub fn save_file(name: String, login: String, password: String) -> std::io::Result<()> {
        // Добавление ".txt" для того чтобы создать файл
        let mut b = format!("{}.txt", name.trim());
        let mut file = File::create(b)?;

        // Запись данных в файл
        b = format!("Name: {}\nLogin: {}\nPassword: {}", name, login, password);
        file.write_all(b.as_bytes())?;

        Ok(())
    }

    // Чтение
    pub fn read_file(name: String) -> std::io::Result<()> {
        // Добавление ".txt" для того чтобы создать файл
        let name_for_open = format!("{}.txt", name.trim());

        // Откртие файла
        let mut file = File::open(name_for_open)?;

        // Чтение
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(())
    }

    // Вывод названия всех файлов
    pub fn print_all_files() -> std::io::Result<()> {
        // Путь до папки
        //let folder_path = "C:\\Program Files";


        Ok(())
    }
}
