// Задача
// Отсортировать строки в файле по аналогии с консольной утилитой sort (man sort — смотрим описание и параметры): на входе подается основной файл с неотсортированными строками, на выходе — файл с отсортированными. Не требуется поддерживать сортировку больших файлов на диске, достаточно провести сортировку в памяти.

// Реализовать поддержку утилиты следующими ключами:
//
// -k — указание столбцов для сортировки (слова в строке могут отображаться в виде колонок, по умолчанию разделитель — пробел)
//
// -n — сортировать по числовому результату
//
// -r — сортировать в обратном порядке
//
// -u — не выводить повторяющиеся строки
//
// Дополнительно
// Реализовать поддержку утилиты следующими ключами:
//
// -M — сортировать по названию месяца
//
// -b — хвостовые пробелы
//
// -c — проверить отсортированные данные
//
// -h — сортировать по числовому результату с учетом суффиксов





use std::env;
use std::fs::File;
use std::io::{Read, Write};

// Функция для записи в файл
fn write_to_file<T: std::fmt::Display>(output: Vec<T>) {
    let mut output_file = File::create("output.txt").expect("Не удалось создать файл"); // Создаем выходной файл
    for item in output {
        write!(output_file, "{}\n", item).expect("Не удалось записать в файл"); // Записываем каждое значение в файл
    }
}

// Функция для проверки, отсортирован ли массив
fn is_sorted<T: Ord>(data: &[T]) -> bool {
    data.windows(2).all(|window| window[0] <= window[1]) // Проверяем, что все пары элементов отсортированы
}

// Функция для парсинга чисел с суффиксами (k, m, b, t)
fn parse_number_with_suffix(s: &str) -> f64 {
    let suffixes = vec!['k', 'm', 'b', 't']; // Определяем возможные суффиксы
    let mut number_str = s.to_lowercase(); // Приводим строку к нижнему регистру
    let mut multiplier = 1.0; // Инициализируем множитель

    if let Some(last_char) = number_str.chars().last() { // Если последний символ есть
        if suffixes.contains(&last_char) { // Проверяем, является ли он суффиксом
            number_str.pop(); // Убираем суффикс
            multiplier = match last_char { // Определяем множитель по суффиксу
                'k' => 1_000.0,
                'm' => 1_000_000.0,
                'b' => 1_000_000_000.0,
                't' => 1_000_000_000_000.0,
                _ => 1.0,
            };
        }
    }

    number_str.parse::<f64>().unwrap_or(0.0) * multiplier // Преобразуем строку в число и умножаем на множитель
}

// Функция для сортировки строк из файла
fn sort_string(file_name: &str, reverse: bool, unique: bool, ignore_trailing_spaces: bool, check_sorted: bool) {
    let mut read_file = File::open(file_name).expect("Не удалось найти файл"); // Открываем файл для чтения
    let mut contents = String::new();
    read_file.read_to_string(&mut contents).expect("Не удалось прочитать файл"); // Читаем содержимое файла

    let mut words: Vec<String> = contents
        .lines()
        .map(|line| if ignore_trailing_spaces { line.trim_end().to_string() } else { line.to_string() }) // Убираем хвостовые пробелы, если это необходимо
        .collect();

    // Проверка отсортированности данных
    if check_sorted {
        if is_sorted(&words) {
            println!("Данные уже отсортированы."); // Если данные отсортированы, выводим сообщение
            return; // Выходим из функции
        } else {
            println!("Данные не отсортированы."); // Если данные не отсортированы, выводим сообщение
        }
    }

    // Сортировка
    words.sort(); // Сортируем массив

    if reverse {
        words.reverse(); // Если указана обратная сортировка, реверсируем массив
    }

    if unique {
        words.dedup(); // Если указано уникальное значение, удаляем дубликаты
    }

    write_to_file(words); // Записываем отсортированный массив в файл
}

// Функция для сортировки чисел
fn sort_num(file_name: &str, unique: bool, ignore_trailing_spaces: bool, with_suffix: bool) {
    let mut read_file = File::open(file_name).expect("Не удалось найти файл"); // Открываем файл для чтения
    let mut contents = String::new();
    read_file.read_to_string(&mut contents).expect("Не удалось прочитать файл"); // Читаем содержимое файла

    let mut numbers: Vec<f64> = contents
        .lines()
        .map(|line| if ignore_trailing_spaces { line.trim_end().to_string() } else { line.to_string() }) // Убираем хвостовые пробелы, если это необходимо
        .map(|s| if with_suffix { parse_number_with_suffix(&s) } else { s.parse::<f64>().unwrap_or(0.0) }) // Парсим числа с учетом суффиксов
        .collect();

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)); // Сортируем числа

    if unique {
        numbers.dedup(); // Если указано уникальное значение, удаляем дубликаты
    }

    let formatted_numbers: Vec<String> = numbers.iter().map(|n| n.to_string()).collect(); // Преобразуем числа обратно в строки
    write_to_file(formatted_numbers); // Записываем отсортированные числа в файл
}

// Функция для сортировки по столбцам
fn sort_by_columns(file_name: &str, columns: Vec<usize>, reverse: bool, unique: bool, ignore_trailing_spaces: bool) {
    let mut read_file = File::open(file_name).expect("Не удалось найти файл"); // Открываем файл для чтения
    let mut contents = String::new();
    read_file.read_to_string(&mut contents).expect("Не удалось прочитать файл"); // Читаем содержимое файла

    let mut lines: Vec<&str> = contents.lines().collect(); // Получаем строки

    lines.sort_by(|a, b| {
        let a_columns: Vec<&str> = a.split_whitespace().collect(); // Разбиваем строку на столбцы
        let b_columns: Vec<&str> = b.split_whitespace().collect(); // Разбиваем строку на столбцы

        for &col in &columns { // Сравниваем по указанным столбцам
            let a_col = a_columns.get(col - 1).unwrap_or(&""); // Получаем значение из первого ряда
            let b_col = b_columns.get(col - 1).unwrap_or(&""); // Получаем значение из второго ряда

            let cmp = a_col.cmp(b_col); // Сравниваем значения
            if cmp != std::cmp::Ordering::Equal {
                return cmp; // Возвращаем результат сравнения
            }
        }

        std::cmp::Ordering::Equal // Если все сравнения равны, возвращаем равенство
    });

    if reverse {
        lines.reverse(); // Если указана обратная сортировка, реверсируем массив
    }

    if unique {
        lines.dedup(); // Если указано уникальное значение, удаляем дубликаты
    }

    write_to_file(lines); // Записываем отсортированные строки в файл
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Получаем аргументы командной строки
    if args.len() < 3 { // Проверяем количество аргументов
        eprintln!("Использование: {} <команда> <файл> [-k <колонки>] [-u] [-b] [-c] [-h]", args[0]); // Если недостаточно аргументов, выводим сообщение
        return;
    }

    let command = &args[1]; // Получаем команду
    let file_name = &args[2]; // Получаем имя файла

    let mut columns: Vec<usize> = Vec::new();
    let mut reverse = false; // Флаг обратной сортировки
    let mut unique = false; // Флаг уникальности
    let mut ignore_trailing_spaces = false; // Флаг игнорирования хвостовых пробелов
    let mut check_sorted = false; // Флаг проверки сортировки
    let mut with_suffix = false; // Флаг обработки суффиксов

    // Обработка дополнительных флагов
    if args.len() > 3 {
        let mut i = 3;
        while i < args.len() {
            match args[i].as_str() {
                "-k" => {
                    columns = args[i + 1]
                        .split(',')
                        .map(|s| s.parse::<usize>().expect("Ошибка: колонки должны быть числами")) // Считываем указанные колонки
                        .collect();
                    i += 2;
                }
                "-u" => {
                    unique = true; // Устанавливаем флаг уникальности
                    i += 1;
                }
                "-b" => {
                    ignore_trailing_spaces = true; // Устанавливаем флаг игнорирования пробелов
                    i += 1;
                }
                "-c" => {
                    check_sorted = true; // Устанавливаем флаг проверки сортировки
                    i += 1;
                }
                "-h" => {
                    with_suffix = true; // Устанавливаем флаг обработки суффиксов
                    i += 1;
                }
                _ => {
                    eprintln!("Неправильный аргумент: {}", args[i]); // Обрабатываем неправильные аргументы
                    return;
                }
            }
        }
    }

    // Вызов соответствующей функции в зависимости от команды
    match command.as_str() {
        "-s" => {
            if !columns.is_empty() {
                sort_by_columns(file_name, columns, reverse, unique, ignore_trailing_spaces); // Сортировка по колонкам
            } else {
                sort_string(file_name, reverse, unique, ignore_trailing_spaces, check_sorted); // Сортировка строк
            }
        }
        "-n" => sort_num(file_name, unique, ignore_trailing_spaces, with_suffix), // Сортировка чисел
        "-rs" => {
            reverse = true; // Устанавливаем флаг обратной сортировки
            if !columns.is_empty() {
                sort_by_columns(file_name, columns, reverse, unique, ignore_trailing_spaces); // Сортировка по колонкам
            } else {
                sort_string(file_name, reverse, unique, ignore_trailing_spaces, check_sorted); // Сортировка строк
            }
        }
        _ => {
            println!("Неправильная команда. Используйте -s для сортировки строк, -n для сортировки чисел, -rs для обратной сортировки строк, -k для сортировки по колонкам, -u для удаления дубликатов, -b для игнорирования хвостовых пробелов, -c для проверки сортировки, -h для сортировки по числовому результату с учетом суффиксов."); // Сообщение о неправильной команде
        }
    }
}
