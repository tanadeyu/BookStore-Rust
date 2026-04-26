# Step 2: 依存関係追加

**目的**: Cargo.toml に依存関係を追加してビルドする

---

## ツールチェーン設定

**GNUツールチェーンを使用します:**

```bash
rustup default stable-x86_64-pc-windows-gnu
```

**確認:**
```bash
rustup show
```

期待される出力:
```
Default host: x86_64-pc-windows-gnu
active toolchain: stable-x86_64-pc-windows-gnu
```

---

## 依存関係の追加

### 1. Traeで `Cargo.toml` を開く

### 2. 依存関係を追加

**⚠️ 注意: `edition` を確認してください**

**問題:**
`cargo init` が古いバージョンの場合、`edition = "2024"` になっていることがあります。これは誤りです。

**正しい内容:**

```toml
[package]
name = "BookStore-Rust"
version = "0.1.0"
edition = "2021"    # ← 2024ではなく2021にすること！

[dependencies]
actix-web = "4.9"
actix-files = "0.6"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono", "uuid"] }
askama = "0.13"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4"] }
dotenv = "0.15"
```

### 3. 依存関係をダウンロードしてビルド

**Git Bash または Trae の統合ターミナルで実行:**

```bash
cd C:/Users/hello/Desktop/project/BookStore-Rust
cargo build
```

**期待される結果:**
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

---

## 確認

**ビルドが成功すること:**
```
Finished `dev` profile [unoptimized + debuginfo] target(s) in XX.XXs
```

**依存関係が正しくダウンロードされていること:**
- `Cargo.lock` が作成されている
- `target/` ディレクトリが作成されている

---

## 次のStep

Step 3: データベース作成

---

**完了条件:**
- ✅ デフォルトツールチェーンが `stable-x86_64-pc-windows-gnu` になっている
- ✅ `Cargo.toml` に正しく依存関係が追加されている（sqlxを使用）
- ✅ `cargo build` が成功する
- ✅ `Cargo.lock` が作成されている
