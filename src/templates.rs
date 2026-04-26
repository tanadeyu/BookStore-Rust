use askama::Template;
use crate::models::{SalesRanking, CategorySales, CustomerSales, ProductWithCategories, Customer, SaleDetail};

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
