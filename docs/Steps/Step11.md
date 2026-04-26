# Step 11: HTMLテンプレート機能実装（Askama）

**目的**: Askamaを使ってHTMLページを表示する

---

## 手順

### 1. Askamaテンプレート設定

**`Cargo.toml` でAskamaが有効になっていることを確認:**
```toml
askama = "0.13"
```

**`templates/` ディレクトリを作成:**
```bash
mkdir -p templates
```

### 2. テンプレート構造体を作成

**`src/templates.rs` を作成:**
```rust
use askama::Template;
use crate::models::{
    SalesRanking, CategorySales, CustomerSales,
    ProductWithCategories, Customer, SaleDetail
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct DashboardTemplate {
    pub total_sales: i64,
    pub total_quantity: i64,
    pub ranking: Vec<SalesRanking>,
    pub category_sales: Vec<CategorySales>,
    pub customer_sales: Vec<CustomerSales>,
}

#[derive(Template)]
#[template(path = "products.html")]
pub struct ProductsTemplate {
    pub products: Vec<ProductWithCategories>,
}

#[derive(Template)]
#[template(path = "customers.html")]
pub struct CustomersTemplate {
    pub customers: Vec<Customer>,
}

#[derive(Template)]
#[template(path = "sales.html")]
pub struct SalesTemplate {
    pub sales: Vec<SaleDetail>,
}

#[derive(Template)]
#[template(path = "sales-detail.html")]
pub struct SalesDetailTemplate {
    pub sale: SaleDetail,
}

#[derive(Template)]
#[template(path = "sales-create.html")]
pub struct SalesCreateTemplate {
    pub customers: Vec<Customer>,
    pub products: Vec<ProductWithCategories>,
}
```

### 3. モデルを更新

**`src/models.rs` に`total_amount`を追加:**
```rust
#[derive(Debug, Serialize)]
pub struct SaleDetail {
    pub id: i64,
    pub customer_id: i64,
    pub customer_name: String,
    pub sale_date: String,
    pub items: Vec<SaleItem>,
    pub total_amount: i32,  // 追加
}

#[derive(Debug, Serialize)]
pub struct ProductWithCategories {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub stock: i32,
    pub published_date: Option<String>,
    pub categories: Vec<String>,
}
```

### 4. HTMLテンプレートを作成

**`templates/index.html`** - ダッシュボード:
- 総売上金額、総販売冊数を表示
- 売上ランキングTOP10
- カテゴリ別売上
- 顧客別購入金額

**`templates/products.html`** - 商品一覧:
- 商品一覧表
- カテゴリ表示

**`templates/customers.html`** - 顧客一覧:
- 顧客一覧表

**`templates/sales.html`** - 販売一覧:
- 販売一覧表
- 新規登録ボタン
- 詳細ボタン（Ajaxモーダル）
- 削除ボタン

**`templates/sales-detail.html`** - 販売明細（Ajax用フラグメント）:
```html
<table style="width: 100%; border-collapse: collapse;">
    <thead>
        <tr>
            <th>商品名</th>
            <th>数量</th>
            <th>単価</th>
            <th>小計</th>
        </tr>
    </thead>
    <tbody>
        {% for item in sale.items %}
        <tr>
            <td>{{ item.product_name }}</td>
            <td>{{ item.quantity }}</td>
            <td>{{ item.sale_price }}円</td>
            <td>{{ item.subtotal }}円</td>
        </tr>
        {% endfor %}
    </tbody>
    <tfoot>
        <tr>
            <td colspan="3" style="text-align: right; font-weight: 600;">合計</td>
            <td style="text-align: right; font-weight: 600;">{{ sale.total_amount }}円</td>
        </tr>
    </tfoot>
</table>
```

**`templates/sales-create.html`** - 販売登録:
```html
<form action="/sales/create" method="post">
    <div class="form-row">
        <div class="form-group">
            <label>販売日</label>
            <input type="date" name="saleDate" required>
        </div>
        <div class="form-group">
            <label>顧客</label>
            <select name="customerId" required>
                <option value="">選択してください</option>
                {% for customer in customers %}
                <option value="{{ customer.id }}">{{ customer.name }}</option>
                {% endfor %}
            </select>
        </div>
    </div>

    <!-- 商品1 -->
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

    <!-- 商品2, 商品3も同様に productId2/quantity2, productId3/quantity3 として追加 -->

    <button type="submit">登録</button>
</form>
```

### 5. ハンドラーを実装

**ダッシュボード:**
```rust
async fn index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // データ取得処理...
    let template = templates::DashboardTemplate { /* データ */ };
    let html = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}
```

**ページハンドラー:**
```rust
async fn products_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error>
    // 商品取得＆カテゴリ結合...
}

async fn customers_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 顧客取得...
}

async fn sales_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 販売取得＆合計計算...
}
```

### 6. 詳細API（Ajax用）

```rust
async fn get_sale_detail(
    id: web::Path<i64>,
    pool: web::Data<SqlitePool>
) -> Result<HttpResponse, actix_web::Error> {
    // 販売詳細取得...
    let template = templates::SalesDetailTemplate { sale };
    let html = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}
```

### 7. ルーティング追加

```rust
.route("/", web::get().to(index))
.route("/products", web::get().to(products_page))
.route("/customers", web::get().to(customers_page))
.route("/sales", web::get().to(sales_page))
.route("/sales/new", web::get().to(sales_create_page))
.route("/sales/create", web::post().to(create_sale))
.route("/sales/{id}/detail", web::get().to(get_sale_detail))
.route("/sales/{id}/delete", web::post().to(delete_sale))
```

### 8. Askamaテンプレート構文

**基本的な構文:**
```html
<!-- 変数表示 -->
{{ variable }}

<!-- 条件分岐 -->
{% if let Some(value) = &optional %}
    {{ value }}
{% else %}
    -
{% endif %}

<!-- ループ -->
{% for item in items %}
    {{ loop.index }}: {{ item.name }}
{% endfor %}

<!-- ループのインデックス -->
{{ loop.index }}    <!-- 1始まり -->
{{ loop.first }}    <!-- 最初の要素 -->
{{ loop.last }}     <!-- 最後の要素 -->
```

### 9. 動作確認

```bash
cargo run
```

**確認URL:**
- http://localhost:8080/ - ダッシュボード
- http://localhost:8080/products - 商品一覧
- http://localhost:8080/customers - 顧客一覧
- http://localhost:8080/sales - 販売一覧

---

## 確認

**動作チェックリスト:**
- ✅ すべてのHTMLページが表示される
- ✅ ダッシュボードに集計データが表示される
- ✅ 詳細モーダルがAjaxで動作する
- ✅ Askamaテンプレートがコンパイルされる

---

## 完了

これでHTMLテンプレート機能が完成しました！

**実装したページ:**
- ダッシュボード（統計サマリー、ランキング、集計）
- 商品一覧（カテゴリ付き）
- 顧客一覧
- 販売一覧（詳細モーダル、削除）
- 販売登録（最大3商品）

---

**完了条件:**
- ✅ AskamaでHTMLテンプレートがレンダリングされる
- ✅ すべてのページがブラウザで表示される
- ✅ Ajaxで詳細が取得できる
- ✅ モーダルが正常に動作する
