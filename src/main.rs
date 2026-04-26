mod models;
mod templates;

use actix_web::{web, App, HttpServer, HttpResponse};
use askama::Template;
use sqlx::{Row, SqlitePool};
use std::{env, fs, path::PathBuf};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:bookstore.db".to_string());

    println!("🔌 Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;
    println!("✅ Database connected");

    // テーブル作成（初回のみ）
    let schema = fs::read_to_string("migrations/init.sql")
        .expect("Failed to read migrations/init.sql");
    sqlx::query(&schema).execute(&pool).await
        .expect("Failed to create tables");
    println!("✅ Tables created/verified");

    // カテゴリを取得
    let categories = sqlx::query_as::<_, models::Category>(
        "SELECT id, name FROM categories"
    )
    .fetch_all(&pool)
    .await?;

    println!("📊 Loaded {} categories from database", categories.len());

    // サンプルカテゴリデータの挿入（初回のみ）
    if categories.is_empty() {
        println!("📁 Inserting sample categories...");

        sqlx::query(
            "INSERT INTO categories (name) VALUES
            ('小説'),
            ('ビジネス書'),
            ('コミック'),
            ('雑誌'),
            ('教育・参考書')"
        )
        .execute(&pool)
        .await?;

        println!("✅ Inserted 5 sample categories");
    }

    // サンプル商品データの挿入（初回のみ）
    println!("🔍 Counting products...");
    let product_count: i64 = match sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await
    {
        Ok(count) => {
            println!("✅ Product count: {}", count);
            count
        },
        Err(e) => {
            eprintln!("❌ Error counting products: {}", e);
            0
        }
    };

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

        // 商品とカテゴリの関連付け
        sqlx::query(
            "INSERT INTO product_categories (product_id, category_id) VALUES
            (1, 5), (2, 5), (3, 5), (4, 5), (5, 5),
            (6, 5), (7, 2), (8, 2), (9, 2), (10, 2)"
        )
        .execute(&pool)
        .await?;

        println!("✅ Inserted product-category associations");
    }

    // サンプル顧客データの挿入（初回のみ）
    println!("🔍 Counting customers...");
    let customer_count: i64 = match sqlx::query_scalar("SELECT COUNT(*) FROM customers")
        .fetch_one(&pool)
        .await
    {
        Ok(count) => {
            println!("✅ Customer count: {}", count);
            count
        },
        Err(e) => {
            eprintln!("❌ Error counting customers: {}", e);
            0
        }
    };

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

    // サンプル販売データの挿入（初回のみ）
    println!("🔍 Counting sales...");
    let sale_count: i64 = match sqlx::query_scalar("SELECT COUNT(*) FROM sale_headers")
        .fetch_one(&pool)
        .await
    {
        Ok(count) => {
            println!("✅ Sale count: {}", count);
            count
        },
        Err(e) => {
            eprintln!("❌ Error counting sales: {}", e);
            0
        }
    };

    if sale_count == 0 {
        println!("🛒 Inserting sample sales...");

        // 顧客IDを取得
        let customers: Vec<(i64, String)> = sqlx::query_as(
            "SELECT id, name FROM customers"
        )
        .fetch_all(&pool)
        .await?;

        // 商品IDを取得
        let products: Vec<(i64, String)> = sqlx::query_as(
            "SELECT id, name FROM products"
        )
        .fetch_all(&pool)
        .await?;

        println!("📋 Customers: {:?}", customers);
        println!("📦 Products: {:?}", products);

        if customers.len() >= 5 && products.len() >= 10 {
            // 販売1: 山田太郎 - 2024-03-15 (Rustプログラミングx2 + 実践Rust入門x1 = 9500円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[0].0)
            .bind("2024-03-15")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
                (?, ?, 2, 3000),
                (?, ?, 1, 3500)"
            )
            .bind(header_id).bind(products[0].0)
            .bind(header_id).bind(products[1].0)
            .execute(&pool).await?;

            // 販売2: 鈴木花子 - 2024-03-20 (アクターモデル詳解x3 = 8400円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[1].0)
            .bind("2024-03-20")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES (?, ?, 3, 2800)"
            )
            .bind(header_id).bind(products[2].0)
            .execute(&pool).await?;

            // 販売3: 佐藤次郎 - 2024-04-01 (Webアプリ開発x2 + データベース設計x1 + アルゴリズム図鑑x1 = 11600円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[2].0)
            .bind("2024-04-01")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
                (?, ?, 2, 3200),
                (?, ?, 1, 2500),
                (?, ?, 1, 2700)"
            )
            .bind(header_id).bind(products[3].0)
            .bind(header_id).bind(products[4].0)
            .bind(header_id).bind(products[5].0)
            .execute(&pool).await?;

            // 販売4: 田中美咲 - 2024-04-10 (デザインパターンx3 + テスト駆動開発x2 = 17200円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[3].0)
            .bind("2024-04-10")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
                (?, ?, 3, 3800),
                (?, ?, 2, 2900)"
            )
            .bind(header_id).bind(products[6].0)
            .bind(header_id).bind(products[7].0)
            .execute(&pool).await?;

            // 販売5: 伊藤健太 - 2024-04-15 (リファクタリングx1 + クリーンアーキテクチャx1 = 7500円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[4].0)
            .bind("2024-04-15")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
                (?, ?, 1, 3300),
                (?, ?, 1, 4200)"
            )
            .bind(header_id).bind(products[8].0)
            .bind(header_id).bind(products[9].0)
            .execute(&pool).await?;

            // 販売6: 山田太郎 - 2024-04-20 (Rustプログラミングx1 = 3000円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[0].0)
            .bind("2024-04-20")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES (?, ?, 1, 3000)"
            )
            .bind(header_id).bind(products[0].0)
            .execute(&pool).await?;

            // 販売7: 鈴木花子 - 2024-04-25 (実践Rust入門x2 + アクターモデル詳解x2 + デザインパターンx1 = 16400円)
            let header_id: i64 = sqlx::query_scalar(
                "INSERT INTO sale_headers (customer_id, sale_date) VALUES (?, ?) RETURNING id"
            )
            .bind(customers[1].0)
            .bind("2024-04-25")
            .fetch_one(&pool)
            .await?;
            sqlx::query(
                "INSERT INTO sales (sale_header_id, product_id, quantity, sale_price) VALUES
                (?, ?, 2, 3500),
                (?, ?, 2, 2800),
                (?, ?, 1, 3800)"
            )
            .bind(header_id).bind(products[1].0)
            .bind(header_id).bind(products[2].0)
            .bind(header_id).bind(products[6].0)
            .execute(&pool).await?;

            println!("✅ Inserted 7 sample sales");
        } else {
            println!("⚠️ Not enough data to create sales");
        }
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/products", web::get().to(products_page))
            .route("/customers", web::get().to(customers_page))
            .route("/sales", web::get().to(sales_page))
            .route("/sales/new", web::get().to(sales_create_page))
            .route("/sales/create", web::post().to(create_sale))
            .route("/sales/{id}/detail", web::get().to(get_sale_detail))
            .route("/sales/{id}/delete", web::post().to(delete_sale))
            .route("/api/categories", web::get().to(get_categories))
            .route("/api/products", web::get().to(get_products))
            .route("/api/customers", web::get().to(get_customers))
            .route("/api/sales", web::get().to(get_sales))
            .route("/api/ranking", web::get().to(get_sales_ranking))
            .route("/api/category-summary", web::get().to(get_category_summary))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .unwrap();

    Ok(())
}

async fn index(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 総売上・総販売数
    let totals = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(s.quantity * s.sale_price), 0) as total_sales,
            COALESCE(SUM(s.quantity), 0) as total_quantity
        FROM sales s
        "#
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let total_sales: i64 = totals.try_get("total_sales")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let total_quantity: i64 = totals.try_get("total_quantity")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 売上ランキングTOP10
    let ranking = sqlx::query(
        r#"
        SELECT
            p.id as product_id,
            p.name as product_name,
            COALESCE(SUM(s.quantity * s.sale_price), 0) as total_amount,
            COALESCE(SUM(s.quantity), 0) as total_quantity,
            COALESCE(COUNT(s.id), 0) as sale_count
        FROM products p
        LEFT JOIN sales s ON p.id = s.product_id
        GROUP BY p.id, p.name
        ORDER BY total_amount DESC
        LIMIT 10
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut rankings = Vec::new();
    for (idx, row) in ranking.iter().enumerate() {
        let product_id: i64 = row.try_get("product_id")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let product_name: String = row.try_get("product_name")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_amount: i64 = row.try_get("total_amount")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_quantity: i64 = row.try_get("total_quantity")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let sale_count: i64 = row.try_get("sale_count")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        rankings.push(models::SalesRanking {
            rank: (idx + 1) as i32,
            product_id,
            product_name,
            total_amount,
            total_quantity,
            sale_count,
        });
    }

    // カテゴリ別売上
    let cat_sales = sqlx::query(
        r#"
        SELECT
            c.name as category_name,
            COALESCE(SUM(s.quantity * s.sale_price), 0) as total_amount,
            COALESCE(SUM(s.quantity), 0) as product_count
        FROM categories c
        LEFT JOIN product_categories pc ON c.id = pc.category_id
        LEFT JOIN products p ON pc.product_id = p.id
        LEFT JOIN sales s ON p.id = s.product_id
        GROUP BY c.id, c.name
        ORDER BY total_amount DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut category_sales = Vec::new();
    for row in cat_sales {
        let category_name: String = row.try_get("category_name")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_amount: i64 = row.try_get("total_amount")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let product_count: i64 = row.try_get("product_count")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        category_sales.push(models::CategorySales {
            category_name,
            total_amount,
            product_count,
        });
    }

    // 顧客別売上
    let cust_sales = sqlx::query(
        r#"
        SELECT
            c.name as customer_name,
            COALESCE(SUM(s.quantity * s.sale_price), 0) as total_amount,
            COUNT(DISTINCT sh.id) as sale_count
        FROM customers c
        LEFT JOIN sale_headers sh ON c.id = sh.customer_id
        LEFT JOIN sales s ON sh.id = s.sale_header_id
        GROUP BY c.id, c.name
        ORDER BY total_amount DESC
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut customer_sales = Vec::new();
    for row in cust_sales {
        let customer_name: String = row.try_get("customer_name")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_amount: i64 = row.try_get("total_amount")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let sale_count: i64 = row.try_get("sale_count")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        customer_sales.push(models::CustomerSales {
            customer_name,
            total_amount,
            sale_count,
        });
    }

    let template = templates::DashboardTemplate {
        total_sales,
        total_quantity,
        ranking: rankings,
        category_sales,
        customer_sales,
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

async fn customers_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let customers = sqlx::query_as::<_, models::Customer>(
        "SELECT id, name, email, member_number FROM customers"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let template = templates::CustomersTemplate {
        customers,
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
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

async fn get_customers(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let customers = sqlx::query_as::<_, models::Customer>(
        "SELECT id, name, email, member_number FROM customers"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(customers))
}

async fn sales_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    let headers = sqlx::query_as::<_, models::SaleHeader>(
        "SELECT id, customer_id, sale_date FROM sale_headers ORDER BY sale_date DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut sales = Vec::new();
    for header in headers {
        let customer: Option<String> = sqlx::query_scalar(
            "SELECT name FROM customers WHERE id = ?"
        )
        .bind(header.customer_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

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

        sales.push(models::SaleDetail {
            id: header.id,
            customer_id: header.customer_id,
            customer_name: customer.unwrap_or_else(|| "不明".to_string()),
            sale_date: header.sale_date,
            items: sale_items,
            total_amount,
        });
    }

    let template = templates::SalesTemplate {
        sales,
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

async fn products_page(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 商品を取得
    let products = sqlx::query_as::<_, models::Product>(
        "SELECT id, name, description, price, stock, published_date FROM products"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 各商品のカテゴリを取得
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
        .map(|row| row.try_get("name")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e)))
        .collect::<Result<Vec<_>, _>>()?;

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

    let template = templates::ProductsTemplate {
        products: products_with_categories,
    };

    let html = template.render()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html))
}

async fn get_sales(pool: web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    // 販売ヘッダーを取得
    let headers = sqlx::query_as::<_, models::SaleHeader>(
        "SELECT id, customer_id, sale_date FROM sale_headers ORDER BY sale_date DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // 各ヘッダーの明細を取得して結合
    let mut sales = Vec::new();
    for header in headers {
        // 顧客名を取得
        let customer: Option<String> = sqlx::query_scalar(
            "SELECT name FROM customers WHERE id = ?"
        )
        .bind(header.customer_id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // 明細を取得
        let items = sqlx::query_as::<_, models::Sale>(
            "SELECT id, sale_header_id, product_id, quantity, sale_price FROM sales WHERE sale_header_id = ?"
        )
        .bind(header.id)
        .fetch_all(pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // 明細に商品名を追加
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

        sales.push(models::SaleDetail {
            id: header.id,
            customer_id: header.customer_id,
            customer_name: customer.unwrap_or_else(|| "不明".to_string()),
            sale_date: header.sale_date,
            items: sale_items,
            total_amount,
        });
    }

    Ok(HttpResponse::Ok().json(sales))
}

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
        let product_id: i64 = row.try_get("product_id")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let product_name: String = row.try_get("product_name")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_amount: i64 = row.try_get("total_amount")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let total_quantity: i64 = row.try_get("total_quantity")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        let sale_count: i64 = row.try_get("sale_count")
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

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
                COALESCE(SUM(s.quantity * s.sale_price), 0) as total_amount,
                COALESCE(SUM(s.quantity), 0) as total_quantity
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
        let mut total_quantity = 0i64;

        for row in summary {
            let product_id: i64 = row.try_get("product_id")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let product_name: String = row.try_get("product_name")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let amount: i64 = row.try_get("total_amount")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            let quantity: i64 = row.try_get("total_quantity")
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            total_amount += amount;
            total_quantity += quantity;

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
            product_count: total_quantity,
            products,
        });
    }

    Ok(HttpResponse::Ok().json(result))
}

// 新規販売登録ページ表示
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

// 販売登録処理
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

// 販売詳細（Ajax用）
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

// 販売削除
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
