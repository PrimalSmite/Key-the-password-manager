pub mod file {
    use std::fs;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::io::{BufRead, BufReader};

    pub const DIR_PATH: &str = "Passwords"; 
    const PASSWORDS: &str = "Passwords/pass.txt";

    // Создание, запись и сохрание
    pub fn save_file(name: &String, login: &String, password: &String) {
        // Добавление ".txt" для того чтобы создать файл
        let mut b = format!("{}/{}.txt",DIR_PATH, name.trim().to_lowercase());
        let mut file = File::create(b).unwrap();

        // Запись данных в файл
        b = format!("Name: {}\nLogin: {}\nPassword: {}", name, login, password);
        match file.write_all(b.as_bytes()){
            Ok(_) => println!("Файл успешно сохранен"),
            Err(err) => println!("Ошибка {}", err)
        }
    }

    // Чтение
    pub fn read_file(name: &String) {
        // Добавление ".txt" для того чтобы создать файл
        let name_for_open = format!("{}/{}.txt",DIR_PATH, name.trim().to_lowercase());

        // Откртие файла
        let mut file = File::open(name_for_open).unwrap();

        // Чтение
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        for line in content.lines(){
            if !line.trim().is_empty(){
                println!("{}", line);
            }
        }

    }

    // Вывод названия всех файлов
    pub fn print_all_files() {
        for entry in fs::read_dir(DIR_PATH).unwrap() {
            let dir = entry.unwrap();
            //let dir_path = dir.path().to_str().unwrap();

            if dir.path().to_str().unwrap().contains("Passwords/") {
                let mut res = dir.path().to_str().unwrap().replace("Passwords/", "");
                if res.contains(".txt") {
                    res = res.replace(".txt", "");
                    println!("{}", res);
                }
            }
        } 

    }

    // Удаление пароля
    pub fn rm_file(name: String){
        // Добавление .txt для обозначения пути файла
        let a = format!("{}{}.txt",DIR_PATH, name.trim().to_lowercase());
        match fs::remove_file(a){
            Ok(_) => println!("File deleted successful"),
            Err(err) =>println!("Ошибка: {}", err)
        };
    }

    // Смена логина
    pub fn change_login(name: &String, new: &String){
        // Временный файл
        let temp_path = "temp.txt";
        let mut temp_file = File::create(temp_path).expect("Unnable to create a temp file");

        // Добавление .txt для обозначения пути файла
        let path = format!("{}/{}.txt",DIR_PATH, name.trim().to_lowercase());
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
    }

    // Смена пароля
    pub fn change_password(name: String, new: String){
        // Временный файл
        let temp_path = "temp.txt";
        let mut temp_file = File::create(temp_path).expect("Unnable to create a temp file");

        // Добавление .txt для обозначения пути файла
        let path = format!("{}{}.txt",DIR_PATH, name.trim().to_lowercase());
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
    }

    // Проверка наличия файла с паролем
    pub fn file_exists() -> i8 {
        if fs::metadata("Passwords/pass.txt").is_ok() {
            1
        } else if fs::metadata("Passwords/none.txt").is_ok() {
            -1
        }else {
            0
        }
    }
}