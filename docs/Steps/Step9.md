# Step 9: 売上ランキングAPI実装

**目的**: 商品別の売上順位を集計して返すAPIを実装する（GROUP BY + ORDER BY）

---

## 手順

### 1. 売上ランキング構造体を追加

**`src/models.rs` にランキング用構造体を追加:**

```rust
#[derive(Debug, Serialize)]
pub struct SalesRanking {
    pub rank: i32,
    pub product_id: i64,
    pub product_name: String,
    pub total_amount: i64,
    pub total_quantity: i64,
    pub sale_count: i64,
}
```

### 2. 売上ランキングAPIを実装

**`src/main.rs` に売上ランキングAPIを追加:**

```rust
async fn get_sales_ranking(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 商品別売上集計
    let rankings = sqlx::query(
        r#"
        SELECT
            p.id as product_id,
            p.name as product_name,
            SUM(s.quantity * s.sale_price) as total_amount,
            SUM(s.quantity) as total_quantity,
            COUNT(s.id) as sale_count
        FROM products p
        LEFT JOIN sales s ON p.id = s.product_id
        GROUP BY p.id, p.name
        ORDER BY total_amount DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // ランク付け
    let mut result = Vec::new();
    for (idx, row) in rankings.iter().enumerate() {
        let product_id: i64 = row.try_get("product_id")?;
        let product_name: String = row.try_get("product_name")?;
        let total_amount: i64 = row.try_get("total_amount")?;
        let total_quantity: i64 = row.try_get("total_quantity")?;
        let sale_count: i64 = row.try_get("sale_count")?;

        result.push(models::SalesRanking {
            rank: (idx + 1) as i32,
            product_id,
            product_name,
            total_amount,
            total_quantity,
            sale_count,
        });
    }

    Ok(HttpResponse::Ok().json(result))
}
```

### 3. ルーティング追加

```rust
.route("/api/ranking", web::get().to(get_sales_ranking))
```

### 4. トップページにリンク追加

```html
<li><a href="/api/ranking">売上ランキング (JSON API)</a></li>
```

### 5. 動作確認

```bash
cargo run
```

**ブラウザでアクセス:**
```
http://localhost:8080/api/ranking
```

**期待されるJSONレスポンス:**
```json
[
  {
    "rank": 1,
    "product_id": 5,
    "product_name": "データベース設計",
    "total_amount": 7500,
    "total_quantity": 3,
    "sale_count": 1
  },
  {
    "rank": 2,
    "product_id": 1,
    "product_name": "Rustプログラミング",
    "total_amount": 6000,
    "total_quantity": 2,
    "sale_count": 1
  }
]
```

---

## 確認

**動作チェックリスト:**
- ✅ `cargo run` でサーバーが起動する
- ✅ `/api/ranking` でランキングが取得できる
- ✅ 売上金額順にソートされている
- ✅ ランクが正しく付与されている
- ✅ 集計値（合計金額、合計数量、販売回数）が正しい

---

## 次のStep

Step 10: カテゴリ別集計API実装

---

**完了条件:**
- ✅ `/api/ranking` で売上ランキングが取得できる
- ✅ GROUP BYで商品別に集計されている
- ✅ ORDER BYで売上金額順にソートされている
- ✅ ランク、商品名、合計金額が含まれている
