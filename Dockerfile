# Этап 1: Использование cargo-chef для кэширования зависимостей
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Этап 2: Планирование сборки
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Этап 3: Сборка зависимостей и приложения
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Сборка зависимостей (кэширование)
RUN cargo chef cook --release --recipe-path recipe.json
# Сборка самого приложения
COPY . .
RUN cargo build --release --bin orchestrator

# Этап 4: Финальный runtime-образ
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Установка libssl3 и необходимых зависимостей
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Копирование бинарника из этапа сборки
COPY --from=builder /app/target/release/orchestrator /app/orchestrator

# Открытие порта
EXPOSE 8080

# Установка команды по умолчанию
CMD ["./orchestrator"]