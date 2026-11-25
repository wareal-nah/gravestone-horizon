# Gravestone (Horizon Port)

[English](#english) | [Русский](#русский)

---

## English

**Gravestone** is a mod for the **Horizon** engine and mod loader. When a player dies, it spawns a secure gravestone containing all inventory items and equipment, and provides an interactive obituary upon respawn.

### All-in-One (.hmod) Architecture
With Horizon's unified mod format, **Gravestone** packages both content resources (3D models, textures, block/item registrations, translations) and native engine hooks (Rust mixins) into a **single unified `gravestone.hmod`** archive.

### Features
- **Drop Preservation**: Spawns a grave at the death location, protecting items from void, lava, and explosions.
- **Accurate Slot Restoration**: Restores exact equipment and inventory positions (armor slots, offhand, hotbar, main storage).
- **Item Retrieval**: Break the grave or use `Shift + Right-Click` to recover items.
- **Interactive Obituary (`gravestone:obituary`)**: Right-click to open an in-game book displaying coordinates, dimension, time of death, and stored items.
- **Bilingual**: Automatic language detection (English and Russian).

### Project Structure
- `assets/`: Content mod sources (models, textures, blockstates, tags, translations, and `horizon-mod.toml`).
- `core/`: Native Rust mod with Horizon mixins and hooks (`Cargo.toml`, `horizon-native.json`, `mixins/`, `src/`).
- `gravestone.hmod`: Unified all-in-one distribution archive containing content and native mixins.

### Installation
1. Place `gravestone.hmod` into your instance's `horizon-mods/` or `horizon-native-mods/` folder.
2. Launch the Horizon client.

### Credits & Authors
- **wareal-nah** — Rust logic, GUI, and Horizon engine port
- **PneumonoIsNotAvailable** — Gravestone 3D model and block/item textures (MIT License)

### License
Distributed under the [MIT License](LICENSE).

---

## Русский

**Gravestone** — модификация для игрового движка и загрузчика **Horizon**. При смерти игрока сохраняет все вещи и экипировку в надгробии на месте гибели и выдаёт интерактивный некролог после возрождения.

### Единый формат (.hmod)
Благодаря обновлённой системе модов Horizon, **Gravestone** объединяет контентную часть (3D-модели, текстуры, регистрацию блоков/предметов, переводы) и нативный движковый код (Rust-миксины) в **один общий файл `gravestone.hmod`**.

### Возможности
- **Защита вещей**: Размещает надгробие на координатах смерти, защищая лут от лавы, пустоты и взрывов.
- **Точный возврат**: Восстанавливает вещи строго в исходные слоты (броня, вторая рука, хотбар, основной инвентарь).
- **Сбор предметов**: Забирайте вещи через `Shift + ПКМ` по надгробию или его разрушением.
- **Интерактивный некролог (`gravestone:obituary`)**: ПКМ открывает экран книги с точными координатами, измерением, датой и списком вещей.
- **Двуязычность**: Автоматическое переключение языков (русский и английский).

### Структура проекта
- `assets/`: Исходные ресурсы контентного мода (модели, текстуры, теги, переводы и `horizon-mod.toml`).
- `core/`: Нативный Rust-мод с миксинами и хуками движка (`Cargo.toml`, `horizon-native.json`, `mixins/`, `src/`).
- `gravestone.hmod`: Единый архив мода «всё в одном» (контент + нативный код).

### Установка
1. Поместите `gravestone.hmod` в папку `horizon-mods/` или `horizon-native-mods/`.
2. Запустите Horizon.

### Авторы и благодарности
- **wareal-nah** — логика на Rust, интерфейс и порт под Horizon
- **PneumonoIsNotAvailable** — 3D-модель надгробия и текстуры блока/предмета (MIT License)

### Лицензия
Распространяется под свободной лицензией [MIT](LICENSE).
