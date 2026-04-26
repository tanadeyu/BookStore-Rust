# Step 5: Webサーバー実装

**目的**: Actix WebでWebサーバーを構築し、ブラウザでアクセスできるようにする

---

## 手順

### 1. main.rsをWebサーバーに書き換え

**`src/main.rs` を以下の内容に書き換え:**

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
```

### 2. サーバーを起動

```bash
cargo run
```

**期待される出力:**
```
🔌 Connecting to database: sqlite://bookstore.db
✅ Database connected
📊 Loaded 5 categories from database
```

サーバーが起動し、`http://127.0.0.1:8080` で待機します。

### 3. ブラウザで確認

**トップページ:**
```
http://localhost:8080/
```

**カテゴリAPI:**
```
http://localhost:8080/api/categories
```

**期待されるJSONレスポンス:**
```json
[
  {"id":1,"name":"小説"},
  {"id":2,"name":"ビジネス書"},
  {"id":3,"name":"コミック"},
  {"id":4,"name":"雑誌"},
  {"id":5,"name":"教育・参考書"}
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ ブラウザで `http://localhost:8080/` にアクセスできる
- ✅ トップページに「BookStore Rust」が表示される
- ✅ `/api/categories` でJSONが返ってくる
- ✅ JSONに5つのカテゴリが含まれている

**サーバーの停止方法:**
- ターミナルで `Ctrl + C` を押す

---

## トラブルシューティング

### エラー: `Address already in use`

**原因:** ポート8080が既に使用されている

**解決策:**
- 別のターミナルで実行中の`cargo run`を停止（`Ctrl + C`）
- またはポート番号を変更（`.bind("127.0.0.1:8081")?`）

### エラー: `the trait bound 'sqlx::Error: ResponseError' is not satisfied`

**原因:** エラー型がActix Webに対応していない

**解決策:**
```rust
// 戻り値の型を変更
async fn get_categories(...) -> Result<HttpResponse, actix_web::Error> {
    // sqlx::Errorをactix_web::Errorに変換
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
}
```

---

## 次のStep

Step 6: 商品一覧API実装

---

**完了条件:**
- ✅ Webサーバーが起動する
- ✅ ブラウザでトップページが見える
- ✅ `/api/categories` でJSONが返ってくる
- ✅ JSONにデータベースのデータが含まれている
