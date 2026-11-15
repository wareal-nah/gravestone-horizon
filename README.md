# Gravestone (Horizon)

[English](#english) | [Русский](#русский)

---

## English

**Overview:** A native Rust implementation of the classic Gravestone mechanic for the Horizon engine and mod loader. When a player dies, a gravestone block spawns at their death coordinates to safely store all items and equipment.

### Features
- **Safe Loot Storage:** Automatically places a gravestone (`gravestone:gravestone`) at the death location, preserving the entire inventory, armor, offhand, and hotbar slots.
- **One-Click Restoration:** Break the gravestone or interact with it (`Shift + Right-Click`) to restore items directly into their original slots.
- **Interactive Obituary Book:** Spawns an obituary item (`gravestone:obituary`) in the player's inventory that opens a custom book GUI displaying exact death coordinates, dimension, timestamp, and a list of lost items.
- **Bilingual Localization:** Full native support for both English (`en_us`) and Russian (`ru_ru`).

### Repository Structure
- `assets/` — Content pack source (`gravestone.hmod`): 3D block/item models, textures, and localizations.
- `core/` — Native Rust module source (`gravestone_core.hnmod`): death event handling, inventory restoration, and custom book GUI.

### Installation
Requires **both** files to work properly:
1. Place `gravestone.hmod` into `horizon-mods/`
2. Place `gravestone_core.hnmod` into `horizon-native-mods/`

### Credits & License
- **wareal-nah** — Rust code, custom GUI, and Horizon implementation (MIT License)
- **PneumonoIsNotAvailable** — Gravestone 3D model and block/item textures from [Gravestones](https://github.com/PneumonoIsNotAvailable/Gravestones) (MIT License)

---

## Русский

**Описание:** Нативная реализация мода на надгробия на языке Rust для движка и загрузчика модов Horizon. При смерти игрока на месте гибели появляется надгробие, которое безопасно сохраняет весь инвентарь и экипировку.

### Возможности
- **Сохранность вещей:** Автоматически устанавливает надгробие (`gravestone:gravestone`) на координатах смерти, сохраняя весь инвентарь, броню, вторую руку и хотбар.
- **Удобный возврат лута:** Сломайте надгробие или нажмите по нему `Shift + ПКМ`, чтобы вернуть все вещи строго в исходные слоты.
- **Интерактивный некролог:** Выдаёт игроку предмет некролога (`gravestone:obituary`), который открывает кастомный книжный интерфейс с координатами смерти, измерением, временем и списком вещей.
- **Локализация:** Полная поддержка английского (`en_us`) и русского (`ru_ru`) языков.

### Структура репозитория
- `assets/` — исходники контентного пака (`gravestone.hmod`): 3D-модели блока и предметов, текстуры и файлы локализации.
- `core/` — исходники нативного модуля на Rust (`gravestone_core.hnmod`): логика смерти, восстановление слотов и интерфейс некролога.

### Установка
Для работы мода необходимы **оба** файла:
1. Поместите `gravestone.hmod` в папку `horizon-mods/`
2. Поместите `gravestone_core.hnmod` в папку `horizon-native-mods/`

### Авторы и лицензия
- **wareal-nah** — код на Rust, интерфейс и реализация под Horizon (лицензия MIT)
- **PneumonoIsNotAvailable** — 3D-модель и текстуры надгробия из проекта [Gravestones](https://github.com/PneumonoIsNotAvailable/Gravestones) (лицензия MIT)
