<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](https://github.com/s00d/switchshuttle/blob/main/LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/github/issues/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/github/stars/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
  [![Donate](https://img.shields.io/badge/Donate-Donationalerts-ff4081?style=for-the-badge)](https://www.donationalerts.com/r/s00d88)

  <img src="https://raw.githubusercontent.com/s00d/switchshuttle/refs/heads/main/icons/logo-min.png" alt="SwitchShuttle Logo" width="200">
  
  # SwitchShuttle
  
  **🚀 グローバルホットキー付きクロスプラットフォームターミナルコマンドマネージャー**

  <img src="https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true" alt="SwitchShuttle Demo" width="600">
  
  <div class="locale">
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
- **💻 コマンドラインインターフェース** - CLIでターミナルから直接コマンドを実行
- **⚙️ 設定管理** - 削除せずに設定を有効/無効にする
- **🔄 スイッチコマンド** - バックグラウンド実行でシステム機能を切り替え
- **📊 モニターコマンド** - 視覚的インジケーター付きのリアルタイムシステムリソース監視

## 🚀 クイックスタート

### ダウンロードとインストール

#### オプション1: Homebrew (macOS - 推奨)
```bash
# Homebrew経由でインストール
brew tap s00d/switchshuttle
brew install --cask switchshuttle
```

#### オプション2: 手動ダウンロード
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

#### 🔄 スイッチコマンド

バックグラウンド実行でシステム機能を切り替え：

```json
{
  "name": "🔧 システム制御",
  "submenu": [
    {
      "name": "📶 WiFiを切り替え",
      "command": "networksetup -setairportpower en0 toggle",
      "switch": "networksetup -getairportpower en0 | grep -q 'On' && echo 'true' || echo 'false'"
    },
    {
      "name": "🔊 ブルートゥースを切り替え",
      "command": "blueutil -p toggle",
      "switch": "blueutil -p | grep -q '1' && echo 'true' || echo 'false'"
    },
    {
      "name": "🌙 ダークモードを切り替え",
      "command": "osascript -e 'tell app \"System Events\" to tell appearance preferences to set dark mode to not dark mode'",
      "switch": "osascript -e 'tell app \"System Events\" to tell appearance preferences to get dark mode'"
    }
  ]
}
```

**スイッチコマンド機能：**
- **バックグラウンド実行** - ターミナルを開かずにコマンドが静かに実行される
- **ステータスチェック** - 現在の状態を自動的に検出
- **視覚的フィードバック** - メニューで有効/無効の状態を表示
- **クロスプラットフォーム** - macOS、Windows、Linuxで動作

#### 📊 モニターコマンド

リアルタイム情報でシステムリソースとサービスを監視：

```json
{
  "name": "📊 システム監視",
  "submenu": [
    {
      "name": "💾 メモリ使用量",
      "command": "top -l 1 | head -n 10",
      "monitor": "memory",
      "icon": "🧠"
    },
    {
      "name": "💻 CPU負荷",
      "command": "top -l 1 | grep 'CPU usage'",
      "monitor": "cpu",
      "icon": "⚡"
    },
    {
      "name": "💾 ディスク容量",
      "command": "df -h | grep '/dev/'",
      "monitor": "disk",
      "icon": "💾"
    },
    {
      "name": "🌐 ネットワークステータス",
      "command": "ifconfig | grep -E 'inet |status:'",
      "monitor": "network",
      "icon": "🌐"
    }
  ]
}
```

**モニターコマンド機能：**
- **メニュー統合** - システムトレイメニューに監視ボタンを追加
- **コマンド実行** - メニューを開くときに監視コマンドを実行
- **データ表示** - メニューインターフェースでコマンド出力を直接表示
- **視覚的インジケーター** - メニュー内のアイコンとステータスインジケーター
- **クロスプラットフォーム** - macOS、Windows、Linuxで動作

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
| `enabled` | Boolean | この設定を読み込むかどうか | `true` |

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
| `switch` | String | ❌ | 現在の状態をチェックするコマンド（スイッチコマンド用） |
| `monitor` | String | ❌ | リアルタイムリソース追跡の監視タイプ |
| `icon` | String | ❌ | 視覚的識別のための絵文字アイコン |

### 設定管理

#### 設定の有効/無効

個別の設定ファイルを有効または無効にして、システムトレイメニューで利用可能なコマンドを制御できます。これは以下に役立ちます：

- **一時的な無効化** - 削除せずに設定を無効にする
- **テスト** - 開発中に設定を有効/無効にする
- **整理** - 複数の設定を保持するが、特定の設定のみを使用する

**ビジュアルエディターで：**
- 設定エディターを開く
- 「設定ステータス」セクションでトグルスイッチを使用
- 有効な設定は読み込まれ、メニューで利用可能になる
- 無効な設定は無視される

**JSON設定で：**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "マイコマンド",
  "enabled": true,
  "commands": [
    {
      "name": "サンプルコマンド",
      "command": "echo Hello World"
    }
  ]
}
```

| パラメータ | タイプ | デフォルト | 説明 |
|-----------|--------|------------|------|
| `enabled` | Boolean | `true` | この設定を読み込み、メニューで利用可能にするかどうか |

**注意：** `enabled`が`false`に設定されているか省略されている場合、設定は無視され、そのコマンドはシステムトレイメニューに表示されません。

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

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE)ファイルを参照してください。

## 🙏 謝辞

- 元の[Shuttle](https://github.com/fitztrev/shuttle)プロジェクトに触発されました
- クロスプラットフォームデスクトップアプリ用の[Tauri](https://tauri.app/)で構築
- UIは[Vue.js](https://vuejs.org/)で動作

## 📞 サポート

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)

---

<div align="center">
  <p>SwitchShuttleコミュニティが❤️で作成</p>
  <p>⭐ このリポジトリが役立つと思ったら星をつけてください！</p>
</div>
