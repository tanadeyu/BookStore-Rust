use sqlx::FromRow;
use serde::{Serialize, Deserialize};

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

// 販売詳細（ヘッダー+明細）
#[derive(Debug, Serialize)]
pub struct SaleDetail {
    pub id: i64,
    pub customer_id: i64,
    pub customer_name: String,
    pub sale_date: String,
    pub items: Vec<SaleItem>,
    pub total_amount: i32,
}

#[derive(Debug, Serialize)]
pub struct SaleItem {
    pub product_id: i64,
    pub product_name: String,
    pub quantity: i32,
    pub sale_price: i32,
    pub subtotal: i32,
}

#[derive(Debug, Serialize)]
pub struct SalesRanking {
    pub rank: i32,
    pub product_id: i64,
    pub product_name: String,
    pub total_amount: i64,
    pub total_quantity: i64,
    pub sale_count: i64,
}

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

// ダッシュボード用
#[derive(Debug, Serialize)]
pub struct CategorySales {
    pub category_name: String,
    pub total_amount: i64,
    pub product_count: i64,
}

#[derive(Debug, Serialize)]
pub struct CustomerSales {
    pub customer_name: String,
    pub total_amount: i64,
    pub sale_count: i64,
}

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
