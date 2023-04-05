//! Это документация (корневого) модуля.
//! Эти строчки пользователь будет видеть на главной странице docs.rs/your_crate
//!
//! # Задание
//!
//! 1. Тесты в этом модуле не компилируются. Исправьте их, добавляя объявления модулей (`mod`),
//!    импорты (`use`) и модификаторы видимости (`pub`). Не изменяйте сам модуль тестов (кроме блока импортов).
//!    Старайтесь не пользоваться подсказками IDE, а писать код самостоятельно.
//!    Используйте `cargo test --lib`, чтобы не компилировать `main.rs` до следующего задания.
//! 2. Бинарный таргет (`main.rs`) в этом крейте не компилируется. Исправьте ошибку, не трогая сам
//!    файл `main.rs`. Для этого вам нужно добавить объявление модуля `prelude`, который будет реэкспортировать
//!    нужные символы.
//! 3. Модуль `foo::bar` определен в отдельном файле `src/foo.rs` как внутренний модуль. Попробуйте
//!    вынести его в отдельный файл `src/foo/bar.rs`. Проверьте, что он доступен в `main.rs` и в тестах.
//! 4. Используйте команду `cargo doc --open` для сборки документации. Посмотрите, как комментарии в
//!    файлах трансформировались в строчки документации.
//!
//! # Подсказки
//!
//! - `cargo test --lib` для запуска тестов (без учета `main.rs`)
//! - `cargo doc --open` для сборки документации
//! - `cargo run` для запуска бинарного таргета `main.rs`
//! - Модули всегда нужно объявлять через ключевое слово `mod` в родительском модуле.
//! - Чтобы импортировать символ, он должен быть публичным (`pub`) и все подмодули на пути до него тоже
//!   должны быть публичными.

pub(crate) mod foo;

pub mod prelude {
    pub use crate::foo::bar::functon_from_bar;
    pub use crate::internal::public_api;
}

/// Эта функция сгенерирована Cargo автоматически при создании проекта. Нам пригодится.
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

/// Один из подмодулей нашего крейта. Он приватный, потому что содержит детали реализации.
///
/// Однако там же определена функция `public_api`, которую мы хотели бы использовать снаружи.
pub(crate) mod internal {
    /// Это часть публичного API нашей библиотеки.
    use crate::add;
    pub fn public_api() -> i32 {
        add(crate::foo::give_answer(), internal_function())
    }

    /// Мы хотим тестировать эту функцию, но не хотим давать к ней доступ извне нашего крейта.
    pub(crate) fn internal_function() -> i32 {
        println!("called `crate::internal::internal_function()`");
        3
    }
}

/// Ниже располагаются наши юнит-тесты.
#[cfg(test)]
mod tests {
    use super::*;

    /// Этот тест сгенерирован Cargo автоматически при создании проекта.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    /// Этот тест проверяет, что функция `give_answer` из модуля `foo` корректно импортирована.
    #[test]
    fn function_from_module_foo_is_accessible() {
        assert_eq!(foo::give_answer(), 42);
    }

    /// Этот тест проверяет, что функция `internal_function` из модуля `internal` доступна в юнит-тестах.
    #[test]
    fn internal_function_is_accessible() {
        assert_eq!(super::internal::internal_function(), 3);
    }

    /// Этот тест проверяет, что функция `functon_from_bar` из модуля `foo::bar` доступна в юнит-тестах.
    #[test]
    fn functon_from_bar_is_accessible() {
        assert_eq!(foo::bar::functon_from_bar(), 10);
    }
}
