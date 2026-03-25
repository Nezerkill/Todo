# 📝 todo-rs

Терминальный менеджер задач на Rust. Быстрый, надёжный, с удобным CLI интерфейсом.

## 🚀 Установка

### Установка через Pacman (Arch Linux)

```bash
cd ~/.config-dev/dotfiles/todo-rs
makepkg -si
```

После установки команда `todo` доступна отовсюду.

## 📖 Использование

### Добавить задачу

```bash
# Базовое использование
todo add "Купить молоко"

# С проектом и приоритетом
todo add "Сделать отчёт" --project work --priority high

# С дедлайном
todo add "Подготовить презентацию" --due 2026-04-01

# С тегами
todo add "Исправить баг" --tags bug,urgent --project dev
```

### Просмотр задач

```bash
# Все задачи
todo list

# Только ожидающие
todo list --pending

# Только выполненные
todo list --done

# По проекту
todo list --project work

# По приоритету
todo list --priority high

# Просроченные
todo list --overdue

# Комбинированные фильтры
todo list --project dev --priority high --pending
```

### Управление задачами

```bash
# Отметить выполненной (по ID или первым 8 символам)
todo done d30ec43d

# Удалить задачу
todo remove 436bf25c

# Показать статистику
todo stats
```

## 📋 Команды

| Команда | Описание |
|---------|----------|
| `add` | Добавить новую задачу |
| `list` | Показать список задач |
| `done` | Отметить задачу выполненной |
| `remove` | Удалить задачу |
| `stats` | Показать статистику |

## 🏷️ Опции задач

### Приоритеты

- `low` — низкий (🟢)
- `medium` — средний (🟡, по умолчанию)
- `high` — высокий (🔴)

### Статусы

- `pending` — ожидает выполнения (⬜)
- `done` — выполнено (✅)
- `deferred` — отложено (⏸️)

### Форматы даты

- `YYYY-MM-DD` — например, `2026-04-01`
- `DD.MM.YYYY` — например, `01.04.2026`
- `YYYY-MM-DD HH:MM` — например, `2026-04-01 15:30`

## 📁 Хранение данных

Задачи хранятся в JSON файле:
```
~/.local/share/todo-rs/tasks.json
```

## 🎯 Примеры

### Рабочий день
```bash
# Добавить задачи на день
todo add "Проверить почту" -p work --priority medium
todo add "Сделать код-ревью" -p work --priority high --due 2026-03-26
todo add "Обновить документацию" -p work --priority low

# Посмотреть задачи по работе
todo list -p work

# После выполнения
todo done <id>
```

### Покупки
```bash
todo add "Купить молоко" --tags дом,еда
todo add "Купить корм коту" --tags дом,питомцы
todo list --pending
```

### Статистика за неделю
```bash
todo stats
```

## 🔧 Сборка

```bash
# Debug сборка
cargo build

# Release сборка (оптимизированная)
cargo build --release

# Запустить без установки
cargo run -- list
```

## 📦 Зависимости

- [clap](https://crates.io/crates/clap) — парсинг CLI аргументов
- [serde](https://crates.io/crates/serde) + [serde_json](https://crates.io/crates/serde_json) — работа с JSON
- [chrono](https://crates.io/crates/chrono) — дата и время
- [comfy-table](https://crates.io/crates/comfy-table) — красивые таблицы
- [colored](https://crates.io/crates/colored) — цвета в терминале
- [uuid](https://crates.io/crates/uuid) — уникальные ID
- [dirs](https://crates.io/crates/dirs) — системные директории

## 🛠️ Планы

- [ ] Редактирование задач (`todo edit`)
- [ ] Подзадачи
- [ ] Заметки к задачам
- [ ] Повторяющиеся задачи
- [ ] Экспорт/импорт (JSON, CSV)
- [ ] Поиск по задачам
- [ ] TUI режим (с fzf)
- [ ] Синхронизация с облаком

## 📄 Лицензия

MIT
