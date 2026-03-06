# Rust async service (лабораторная)

Асинхронный сервис на Rust для варианта `power-consumption-of-cameras`.

Сервис:
- принимает задачу расчета на `POST /calculate-consumption`;
- сразу возвращает HTTP-ответ;
- выполняет расчет в фоне с задержкой;
- отправляет результат в основной Go-сервис на endpoint обновления.

## Зависимости

- Rust + Cargo
- Запущенный основной сервис из папки `power-consumption-of-cameras` (обычно `http://localhost:8080`)

## Настройка

1. Скопировать шаблон:

```bash
cp .env.example .env
```

2. При необходимости изменить значения в `.env`.

## Запуск

```bash
cargo run
```

По умолчанию сервис слушает `http://127.0.0.1:8001`.

## Тесты

```bash
cargo test
```

## Контракт межсервисного взаимодействия

### Вход (из Go-сервиса)

`POST /calculate-consumption`

```json
{
  "request_id": 1,
  "calculations": [
    { "id": 100, "power": 80.0 },
    { "id": 101, "power": 120.0 }
  ]
}
```

### Выход (callback в Go-сервис)

`POST {BACKEND_CALLBACK_URL}`

```json
{
  "request_id": 1,
  "calculations": [
    { "calculation_id": 100, "monthly_cost": 316.8 },
    { "calculation_id": 101, "monthly_cost": 475.2 }
  ],
  "token": "..."
}
```

Формула расчета:

`monthly_cost = power * 24 * 30 * tariff / 1000`
