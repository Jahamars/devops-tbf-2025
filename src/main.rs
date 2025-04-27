fn main() {
// Чтение количества резервуаров
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let n: usize = input.trim().parse().unwrap();

// Чтение начальных объемов
input.clear();
std::io::stdin().read_line(&mut input).unwrap();
let volumes: Vec<i32> = input
    .trim()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

match min_operations_to_equalize(&volumes) {
    Some(operations) => println!("{}", operations),
    None => println!("-1"),
}
}

fn min_operations_to_equalize(volumes: &[i32]) -> Option<i32> {
let n = volumes.len();
let max_volume = *volumes.iter().max().unwrap();

// Рассчитываем необходимые добавления для каждого резервуара
let mut additions = vec![0; n];
for i in 0..n {
    additions[i] = max_volume - volumes[i];
}

// Проверяем возможность выравнивания
for i in 1..n {
    if additions[i-1] < additions[i] {
        // Если для предыдущего резервуара нужно добавить меньше,
        // чем для следующего, то выравнивание невозможно
        return None;
    }
}

// Считаем минимальное количество операций
let mut operations = 0;
for i in 0..n {
    // Добавляем разницу между текущим добавлением и следующим (если есть)
    let next_addition = if i < n-1 { additions[i+1] } else { 0 };
    operations += additions[i] - next_addition;
}

Some(operations)
}
