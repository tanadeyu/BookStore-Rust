# Step 6: 商品一覧API実装

**目的**: 商品情報をJSON APIで返すエンドポイントを実装する

---

## 手順

### 1. 商品取得APIを実装

**`src/main.rs` に商品一覧APIを追加:**

```rust
mod models;

use actix_web::{web, App, HttpServer, HttpResponse};
use sqlx::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://bookstore.db".to_string());

    println!("🔌 Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;
    println!("✅ Database connected");

    // カテゴリを取得
    let categories = sqlx::query_as::<_, models::Category>(
        "SELECT id, name FROM categories"
    )
    .fetch_all(&pool)
    .await?;

    println!("📊 Loaded {} categories from database", categories.len());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/api/categories", web::get().to(get_categories))
            .route("/api/products", web::get().to(get_products))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .unwrap();

    Ok(())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>BookStore Rust</title>
            <meta charset="utf-8">
        </head>
        <body>
            <h1>📚 BookStore Rust</h1>
            <p>Actix Web + sqlx + SQLite で動いています！</p>
            <ul>
                <li><a href="/api/categories">カテゴリ一覧 (JSON API)</a></li>
                <li><a href="/api/products">商品一覧 (JSON API)</a></li>
            </ul>
        </body>
        </html>
        "#
    )
}

async fn get_categories(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let categories = sqlx::query_as::<_, models::Category>(
        "SELECT id, name FROM categories"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(categories))
}

async fn get_products(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let products = sqlx::query_as::<_, models::Product>(
        "SELECT id, name, description, price, stock, published_date FROM products"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(products))
}
```

### 2. データベースに商品データを追加

まだ商品データがない場合は、`src/main.rs` の起動時に追加します：

```rust
// サンプル商品データの挿入（初回のみ）
let product_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
    .fetch_one(&pool)
    .await?;

if product_count == 0 {
    println!("📦 Inserting sample products...");

    sqlx::query(
        "INSERT INTO products (name, description, price, stock, published_date) VALUES
        ('Rustプログラミング', 'Rust言語の入門書', 3000, 50, '2024-01-15'),
        ('実践Rust入門', 'Rustの実践的な使い方', 3500, 30, '2024-02-20'),
        ('アクターモデル詳解', '並行処理の基礎', 2800, 20, '2024-03-10'),
        ('Webアプリ開発', 'Actix Webで作るWebアプリ', 3200, 40, '2024-04-01'),
        ('データベース設計', 'SQLと設計の基礎', 2500, 60, '2024-01-25'),
        ('アルゴリズム図鑑', '図解で学ぶアルゴリズム', 2700, 45, '2024-02-15'),
        ('デザインパターン', 'GoFパターンをRustで', 3800, 25, '2024-03-25'),
        ('テスト駆動開発', 'TDDの実践', 2900, 35, '2024-04-10'),
        ('リファクタリング', 'コード改善の技法', 3300, 28, '2024-01-30'),
        ('クリーンアーキテクチャ', 'ソフトウェア構造設計', 4200, 22, '2024-03-05')"
    )
    .execute(&pool)
    .await?;

    println!("✅ Inserted 10 sample products");
}
```

### 3. サーバーを起動

```bash
cargo run
```

**期待される出力:**
```
🔌 Connecting to database: sqlite://bookstore.db
✅ Database connected
📊 Loaded 5 categories from database
📦 Inserting sample products...
✅ Inserted 10 sample products
```

### 4. 動作確認

**ブラウザでアクセス:**
```
http://localhost:8080/api/products
```

**期待されるJSONレスポンス:**
```json
[
  {"id":1,"name":"Rustプログラミング","description":"Rust言語の入門書","price":3000,"stock":50,"published_date":"2024-01-15"},
  {"id":2,"name":"実践Rust入門","description":"Rustの実践的な使い方","price":3500,"stock":30,"published_date":"2024-02-20"},
  {"id":3,"name":"アクターモデル詳解","description":"並行処理の基礎","price":2800,"stock":20,"published_date":"2024-03-10"},
  {"id":4,"name":"Webアプリ開発","description":"Actix Webで作るWebアプリ","price":3200,"stock":40,"published_date":"2024-04-01"},
  {"id":5,"name":"データベース設計","description":"SQLと設計の基礎","price":2500,"stock":60,"published_date":"2024-01-25"},
  {"id":6,"name":"アルゴリズム図鑑","description":"図解で学ぶアルゴリズム","price":2700,"stock":45,"published_date":"2024-02-15"},
  {"id":7,"name":"デザインパターン","description":"GoFパターンをRustで","price":3800,"stock":25,"published_date":"2024-03-25"},
  {"id":8,"name":"テスト駆動開発","description":"TDDの実践","price":2900,"stock":35,"published_date":"2024-04-10"},
  {"id":9,"name":"リファクタリング","description":"コード改善の技法","price":3300,"stock":28,"published_date":"2024-01-30"},
  {"id":10,"name":"クリーンアーキテクチャ","description":"ソフトウェア構造設計","price":4200,"stock":22,"published_date":"2024-03-05"}
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ サンプル商品が10件挿入される（初回のみ）
- ✅ `/api/products` で商品一覧が取得できる
- ✅ JSONに商品情報（名前、説明、価格、在庫、発行日）が含まれている

---

## トラブルシューティング

### 商品データが空の場合

データベースを削除して再作成：
```bash
rm bookstore.db
cargo run
```

---

## 次のStep

Step 7: 顧客一覧API実装

---

**完了条件:**
- ✅ `/api/products` で商品一覧が取得できる
- ✅ サンプル商品10件がデータベースに登録されている
- ✅ JSON形式で正しくレスポンスが返ってくる
