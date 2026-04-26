# Step 8: 販売ヘッダー・明細API実装

**目的**: 販売データ（ヘッダーと明細）をJSON APIで返すエンドポイントを実装する

---

## 手順

### 1. 販売ヘッダー・明細構造体を追加

**`src/models.rs` にレスポンス用構造体を追加:**

```rust
// 販売詳細（ヘッダー+明細）
#[derive(Debug, Serialize)]
pub struct SaleDetail {
    pub id: i64,
    pub customer_id: i64,
    pub customer_name: String,
    pub sale_date: String,
    pub items: Vec<SaleItem>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SaleItem {
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub sale_price: i32,
    pub subtotal: i32,
}
```

### 2. 販売一覧APIを実装

**`src/main.rs` に販売一覧APIを追加:**

```rust
async fn get_sales(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 販売ヘッダーを取得
    let headers = sqlx::query_as::<_, models::SaleHeader>(
        "SELECT id, customer_id, sale_date FROM sale_headers ORDER BY sale_date DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 各ヘッダーの明細を取得
    let mut sales = Vec::new();
    for header in headers {
        let items = sqlx::query_as::<_, models::Sale>(
            "SELECT id, sale_header_id, product_id, quantity, sale_price FROM sales WHERE sale_header_id = ?"
        )
        .bind(header.id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // 顧客名を取得
        let customer: Option<String> = sqlx::query_scalar(
            "SELECT name FROM customers WHERE id = ?"
        )
        .bind(header.customer_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        sales.push(models::SaleDetail {
            id: header.id,
            customer_id: header.customer_id,
            customer_name: customer.unwrap_or_else(|| "不明".to_string()),
            sale_date: header.sale_date,
            items: items.into_iter().map(|item| {
                // 商品名を取得
                let product_name: String = sqlx::query_scalar(
                    "SELECT name FROM products WHERE id = ?"
                )
                .bind(item.product_id)
                .fetch_one(pool.get_ref())
                .await
                .unwrap_or_else(|_| "不明".to_string());

                models::SaleItem {
                    product_id: item.product_id,
                    product_name,
                    quantity: item.quantity,
                    sale_price: item.sale_price,
                    subtotal: item.quantity * item.sale_price,
                }
            }).collect(),
        });
    }

    Ok(HttpResponse::Ok().json(sales))
}
```

### 3. サンプル販売データの挿入

```rust
// サンプル販売データの挿入（初回のみ）
let sale_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sale_headers")
    .fetch_one(&pool)
    .await?;

if sale_count == 0 {
    println!("🛒 Inserting sample sales...");

    // 販売ヘッダー1
    let header_id = sqlx::query_scalar(
        "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
    )
    .bind(1)  // 山田太郎
    .bind("2024-04-01")
    .fetch_one(&pool)
    .await?;

    // 明細
    sqlx::query(
        "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
        (?, 1, 2, 3000),  -- Rustプログラミング x2
        (?, 3, 1, 2800)"  -- アクターモデル詳解 x1
    )
    .bind(header_id)
    .bind(header_id)
    .execute(&pool)
    .await?;

    // 販売ヘッダー2
    let header_id = sqlx::query_scalar(
        "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
    )
    .bind(2)  // 鈴木花子
    .bind("2024-04-02")
    .fetch_one(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
        (?, 4, 1, 3200),  -- Webアプリ開発
        (?, 5, 3, 2500)"  -- データベース設計 x3
    )
    .bind(header_id)
    .bind(header_id)
    .execute(&pool)
    .await?;

    println!("✅ Inserted 2 sample sales");
}
```

### 4. ルーティング追加

```rust
.route("/api/sales", web::get().to(get_sales))
```

### 5. トップページにリンク追加

```html
<li><a href="/api/sales">販売一覧 (JSON API)</a></li>
```

### 6. 動作確認

```bash
cargo run
```

**ブラウザでアクセス:**
```
http://localhost:8080/api/sales
```

**期待されるJSONレスポンス:**
```json
[
  {
    "id": 2,
    "customer_id": 2,
    "customer_name": "鈴木花子",
    "sale_date": "2024-04-02",
    "items": [
      {"product_id": 4, "product_name": "Webアプリ開発", "quantity": 1, "sale_price": 3200, "subtotal": 3200},
      {"product_id": 5, "product_name": "データベース設計", "quantity": 3, "sale_price": 2500, "subtotal": 7500}
    ]
  },
  {
    "id": 1,
    "customer_id": 1,
    "customer_name": "山田太郎",
    "sale_date": "2024-04-01",
    "items": [
      {"product_id": 1, "product_name": "Rustプログラミング", "quantity": 2, "sale_price": 3000, "subtotal": 6000},
      {"product_id": 3, "product_name": "アクターモデル詳解", "quantity": 1, "sale_price": 2800, "subtotal": 2800}
    ]
  }
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ サンプル販売データが挿入される（初回のみ）
- ✅ `/api/sales` で販売一覧が取得できる
- ✅ ヘッダーと明細が結合されて返ってくる
- ✅ 顧客名と商品名が正しく表示される

---

## 次のStep

Step 9: 売上ランキングAPI実装

---

**完了条件:**
- ✅ `/api/sales` で販売一覧が取得できる
- ✅ ヘッダー-明細構造でデータが返ってくる
- ✅ 顧客名と商品名が結合されている
