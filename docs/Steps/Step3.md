# Step 3: データベース作成

**目的**: SQLiteデータベースを作成してテーブルを定義する

---

## 手順

### 1. 環境変数ファイルの作成

**`.env` をプロジェクトルートに作成:**

```bash
cd C:/Users/hello/Desktop/project/BookStore-Rust
echo "DATABASE_URL=sqlite://bookstore.db" > .env
```

**確認:**
```bash
cat .env
```

期待される出力:
```
DATABASE_URL=sqlite://bookstore.db
```

### 2. SQLスクリプトでテーブル作成

**`migrations/init.sql` を作成:**

```bash
mkdir -p migrations
```

**`migrations/init.sql` の内容:**

```sql
-- カテゴリマスタ
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

-- 商品マスタ
CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    price INTEGER,
    stock INTEGER DEFAULT 0,
    published_date TEXT
);

-- 商品-カテゴリ中間テーブル
CREATE TABLE IF NOT EXISTS product_categories (
    product_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    PRIMARY KEY (product_id, category_id),
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

-- 顧客マスタ
CREATE TABLE IF NOT EXISTS customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE,
    member_number TEXT UNIQUE
);

-- 販売ヘッダー
CREATE TABLE IF NOT EXISTS sale_headers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    customer_id INTEGER NOT NULL,
    sale_date TEXT NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id)
);

-- 販売明細
CREATE TABLE IF NOT EXISTS sales (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sale_header_id INTEGER NOT NULL,
    product_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    sale_price INTEGER NOT NULL,
    FOREIGN KEY (sale_header_id) REFERENCES sale_headers(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id)
);
```

### 3. データベース初期化

**SQLファイルを実行してデータベースを作成:**

```bash
sqlite3 bookstore.db < migrations/init.sql
```

**またはsqlx CLIを使う（推奨）:**

```bash
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

---

## 確認

**テーブルが作成されていることを確認:**

```bash
sqlite3 bookstore.db ".tables"
```

**期待される出力:**
```
categories              product_categories       sales
products                sale_headers             customers
```

**テーブル構造を確認:**

```bash
sqlite3 bookstore.db ".schema products"
```

---

## 現在のプロジェクト構造

```
BookStore-Rust/
├── .env                 ✅ 環境変数（DATABASE_URL）
├── bookstore.db         ✅ SQLiteデータベース
├── migrations/
│   └── init.sql        ✅ テーブル定義SQL
├── src/
├── docs/
│   └── Steps/          # 各実装ステップのドキュメント
└── Cargo.toml
```

---

## トラブルシューティング

### エラー: `sqlite3: command not found`

**原因:** sqlite3コマンドがインストールされていない

**解決策:** sqlx CLIを使用する方法に変更してください

### エラー: `Database already exists`

**原因:** データベースが既に作成されている

**解決策:** 問題ありません。既存のデータベースを使用します

---

## 次のStep

Step 4: Rustコードでデータベース接続

---

**完了条件:**
- ✅ `.env` がプロジェクトルートに作成されている
- ✅ `migrations/init.sql` が作成されている
- ✅ `bookstore.db` が作成されている
- ✅ 6つのテーブルが作成されている
