# Step 10: カテゴリ別集計API実装

**目的**: カテゴリごとの売上を集計して返すAPIを実装する（JOIN + 集計関数）

---

## 手順

### 1. カテゴリ別集計構造体を追加

**`src/models.rs` に集計用構造体を追加:**

```rust
#[derive(Debug, Serialize)]
pub struct CategorySummary {
    pub category_id: i64,
    pub category_name: String,
    pub total_amount: i64,
    pub product_count: i64,
    pub products: Vec<CategoryProduct>,
}

#[derive(Debug, Serialize)]
pub struct CategoryProduct {
    pub product_id: i64,
    pub product_name: String,
    pub total_amount: i64,
}
```

### 2. カテゴリ別集計APIを実装

**`src/main.rs` にカテゴリ別集計APIを追加:**

```rust
async fn get_category_summary(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // カテゴリ一覧を取得
    let categories = sqlx::query_as::<_, models::Category>(
        "SELECT id, name FROM categories ORDER BY name"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut result = Vec::new();

    for category in categories {
        // カテゴリ別売上集計
        let summary = sqlx::query(
            r#"
            SELECT
                p.id as product_id,
                p.name as product_name,
                COALESCE(SUM(s.quantity * s.sale_price), 0) as total_amount
            FROM products p
            LEFT JOIN product_categories pc ON p.id = pc.product_id
            LEFT JOIN sales s ON p.id = s.product_id
            WHERE pc.category_id = ?
            GROUP BY p.id, p.name
            ORDER BY total_amount DESC
            "#
        )
        .bind(category.id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        let mut products = Vec::new();
        let mut total_amount = 0i64;

        for row in summary {
            let product_id: i64 = row.try_get("product_id")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let product_name: String = row.try_get("product_name")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let amount: i64 = row.try_get("total_amount")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            total_amount += amount;

            products.push(models::CategoryProduct {
                product_id,
                product_name,
                total_amount: amount,
            });
        }

        result.push(models::CategorySummary {
            category_id: category.id,
            category_name: category.name,
            total_amount,
            product_count: products.len() as i64,
            products,
        });
    }

    Ok(HttpResponse::Ok().json(result))
}
```

### 3. ルーティング追加

```rust
.route("/api/category-summary", web::get().to(get_category_summary))
```

### 4. トップページにリンク追加

```html
<li><a href="/api/category-summary">カテゴリ別集計 (JSON API)</a></li>
```

### 5. 動作確認

```bash
cargo run
```

**ブラウザでアクセス:**
```
http://localhost:8080/api/category-summary
```

**期待されるJSONレスポンス:**
```json
[
  {
    "category_id": 1,
    "category_name": "小説",
    "total_amount": 0,
    "product_count": 0,
    "products": []
  },
  {
    "category_id": 2,
    "category_name": "ビジネス書",
    "total_amount": 0,
    "product_count": 0,
    "products": []
  }
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ `/api/category-summary` でカテゴリ別集計が取得できる
- ✅ 各カテゴリの合計金額が計算されている
- ✅ 商品数が正しくカウントされている
- ✅ 商品ごとの売上が表示されている

---

## 完了

これで全Step（Step 1-10）が完了しました！

**実装した機能:**
- Step 1-5: プロジェクト作成、依存関係、データベース、Webサーバー
- Step 6: 商品一覧API
- Step 7: 顧客一覧API
- Step 8: 販売ヘッダー・明細API
- Step 9: 売上ランキングAPI
- Step 10: カテゴリ別集計API

---

**完了条件:**
- ✅ `/api/category-summary` でカテゴリ別集計が取得できる
- ✅ JOINで商品とカテゴリを結合している
- ✅ 集計関数で合計金額を計算している
- ✅ カテゴリごとの商品リストが含まれている
