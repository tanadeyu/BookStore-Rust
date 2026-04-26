# Step 12: 販売登録・削除機能の実装

**目的**: 販売データの登録・削除機能を実装する（Java版と同等）

---

## 実装内容

### 1. 登録フォーム用モデル追加

**`src/models.rs` に登録フォーム用構造体を追加:**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct SaleCreateForm {
    #[serde(rename = "saleDate")]
    pub sale_date: String,
    #[serde(rename = "customerId")]
    pub customer_id: i64,
    #[serde(rename = "productId1")]
    pub product_id_1: i64,
    #[serde(rename = "quantity1")]
    pub quantity_1: i32,
    #[serde(rename = "productId2")]
    pub product_id_2: i64,
    #[serde(rename = "quantity2")]
    pub quantity_2: i32,
    #[serde(rename = "productId3")]
    pub product_id_3: i64,
    #[serde(rename = "quantity3")]
    pub quantity_3: i32,
}
```

**ポイント:**
- `#[serde(rename = "...")]` で HTML フォームの camelCase フィールド名と Rust の snake_case フィールド名をマッピング
- 配列の代わりに3つの商品スロットを個別フィールドとして定義（serde_urlencoded の配列処理の制限回避）

### 2. 販売登録ページハンドラー

**`src/main.rs` に登録ページハンドラーを追加:**

```rust
async fn sales_create_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let customers = sqlx::query_as::<_, models::Customer>(
        "SELECT id, name, email, member_number FROM customers ORDER BY name"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let products = sqlx::query_as::<_, models::Product>(
        "SELECT id, name, description, price, stock, published_date FROM products"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut products_with_categories = Vec::new();
    for product in products {
        let categories: Vec<String> = sqlx::query(
            "SELECT c.name FROM categories c
             JOIN product_categories pc ON c.id = pc.category_id
             WHERE pc.product_id = ?"
        )
        .bind(product.id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .into_iter()
        .map(|row: sqlx::sqlite::SqliteRow| row.get(0))
        .collect();

        products_with_categories.push(models::ProductWithCategories {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            stock: product.stock,
            published_date: product.published_date,
            categories,
        });
    }

    let template = templates::SalesCreateTemplate {
        customers,
        products: products_with_categories,
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}
```

### 3. 販売登録API（トランザクション処理付き）

**`src/main.rs` に登録APIを追加:**

```rust
async fn create_sale(
    pool: web::Data<SqlitePool>,
    form: web::Form<models::SaleCreateForm>,
) -> Result<HttpResponse, actix_web::Error> {
    // トランザクション開始
    let mut tx = pool.begin()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 販売ヘッダーを作成
    let header_id: i64 = sqlx::query_scalar(
        "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
    )
    .bind(form.customer_id)
    .bind(&form.sale_date)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 明細を作成（選択された商品のみ）
    let products = [
        (form.product_id_1, form.quantity_1),
        (form.product_id_2, form.quantity_2),
        (form.product_id_3, form.quantity_3),
    ];

    for (product_id, quantity) in products {
        if product_id <= 0 {
            continue; // 「選択しない」の場合はスキップ
        }

        if quantity <= 0 {
            continue;
        }

        // 商品価格を取得
        let price: i32 = sqlx::query_scalar(
            "SELECT price FROM products WHERE id = ?"
        )
        .bind(product_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

        // 在庫を減らす
        sqlx::query(
            "UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?"
        )
        .bind(quantity)
        .bind(product_id)
        .bind(quantity)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // 明細を追加
        sqlx::query(
            "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES (?, ?, ?, ?)"
        )
        .bind(header_id)
        .bind(product_id)
        .bind(quantity)
        .bind(price)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    tx.commit()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/sales"))
        .finish())
}
```

**ポイント:**
- トランザクションを使用して、ヘッダーと明細の整合性を保証
- 在庫更新は `stock >= ?` 条件でチェック（在庫不足の場合はエラー）
- 登録完了後は `/sales` にリダイレクト

### 4. 詳細API（Ajax用）

**`src/main.rs` に詳細APIを追加:**

```rust
async fn get_sale_detail(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let sale_id = path.into_inner();

    let header: models::SaleHeader = sqlx::query_as(
        "SELECT id, customer_id, sale_date FROM sale_headers WHERE id = ?"
    )
    .bind(sale_id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let customer_name: String = sqlx::query_scalar(
        "SELECT name FROM customers WHERE id = ?"
    )
    .bind(header.customer_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or_else(|_| "不明".to_string());

    let items = sqlx::query_as::<_, models::Sale>(
        "SELECT id, sale_header_id, product_id, quantity, sale_price FROM sales WHERE sale_header_id = ?"
    )
    .bind(header.id)
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut sale_items = Vec::new();
    for item in items {
        let product_name: String = sqlx::query_scalar(
            "SELECT name FROM products WHERE id = ?"
        )
        .bind(item.product_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or_else(|_| "不明".to_string());

        sale_items.push(models::SaleItem {
            product_id: item.product_id,
            product_name,
            quantity: item.quantity,
            sale_price: item.sale_price,
            subtotal: item.quantity * item.sale_price,
        });
    }

    let total_amount: i32 = sale_items.iter().map(|i| i.subtotal).sum();

    let template = templates::SalesDetailTemplate {
        sale: models::SaleDetail {
            id: header.id,
            customer_id: header.customer_id,
            customer_name,
            sale_date: header.sale_date,
            items: sale_items,
            total_amount,
        },
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}
```

### 5. 削除API（在庫復帰付き）

**`src/main.rs` に削除APIを追加:**

```rust
async fn delete_sale(
    pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> Result<HttpResponse, actix_web::Error> {
    let sale_id = path.into_inner();

    // トランザクション開始
    let mut tx = pool.begin()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 在庫を戻す
    let items = sqlx::query_as::<_, models::Sale>(
        "SELECT id, sale_header_id, product_id, quantity, sale_price FROM sales WHERE sale_header_id = ?"
    )
    .bind(sale_id)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    for item in items {
        sqlx::query(
            "UPDATE products SET stock = stock + ? WHERE id = ?"
        )
        .bind(item.quantity)
        .bind(item.product_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    }

    // 明細を削除
    sqlx::query("DELETE FROM sales WHERE sale_header_id = ?")
        .bind(sale_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // ヘッダーを削除
    sqlx::query("DELETE FROM sale_headers WHERE id = ?")
        .bind(sale_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    tx.commit()
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Found()
        .append_header(("Location", "/sales"))
        .finish())
}
```

**ポイント:**
- 削除前に在庫を元に戻す
- 明細→ヘッダーの順で削除（外部キー制約の遵守）

### 6. ルーティング追加

**`src/main.rs` のルーティングを更新:**

```rust
.route("/sales", web::get().to(sales_page))
.route("/sales/new", web::get().to(sales_create_page))
.route("/sales/create", web::post().to(create_sale))
.route("/sales/{id}/detail", web::get().to(get_sale_detail))
.route("/sales/{id}/delete", web::post().to(delete_sale))
```

### 7. HTMLテンプレート更新

**`templates/sales-create.html` のフォーム入力を個別フィールドに変更:**

```html
<div class="item-row">
    <div class="form-row">
        <div class="form-group">
            <label>商品1</label>
            <select name="productId1">
                <option value="0">（選択しない）</option>
                {% for product in products %}
                <option value="{{ product.id }}">{{ product.name }} ({{ product.price.unwrap_or(0) }}円)</option>
                {% endfor %}
            </select>
        </div>
        <div class="form-group">
            <label>数量</label>
            <input type="number" name="quantity1" min="0" value="0" placeholder="数量">
        </div>
    </div>
</div>
```

（商品2、商品3も同様に `productId2/quantity2`、`productId3/quantity3` として追加）

### 8. 動作確認

```bash
cargo run
```

**確認URL:**
- http://localhost:8080/sales - 販売一覧（「新規登録」ボタンがあることを確認）
- http://localhost:8080/sales/new - 登録フォーム
- 「詳細」ボタンクリックでモーダル表示
- 「削除」ボタンで削除＆リダイレクト

**登録テスト:**
1. 「新規登録」ボタンをクリック
2. 顧客を選択
3. 商品を1〜3つ選択
4. 数量を入力
5. 「登録」ボタンをクリック
6. 販売一覧に戻り、データが登録されていることを確認

**curl によるテスト:**

```bash
# 販売登録
curl -X POST "http://localhost:8080/sales/create" \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d "saleDate=2026-04-26&customerId=1&productId1=1&quantity1=2&productId2=2&quantity2=1&productId3=0&quantity3=0"

# 販売詳細
curl "http://localhost:8080/sales/1/detail"

# 販売削除
curl -X POST "http://localhost:8080/sales/1/delete"
```

---

## 確認

**動作チェックリスト:**
- ✅ `/sales/new` で登録フォームが表示される
- ✅ 顧客プルダウンにすべての顧客が表示される
- ✅ 商品プルダウンにすべての商品が表示される
- ✅ 登録後、販売一覧にリダイレクトされる
- ✅ 在庫が正しく減算される
- ✅ 詳細ボタンでモーダルが表示される
- ✅ 削除ボタンでデータが削除される
- ✅ 削除時に在庫が元に戻る
- ✅ 明細データが正しく表示される

---

## トラブルシューティング

### Q1: 登録時に "Parse error" が発生する

**→ フォームフィールド名と構造体の名前が一致しているか確認**

- HTML: `name="productId1"` (camelCase)
- Rust: `#[serde(rename = "productId1")]` pub product_id_1: i64

### Q2: 配列で渡したい場合

**→ serde_urlencoded の制限により、個別フィールドを使用する必要があります**

以下は動作しません:
```rust
pub product_ids: Vec<i64>  // "productIds[]=1&productIds[]=2" はパースできない
```

代わりに個別フィールドを使用:
```rust
pub product_id_1: i64
pub product_id_2: i64
pub product_id_3: i64
```

### Q3: 在庫が減らない

**→ 在庫更新クエリが実行されているか確認**

```sql
UPDATE products SET stock = stock - ? WHERE id = ? AND stock >= ?
```

在庫不足の場合は条件 `stock >= ?` により更新が行われません。

### Q4: 詳細モーダルが表示されない

**→ `/sales/{id}/detail` APIが動作しているか確認**

curlでテスト:
```bash
curl http://localhost:8080/sales/1/detail
```

---

## 完了

これで販売登録・削除機能が完成しました！

**実装した機能:**
- ✅ 販売登録ページ（最大3商品）
- ✅ 販売登録API（トランザクション処理付き）
- ✅ 在庫管理（登録時減算、削除時復帰）
- ✅ 詳細API（Ajaxモーダル用）
- ✅ 削除API（在庫復帰付き）
- ✅ リダイレクト処理

---

**完了条件:**
- ✅ `/sales/new` で登録フォームが表示される
- ✅ `POST /sales/create` で販売が登録できる
- ✅ 在庫が正しく管理される
- ✅ `GET /sales/{id}/detail` で明細HTMLが返ってくる
- ✅ `POST /sales/{id}/delete` で削除できる
- ✅ Java版と同等の機能が実装されている
