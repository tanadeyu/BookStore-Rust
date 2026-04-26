# Step 4: Rustコードでデータベース操作

**目的**: sqlxを使ってデータベースにデータを挿入・取得する

---

## 手順

### 1. モデル（struct）の定義

**`src/models.rs` を作成:**

```rust
use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, Serialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub stock: i32,
    pub published_date: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub member_number: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SaleHeader {
    pub id: i64,
    pub customer_id: i64,
    pub sale_date: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Sale {
    pub id: i64,
    pub sale_header_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub sale_price: i32,
}
```

### 2. main.rsを更新

**`src/main.rs` を書き換え:**

```rust
mod models;

use sqlx::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://bookstore.db".to_string());

    let pool = SqlitePool::connect(&database_url).await?;

    // カテゴリを挿入
    sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind("小説")
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind("ビジネス書")
        .execute(&pool)
        .await?;

    // カテゴリを取得
    let categories = sqlx::query_as::<_, models::Category>(
        "SELECT id, name FROM categories"
    )
    .fetch_all(&pool)
    .await?;

    println!("カテゴリ一覧:");
    for category in categories {
        println!("  - {}: {}", category.id, category.name);
    }

    Ok(())
}
```

### 3. 実行

```bash
cargo run
```

**期待される出力:**
```
カテゴリ一覧:
  - 1: 小説
  - 2: ビジネス書
```

---

## 確認

**プロジェクト構造:**
```
BookStore-Rust/
├── src/
│   ├── main.rs        ✅ データベース操作コード
│   └── models.rs      ✅ モデル定義
├── .env
├── bookstore.db
├── migrations/
├── docs/
│   └── Steps/         # 各実装ステップのドキュメント
└── Cargo.toml
```

---

## 次のStep

Step 5: Webサーバーの実装（Actix Web）

---

**完了条件:**
- ✅ `src/models.rs` が作成されている
- ✅ `src/main.rs` が更新されている
- ✅ `cargo run` でデータが挿入・取得できる
- ✅ コンソールにカテゴリ一覧が表示される
