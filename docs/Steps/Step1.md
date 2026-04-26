# Step 1: プロジェクト作成

**目的**: Rustプロジェクトの初期化とTraeで開く

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
```

---

## 手順

### 1. Git Bashを開く

### 2. プロジェクトを作成するディレクトリに移動

```bash
cd C:/Users/hello/Desktop/project/BookStore-Rust
```

### 3. Rustプロジェクトを初期化

```bash
cargo init
```

**実行結果:**
```
     Creating binary (application) package
warning: the name `BookStore-Rust` is not snake_case or kebab-case which is recommended for package names, consider `bookstore-rust`
note: see more `Cargo.toml` keys and their definitions and their documentation at https://doc.rust-lang.org/cargo/reference/manifest.html
```

**警告について:**
- パッケージ名が `BookStore-Rust` になっていますが、問題ありません
- `bookstore-rust` という名前が推奨されますが、そのままでOK

### 4. Traeでプロジェクトを開く

1. Traeを起動
2. `ファイル → フォルダーを開く`
3. `C:\Users\hello\Desktop\project\BookStore-Rust` を選択

---

## 確認

**以下が作成されていることを確認:**

```
BookStore-Rust/
├── .git/
├── .gitignore
├── Cargo.toml        ← プロジェクトルートにあること
├── src/
│   └── main.rs
├── docs/
│   └── Steps/        # 各実装ステップのドキュメント
└── Reference_proj/
```

**Traeでプロジェクトが開けていることを確認:**
- 左側のファイルツリーに `Cargo.toml` がプロジェクトルートに表示される
- `src/main.rs` の内容が見える
- rust-analyzer が動作している（変数の上に型ヒントが表示される）

---

## src/main.rs の初期内容

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## 次のStep

Step 2: 依存関係追加

---

**完了条件:**
- ✅ デフォルトツールチェーンが `stable-x86_64-pc-windows-gnu` になっている
- ✅ `Cargo.toml` が `BookStore-Rust/` 直下に作成されている
- ✅ Traeでプロジェクト全体が開けている
- ✅ rust-analyzer が動作している
