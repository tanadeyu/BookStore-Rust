# BookStore-Rust Project Notes

## Toolchain

**Default:** GNU (x86_64-pc-windows-gnu)
- Git Bash でビルド可能
- メインプロジェクトの開発に使用

## Project Structure

- `Cargo.toml` at project root (not in subdirectory)
- `src/` for source code
- `Steps/` for detailed procedure documents
- `docs/` for documentation
- `templates/` for HTML templates (Askama)

## Edition

Must be `2021` (not 2024)

## Dependencies

Actix Web 4.9, sqlx 0.8, Askama 0.13, SQLite, Tokio 1.x, serde 1.0

## Development Notes

- Use sqlx for database operations (compile-time checked queries)
- Use Askama for HTML templating (type-safe)
- GNU toolchain recommended for Git Bash development
- Database file: `bookstore.db` (created automatically if missing)
- Sample data inserted on first run (7 sales, 5 customers, 10 products, 5 categories)
