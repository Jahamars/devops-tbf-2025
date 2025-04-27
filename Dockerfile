# Используем базовый образ Rust для сборки
FROM rust:1.83.0 as builder

# Создаем рабочую директорию
WORKDIR /app

# Копируем файлы с зависимостями
COPY Cargo.toml Cargo.lock ./

# Создаем пустой исходный файл, чтобы собрать зависимости отдельно
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

# Копируем фактический исходный код
COPY src ./src

# Собираем приложение
RUN cargo build --release

# Используем минимальный образ для запуска
FROM debian:bullseye-slim

# Устанавливаем необходимые библиотеки
RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Копируем исполняемый файл из предыдущего этапа
COPY --from=builder /app/target/release/tbf2025 /usr/local/bin/

# Настраиваем точку входа
ENTRYPOINT ["tbf2025"]
