# Step 7: 顧客一覧API実装

**目的**: 顧客情報をJSON APIで返すエンドポイントを実装する

---

## 手順

### 1. 顧客取得APIを実装

**`src/main.rs` に顧客一覧APIを追加:**

ルーティングに追加：
```rust
.route("/api/customers", web::get().to(get_customers))
```

ハンドラーを追加：
```rust
async fn get_customers(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let customers = sqlx::query_as::<_, models::Customer>(
        "SELECT id, name, email, member_number FROM customers"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(customers))
}
```

### 2. サンプル顧客データの挿入

起動時に顧客データを挿入：
```rust
// サンプル顧客データの挿入（初回のみ）
let customer_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
    .fetch_one(&pool)
    .await?;

if customer_count == 0 {
    println!("👤 Inserting sample customers...");

    sqlx::query(
        "INSERT INTO customers (name, email, member_number) VALUES
        ('山田太郎', 'yamada@example.com', 'M001'),
        ('鈴木花子', 'suzuki@example.com', 'M002'),
        ('佐藤次郎', 'sato@example.com', 'M003'),
        ('田中美咲', 'tanaka@example.com', 'M004'),
        ('伊藤健太', 'ito@example.com', 'M005')"
    )
    .execute(&pool)
    .await?;

    println!("✅ Inserted 5 sample customers");
}
```

### 3. トップページにリンク追加

```html
<li><a href="/api/customers">顧客一覧 (JSON API)</a></li>
```

### 4. サーバーを起動

```bash
cargo run
```

### 5. 動作確認

**ブラウザでアクセス:**
```
http://localhost:8080/api/customers
```

**期待されるJSONレスポンス:**
```json
[
  {"id":1,"name":"山田太郎","email":"yamada@example.com","member_number":"M001"},
  {"id":2,"name":"鈴木花子","email":"suzuki@example.com","member_number":"M002"},
  {"id":3,"name":"佐藤次郎","email":"sato@example.com","member_number":"M003"},
  {"id":4,"name":"田中美咲","email":"tanaka@example.com","member_number":"M004"},
  {"id":5,"name":"伊藤健太","email":"ito@example.com","member_number":"M005"}
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ サンプル顧客が5件挿入される（初回のみ）
- ✅ `/api/customers` で顧客一覧が取得できる
- ✅ JSONに顧客情報（名前、メール、会員番号）が含まれている

---

## 次のStep

Step 8: 販売ヘッダー・明細API実装

---

**完了条件:**
- ✅ `/api/customers` で顧客一覧が取得できる
- ✅ サンプル顧客5件がデータベースに登録されている
- ✅ JSON形式で正しくレスポンスが返ってくる
