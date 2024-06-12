pub mod file {
    use std::fs;
    use std::fs::File;
    use std::io::{BufWriter, Read, Write};
    use std::io::{BufRead, BufReader};

    // Создание, запись и сохрание
    pub fn save_file(name: String, login: String, password: String) -> std::io::Result<()> {
        // Добавление ".txt" для того чтобы создать файл
        let mut b = format!("{}.txt", name.trim());
        let mut file = File::create(b)?;

        // Запись данных в файл
        b = format!("Name: {}\nLogin: {}\nPassword: {}", name, login, password);
        file.write_all(b.as_bytes())?;

        // Удаление лишних строк в файле
        delete_empty_lines(b);

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
        //let folder_path = ".";

        for entry in fs::read_dir(".")? {
            let dir = entry?;
            println!("{:?}", dir.path());
        } 

        Ok(())
    }

    // Удаление пароля
    pub fn rm_file(name: String){
        // Добавление .txt для обозначения пути файла
        let a = format!("{}.txt", name.trim());
        match fs::remove_file(a){
            Ok(_) => println!("File deleted successful"),
            Err(err) =>println!("Ошибка: {}", err)
        };
    }

    // Смена логина
    pub fn change_login(name: String, new: String){
        // Временный файл
        let temp_path = "temp.txt";
        let mut temp_file = File::create(temp_path).expect("Unnable to create a temp file");

        // Добавление .txt для обозначения пути файла
        let path = format!("{}.txt", name.trim());
        let file = File::open(path.clone()).expect("Unnable to open file");
        let reader = BufReader::new(&file); 

        // Старый и новый логины
        let new_login = format!("Login: {}", new);

        // Поиск строки
        for line in reader.lines(){
            let line = line.expect("Unnable to read line");

            if line.starts_with("Login: "){
                writeln!(temp_file, "{}", new_login).expect("Unable to write to temp file");
            } else{
                writeln!(temp_file,"{}",line).expect("Unable to write to temp file");
            }
        }

        // Смена имени временного файла
        fs::rename(temp_path, path.clone()).expect("Unable to rename temp file");
        // Удаление лишних строк в файле
        delete_empty_lines(path);
    }

    // Смена пароля
    pub fn change_password(name: String, new: String){
        // Временный файл
        let temp_path = "temp.txt";
        let mut temp_file = File::create(temp_path).expect("Unnable to create a temp file");

        // Добавление .txt для обозначения пути файла
        let path = format!("{}.txt", name.trim());
        let file = File::open(path.clone()).expect("Unnable to open file");
        let reader = BufReader::new(&file); 

        // Старый и новый логины
        let new_login = format!("Password: {}", new);

        // Поиск строки
        for line in reader.lines(){
            let line = line.expect("Unnable to read line");

            if line.starts_with("Password: "){
                writeln!(temp_file, "{}", new_login).expect("Unable to write to temp file");
            } else{
                writeln!(temp_file,"{}",line).expect("Unable to write to temp file");
            }
        }

        // Смена имени временного файла
        fs::rename(temp_path, path.clone()).expect("Unable to rename temp file");
        // Удаление путсых строк 
        delete_empty_lines(path);
    }

    // Удаление пустых строк 
    fn delete_empty_lines(name: String){
        // Пути до файлов
        let input_path = format!("{}.txt", name);
        let output_path = format!("{}_out.txt",name);
        // Файлы
        let input_file = File::open(input_path.clone()).expect("Unable to open file");
        let output_file = File::create(output_path).expect("Unable to open file");

        let reader = BufReader::new(input_file);
        let mut writer = BufWriter::new(output_file);

        for line in reader.lines(){
            let line = line.expect("Unable to read line");
            if !line.trim().is_empty(){
                writeln!(writer, "{}", line).expect("Unable to write line");
            }
        }

        fs::remove_file(input_path).expect("Cannot delete input file");
    }
}
