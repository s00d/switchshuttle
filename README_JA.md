<div align="center">
  <img src="https://raw.githubusercontent.com/s00d/switchshuttle/refs/heads/main/icons/logo-min.png" alt="SwitchShuttle Logo" width="200">
  
  # SwitchShuttle
  
  **🚀 グローバルホットキー付きクロスプラットフォームターミナルコマンドマネージャー**
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue.svg)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
  [![Release](https://img.shields.io/github/v/release/s00d/switchshuttle)](https://github.com/s00d/switchshuttle/releases)
  [![Downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total)](https://github.com/s00d/switchshuttle/releases)
  
  <img src="https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true" alt="SwitchShuttle Demo" width="600">
  
  <div>
    <a href="README.md">🇺🇸 English</a> • 
    <a href="README_ZH.md">🇨🇳 中文</a> • 
    <a href="README_RU.md">🇷🇺 Русский</a> • 
    <a href="README_DE.md">🇩🇪 Deutsch</a> • 
    <a href="README_JA.md">🇯🇵 日本語</a>
  </div>
</div>

---

## ✨ SwitchShuttleとは？

SwitchShuttleは、ターミナルコマンドの管理と実行方法を革新する強力なクロスプラットフォームシステムトレイアプリケーションです。最新技術（Rust + Tauri + Vue.js）で構築され、最も頻繁に使用するターミナル操作の整理、カスタマイズ、迅速なアクセスのためのエレガントなインターフェースを提供します。

### 🎯 主な機能

- **🖥️ マルチプラットフォームサポート** - macOS、Windows、Linuxでシームレスに動作
- **⚡ グローバルホットキー** - キーボードショートカットでどこからでも即座にコマンドを実行
- **🎨 複数ターミナルサポート** - iTerm、Terminal、Warp、Alacritty、Hyperなど
- **📁 スマートな整理** - コマンドの整理のためのネストしたサブメニューを作成
- **🔧 動的入力** - ユーザー入力が必要なコマンドのためのインタラクティブなプロンプト
- **🔄 複数の実行モード** - 現在のウィンドウ、新しいタブ、または新しいウィンドウで実行
- **🚀 自動起動** - システム起動時に起動して即座にアクセス可能
- **🎨 モダンUI** - Vue.jsで構築された美しく直感的なインターフェース

## 🚀 クイックスタート

### ダウンロードとインストール

1. **ダウンロード** [GitHub Releases](https://github.com/s00d/switchshuttle/releases)からお使いのプラットフォーム用の最新リリースをダウンロード
2. **インストール** アプリケーションをインストール
3. **起動** SwitchShuttleを起動 - システムトレイに表示されます
4. **右クリック** トレイアイコンを右クリックしてメニューにアクセス

### 初期設定

1. **設定を編集** → デフォルトエディタで設定ファイルを開きます
2. **コマンドを追加** JSON形式を使用して（以下の例を参照）
3. **保存して再起動** アプリケーションを保存して再起動
4. **楽しむ** 整理されたコマンドショートカットをお楽しみください！

## 📋 設定ガイド

### 基本構造

SwitchShuttleはJSON設定ファイルを使用し、以下に保存されます：
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\switch-shuttle\`

### 簡単な例

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "マイコマンド",
  "commands": [
    {
      "name": "🚀 開発サーバーを起動",
      "command": "npm run dev",
      "hotkey": "Ctrl+Shift+D"
    },
    {
      "name": "📦 依存関係をインストール",
      "command": "npm install",
      "hotkey": "Ctrl+Shift+I"
    },
    {
      "name": "🔧 開発ツール",
      "submenu": [
        {
          "name": "🧪 テストを実行",
          "command": "npm test",
          "hotkey": "Ctrl+Shift+T"
        },
        {
          "name": "📊 プロジェクトをビルド",
          "command": "npm run build",
          "hotkey": "Ctrl+Shift+B"
        }
      ]
    }
  ]
}
```

### 高度な機能

#### 🔧 動的入力

ユーザー入力にプロンプトを表示するインタラクティブなコマンドを作成：

```json
{
  "name": "📝 新しいコンポーネントを作成",
  "inputs": {
    "componentName": "MyComponent",
    "componentType": "functional"
  },
  "commands": [
    "mkdir -p src/components/[componentName]",
    "touch src/components/[componentName]/index.tsx",
    "echo 'import React from \"react\";' > src/components/[componentName]/index.tsx",
    "echo 'export const [componentName] = () => <div>[componentName]</div>;' >> src/components/[componentName]/index.tsx"
  ],
  "hotkey": "Ctrl+Shift+N"
}
```

#### 🔄 複数コマンド

コマンドシーケンスを実行：

```json
{
  "name": "🔄 完全な開発サイクル",
  "commands": [
    "git pull origin main",
    "npm install",
    "npm run lint",
    "npm run test",
    "npm run build"
  ],
  "hotkey": "Ctrl+Shift+F"
}
```

#### 📁 ネストしたサブメニュー

階層メニューでコマンドを整理：

```json
{
  "name": "🐳 Docker操作",
  "submenu": [
    {
      "name": "🚀 サービスを起動",
      "submenu": [
        {
          "name": "🏗️ 開発",
          "command": "docker-compose -f docker-compose.dev.yml up -d"
        },
        {
          "name": "🏭 本番",
          "command": "docker-compose -f docker-compose.prod.yml up -d"
        }
      ]
    },
    {
      "name": "🛑 すべて停止",
      "command": "docker-compose down"
    }
  ]
}
```

## ⚙️ 設定リファレンス

### メイン設定

| パラメータ | タイプ | 説明 | デフォルト |
|-----------|--------|------|------------|
| `terminal` | String | 使用するターミナルアプリケーション | `"terminal"` |
| `launch_in` | String | コマンドを起動する場所 | `"current"` |
| `theme` | String | ターミナルテーマ（サポートされている場合） | - |
| `title` | String | ウィンドウ/タブのタイトル | - |
| `menu_hotkey` | String | メニューを開くためのグローバルホットキー | - |
| `commands` | Array | コマンド設定のリスト | `[]` |

### ターミナルオプション

| ターミナル | macOS | Windows | Linux |
|-----------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### 起動モード

| モード | 説明 |
|-------|------|
| `current` | 現在のターミナルウィンドウで実行 |
| `new_tab` | 新しいタブを開いて実行 |
| `new_window` | 新しいウィンドウを開いて実行 |

### コマンド設定

| パラメータ | タイプ | 必須 | 説明 |
|-----------|--------|------|------|
| `name` | String | ✅ | コマンドの表示名 |
| `command` | String | ❌ | 実行する単一コマンド |
| `commands` | Array | ❌ | 実行する複数コマンド |
| `submenu` | Array | ❌ | ネストしたサブコマンド |
| `inputs` | Object | ❌ | 動的入力フィールド |
| `hotkey` | String | ❌ | グローバルホットキー |

## 🎯 ユースケース

### 👨‍💻 開発者
- **クイックプロジェクトナビゲーション** - 異なるプロジェクトに即座にジャンプ
- **ビルドとテストワークフロー** - ワンクリックで開発サイクル
- **Docker管理** - ホットキーでコンテナの開始/停止
- **Git操作** - 頻繁に使用するGitコマンドを手元に

### 🛠️ DevOpsエンジニア
- **サーバー管理** - SSH接続とサーバーコマンド
- **監視ツール** - ログとメトリクスへのクイックアクセス
- **デプロイメントスクリプト** - 自動化されたデプロイメントワークフロー
- **データベース操作** - 頻繁に使用するデータベースコマンド

### 🎨 デザイナー
- **アセット最適化** - 画像処理と最適化
- **デザインシステムツール** - コンポーネント生成と更新
- **プロトタイプサーバー** - クイックデザインサーバー起動

## 🔧 ソースからのビルド

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install) (最新の安定版)
- [Node.js](https://nodejs.org/) (v16以上)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### ビルド手順

```bash
# リポジトリをクローン
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# 依存関係をインストール
npm install

# 開発モード
npm run tauri dev

# 本番用にビルド
npm run tauri build
```

### プラットフォーム固有の注意事項

#### macOS
```bash
# 署名の問題が発生した場合
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## 🤝 貢献

貢献を歓迎します！以下の方法でお手伝いできます：

1. **フォーク** リポジトリをフォーク
2. **作成** 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. **コミット** 変更をコミット (`git commit -m 'Add amazing feature'`)
4. **プッシュ** ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. **プルリクエスト** プルリクエストを開く

### 開発ガイドライン

- 既存のコードスタイルに従う
- 新機能にテストを追加
- 必要に応じてドキュメントを更新
- クロスプラットフォーム互換性を確保

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 🙏 謝辞

- 元の[Shuttle](https://github.com/fitztrev/shuttle)プロジェクトに触発されました
- クロスプラットフォームデスクトップアプリ用の[Tauri](https://tauri.app/)で構築
- UIは[Vue.js](https://vuejs.org/)で動作

## 📞 サポート

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Discussions**: [GitHub Discussions](https://github.com/s00d/switchshuttle/discussions)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>SwitchShuttleコミュニティが❤️で作成</p>
  <p>⭐ このリポジトリが役立つと思ったら星をつけてください！</p>
</div>
