<div align="center">
  
  [![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/s00d/switchshuttle)
  [![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](https://github.com/s00d/switchshuttle/blob/main/LICENSE)
  [![GitHub release](https://img.shields.io/github/v/release/s00d/switchshuttle?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub downloads](https://img.shields.io/github/downloads/s00d/switchshuttle/total?style=for-the-badge)](https://github.com/s00d/switchshuttle/releases)
  [![GitHub issues](https://img.shields.io/badge/github-issues-orange?style=for-the-badge)](https://github.com/s00d/switchshuttle/issues)
  [![GitHub stars](https://img.shields.io/badge/github-stars-yellow?style=for-the-badge)](https://github.com/s00d/switchshuttle/stargazers)
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
- **📅 スケジュールコマンド** - cron式によるタスクの自動化
- **🎯 テンプレートシステム** - 一般的なワークフロー用の事前構築されたコマンドテンプレート

## 🖥️ ユーザーインターフェース概要

SwitchShuttleは、いくつかの主要コンポーネントを持つ現代的な直感的なインターフェースを提供します：

### 🎛️ メインインターフェースコンポーネント

#### 1. **設定エディター**
- **ビジュアルJSONエディター** - シンタックスハイライトとバリデーション付きの設定編集
- **テンプレートシステム** - 一般的なワークフロー用の事前構築されたコマンドテンプレートのインポート
- **リアルタイムバリデーション** - 設定エラーの即座のフィードバック
- **自動保存** - 入力時に変更が自動的に保存される
- **設定管理** - 削除せずに設定を有効/無効にする
- **検索とフィルター** - 特定の設定を素早く見つける
- **設定の複製** - テスト用に既存の設定のコピーを作成

#### 2. **コマンド管理**
- **コマンドビルダー** - ビジュアルフォームインターフェースでのコマンド作成
- **ホットキー設定** - 即座のコマンド実行のためのグローバルショートカット設定
- **アイコン選択** - より良い視覚的整理のための絵文字アイコン選択
- **入力フィールド** - インタラクティブコマンド用の動的入力プロンプト設定
- **ネストしたサブメニュー** - 階層的構造でのコマンド整理
- **コマンドバリデーション** - コマンドシンタックスのリアルタイムバリデーション

#### 3. **設定パネル**
- **ターミナル選択** - お好みのターミナルアプリケーションの選択
- **起動モード** - コマンド実行方法の設定（現在/新しいタブ/新しいウィンドウ）
- **テーマ設定** - アプリケーション外観のカスタマイズ
- **自動起動設定** - システム起動時の有効/無効設定
- **グローバルホットキー設定** - メニューアクセス用のシステム全体ショートカット設定

#### 4. **システムトレイメニュー**
- **クイックアクセス** - 即座のコマンドアクセスのためのトレイアイコンの右クリック
- **ステータスインジケーター** - スイッチコマンドとモニタリングの視覚的フィードバック
- **ネストしたメニュー** - 簡単なナビゲーションのための整理されたコマンド階層
- **グローバルホットキー** - 即座のコマンド実行のためのキーボードショートカット
- **リアルタイムモニタリング** - ライブシステムリソースインジケーター

### 🎨 インターフェース機能

#### **ビジュアルコマンドビルダー**
```json
{
  "name": "🚀 開発サーバーを起動",
  "command": "npm run dev",
  "hotkey": "Ctrl+Shift+D",
  "icon": "🚀",
  "background": false,
  "inputs": {
    "port": "3000",
    "host": "localhost"
  }
}
```

#### **テンプレートシステム**
SwitchShuttleには一般的な開発ワークフロー用の事前構築されたテンプレートが含まれています：

- **開発** - Git操作、ビルドツール、テスト
- **DevOps** - Docker、Kubernetes、サーバー管理
- **データベース** - MySQL、PostgreSQL、MongoDB操作
- **クラウド** - AWS、Azure、Google Cloudコマンド
- **セキュリティ** - ネットワークスキャン、脆弱性評価
- **モニタリング** - システムリソース、ログ、メトリクス
- **ユーティリティ** - ファイル操作、システムツール
- **スケジューラー** - Cronジョブと自動化タスク

#### **スマートな整理**
- **ネストしたサブメニュー** - 論理的なグループでのコマンド整理
- **アイコンサポート** - 絵文字アイコンによる視覚的識別
- **ホットキー管理** - 即座のアクセスのためのグローバルショートカット
- **ステータスインジケーター** - スイッチコマンドのリアルタイムフィードバック
- **検索機能** - コマンドの迅速な発見

## 🔒 セキュリティマネージャー

SwitchShuttleには、潜在的に有害なコマンドからシステムを保護し、コマンド実行を細かく制御できる包括的なセキュリティマネージャーが含まれています。

### 🛡️ セキュリティ機能

#### **コマンド検証**
- **長さ制限**: 最大コマンド長（1000文字）と入力長（500文字）で、長すぎるコマンドを防止
- **ブロックされたコマンド**: 実行してはいけない危険なコマンドのリストを定義
- **疑わしいパターン**: 潜在的に有害なコマンドパターンを検出・ブロックするための正規表現パターンを使用
- **リアルタイム検証**: コマンドは編集時に検証され、安全性を確保

#### **セキュリティ設定**
- **セキュリティの有効/無効**: 必要に応じてセキュリティ機能を切り替え
- **カスタムブロックリスト**: ブロックされたコマンドリストに特定のコマンドを追加
- **パターンマッチング**: 疑わしいコマンド構造を検出するための正規表現パターンを定義
- **長さ制限**: コマンドとユーザー入力の最大長を設定

#### **動作原理**
1. **エディター検証**: SecurityManagerが設定エディターでコマンドを保存前に検証
2. **パターンマッチング**: コマンドがブロックされたパターンと疑わしい正規表現パターンに対してチェック
3. **長さ検証**: コマンドと入力が最大長制限に対して検証
4. **ブロックリストチェック**: コマンドがユーザー定義のブロックされたコマンドリストと比較
5. **安全な設定**: 検証された設定のみが保存と使用を許可



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

1. **設定エディターを開く** - システムトレイメニューの「設定を編集」をクリック
2. **ターミナルを選択** - お好みのターミナルアプリケーションを選択
3. **コマンドを追加** - ビジュアルエディターを使用するかテンプレートをインポート
4. **ホットキーを設定** - クイックアクセスのためのグローバルショートカットを設定
5. **保存して再起動** - コマンドがトレイメニューで利用可能になります

### インターフェースのステップバイステップガイド

#### **ステップ1: 設定エディター**
- SwitchShuttleを開き「エディター」タブに移動
- ターミナルアプリケーションを選択（iTerm、Terminal、Warpなど）
- 起動モードを設定（現在のウィンドウ、新しいタブ、または新しいウィンドウ）
- メニューアクセス用のグローバルホットキーを設定

#### **ステップ2: コマンドの追加**
- 「コマンドを追加」をクリックして新しいコマンドを作成
- コマンドの詳細を入力：
  - **名前**: コマンドの表示名
  - **コマンド**: 実行する実際のターミナルコマンド
  - **ホットキー**: グローバルショートカット（オプション）
  - **アイコン**: 視覚的識別のための絵文字アイコン
  - **バックグラウンド**: バックグラウンドで実行するかどうか
  - **入力**: インタラクティブコマンド用の動的入力フィールド

#### **ステップ3: テンプレートの使用**
- 「テンプレートをインポート」をクリックして事前構築されたコマンドコレクションにアクセス
- 開発、DevOps、データベースなどのカテゴリを閲覧
- 必要なテンプレートを選択してインポート
- 必要に応じてインポートされたコマンドをカスタマイズ

#### **ステップ4: システムトレイアクセス**
- SwitchShuttleトレイアイコンを右クリック
- 整理されたコマンドメニューを閲覧
- 即座のコマンド実行のためのグローバルホットキーを使用
- リアルタイムインジケーターでシステムステータスを監視

#### **ステップ5: 高度な機能**
- **スイッチコマンド**: 視覚的ステータスインジケーター付きのシステム機能切り替え
- **モニターコマンド**: システムリソースのリアルタイム監視
- **スケジュールコマンド**: cron式によるタスクの自動化
- **ネストしたメニュー**: 階層的構造でのコマンド整理

## 💻 コマンドラインインターフェース（CLI）

SwitchShuttleは、GUIを開かずにコマンドを迅速に実行するための強力なコマンドラインインターフェースも提供します。

### CLIの使用

#### コマンドの実行
```bash
# コマンドIDで実行
switch-shuttle cmd_8

# コマンド名で実行（大文字小文字を区別しない）
switch-shuttle "例のコマンド"
```

#### すべてのコマンドをリスト
```bash
# ID付きで利用可能なすべてのコマンドを表示
switch-shuttle --list
# または
switch-shuttle -l
```

#### コマンドの検索
```bash
# 特定のテキストを含むコマンドを検索
switch-shuttle --search "git"
# または
switch-shuttle -s "docker"
```

### 異なるオペレーティングシステムでのCLI実行

#### macOS
```bash
# Homebrew経由でインストールした場合
switch-shuttle --list

# 手動でインストールした場合
/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle --list

# より簡単なアクセスのためのエイリアスを作成
echo 'alias switch-shuttle="/Applications/switch-shuttle.app/Contents/MacOS/SwitchShuttle"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows
```bash
# インストーラー経由でインストールした場合
"C:\Program Files\SwitchShuttle\switch-shuttle.exe" --list

# wingetまたはchocolatey経由でインストールした場合
switch-shuttle --list

# より簡単なアクセスのためにPATHに追加
# システムPATHに"C:\Program Files\SwitchShuttle"を追加
```

#### Linux
```bash
# パッケージマネージャー経由でインストールした場合
switch-shuttle --list

# 手動でインストールした場合
./switch-shuttle --list

# 実行可能にしてPATHに追加
chmod +x switch-shuttle
sudo mv switch-shuttle /usr/local/bin/
```

### CLIの例

```bash
# クイックGit操作
switch-shuttle "git status"
switch-shuttle "git pull"

# 開発ワークフロー
switch-shuttle "npm run dev"
switch-shuttle "docker-compose up"

# 利用可能なすべてのコマンドをリスト
switch-shuttle --list

# データベース関連のコマンドを検索
switch-shuttle --search "database"
```

### CLI機能

- **🚀 高速実行** - ターミナルから即座にコマンドを実行
- **🔍 スマート検索** - IDまたは名前でコマンドを検索
- **📋 コマンドリスト** - 利用可能なすべてのコマンドを表示
- **⚡ GUI不要** - 自動化とスクリプトに最適
- **🔄 実行後終了** - クリーンなターミナル体験

## 📋 設定ガイド

### 基本構造

SwitchShuttleはJSON設定ファイルを使用し、以下に保存されます：
- **macOS/Linux**: `~/.config/switch-shuttle/`
- **Windows**: `C:\Users\<ユーザー名>\AppData\Roaming\switch-shuttle\`

### 簡単な例

```json
{
  "terminal": "iterm",
  "launch_in": "new_tab",
  "title": "私のコマンド",
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

ユーザー入力が必要なインタラクティブなコマンドを作成：

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
  "background": true,
  "hotkey": "Ctrl+Shift+F"
}
```

#### ⏰ スケジュールコマンド（Cron）

cron式を使用してコマンドを自動実行するようにスケジュール：

```json
{
  "name": "🔄 自動バックアップ",
  "commands": [
    "rsync -av /source/ /backup/"
  ],
  "scheduler": "0 2 * * *",
  "background": true,
  "hotkey": "Ctrl+Shift+B"
}
```

**Cron式フォーマット:**
スケジューラーは6フィールドの標準cron式を使用：`秒 分 時 日 月 曜日`

**一般的なCron例:**
- `"0 0 * * * *"` - 毎時0分
- `"0 0 2 * * *"` - 毎日午前2時
- `"0 30 9 * * 1-5"` - 平日午前9時30分
- `"0 0 12 * * 1"` - 毎週月曜日正午
- `"0 0 0 1 * *"` - 毎月1日
- `"0 */15 * * * *"` - 15分ごと
- `"0 0 0 * * 0"` - 毎週日曜日深夜

**スケジューラー機能:**
- **バックグラウンド実行** - ターミナルを開かずにコマンドが静かに実行
- **Cronサポート** - 完全なcron式パースと実行
- **エラーハンドリング** - cronパースエラー時の適切なフォールバック
- **クロスプラットフォーム** - macOS、Windows、Linuxで動作

#### 🖥️ バックグラウンド実行

コマンドの実行方法を制御 - ConsolePoolを使用したバックグラウンドまたは通常のターミナル実行：

```json
{
  "name": "🚀 サーバーを起動",
  "commands": [
    "npm run dev"
  ],
  "background": true,
  "hotkey": "Ctrl+Shift+S"
}
```

**バックグラウンド実行オプション:**
- `"background": true` - ConsolePoolを使用した実行（バックグラウンド）
- `"background": false` - 通常のターミナル実行
- `"background": null`または省略 - コマンドタイプに基づく自動検出

#### 📁 ネストしたサブメニュー

階層的メニューでコマンドを整理：

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
      "name": "🔊 Bluetoothを切り替え",
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

**スイッチコマンド機能:**
- **バックグラウンド実行** - ターミナルを開かずにコマンドが静かに実行
- **ステータスチェック** - 現在の状態を自動的に検出
- **視覚的フィードバック** - メニューで有効/無効ステータスを表示
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

**モニターコマンド機能:**
- **メニュー統合** - システムトレイメニューに監視ボタンを追加
- **コマンド実行** - メニューが開いたときに監視コマンドを実行
- **データ表示** - メニューインターフェースでコマンド出力を直接表示
- **視覚的インジケーター** - メニューのアイコンとステータスインジケーター
- **クロスプラットフォーム** - macOS、Windows、Linuxで動作

## ⚙️ 設定リファレンス

### メイン設定

| パラメータ | タイプ | 説明 | デフォルト |
|-----------|--------|------|------------|
| `terminal` | String | 使用するターミナルアプリケーション | `"terminal"` |
| `launch_in` | String | コマンドを起動する場所 | `"current"` |
| `theme` | String | ターミナルテーマ（サポートされている場合） | - |
| `title` | String | ウィンドウ/タブタイトル | - |
| `menu_hotkey` | String | メニューを開くグローバルホットキー | - |
| `commands` | Array | コマンド設定のリスト | `[]` |
| `enabled` | Boolean | この設定を読み込むかどうか | `true` |

### ターミナルオプション

| ターミナル | macOS | Windows | Linux |
|------------|-------|---------|-------|
| `iterm` | ✅ | ❌ | ❌ |
| `terminal` | ✅ | ✅ | ✅ |
| `warp` | ✅ | ❌ | ❌ |
| `alacritty` | ✅ | ✅ | ✅ |
| `hyper` | ✅ | ✅ | ✅ |

### 起動モード

| モード | 説明 |
|--------|------|
| `current` | 現在のターミナルウィンドウで実行 |
| `new_tab` | 新しいタブを開いて実行 |
| `new_window` | 新しいウィンドウを開いて実行 |

### コマンド設定

| パラメータ | タイプ | 必須 | 説明 |
|-----------|--------|------|------|
| `name` | String | ✅ | コマンドの表示名 |
| `commands` | Array | ❌ | 実行する複数のコマンド |
| `submenu` | Array | ❌ | ネストしたサブコマンド |
| `switch` | String | ❌ | スイッチ状態をチェックするコマンド（true/falseを返す） |
| `monitor` | String | ❌ | 監視用の表示値を取得するコマンド |
| `inputs` | Object | ❌ | 動的入力フィールド |
| `hotkey` | String | ❌ | グローバルホットキーショートカット |
| `icon` | String | ❌ | 視覚的識別のための絵文字アイコン |
| `background` | Boolean | ❌ | バックグラウンドで実行（ConsolePool）または通常のターミナル |
| `scheduler` | String | ❌ | スケジュール実行のためのcron式 |

### 設定管理

#### 設定の有効/無効

個別の設定ファイルを有効または無効にして、システムトレイメニューで利用可能なコマンドを制御できます。これは以下に役立ちます：

- **一時的な無効化** - 削除せずに設定を無効化
- **テスト** - 開発中の設定の有効/無効化
- **整理** - 複数の設定を保持し、特定のもののみ使用

**ビジュアルエディターで:**
- 設定エディターを開く
- 「設定ステータス」セクションのトグルスイッチを使用
- 有効な設定は読み込まれ、メニューで利用可能
- 無効な設定は無視される

**JSON設定で:**
```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "title": "私のコマンド",
  "enabled": true,
  "commands": [
    {
      "name": "例のコマンド",
      "command": "echo Hello World"
    }
  ]
}
```

| パラメータ | タイプ | デフォルト | 説明 |
|-----------|--------|------------|------|
| `enabled` | Boolean | `true` | この設定を読み込み、メニューで利用可能にするかどうか |

**注意:** `enabled`が`false`に設定されているか省略されている場合、設定は無視され、そのコマンドはシステムトレイメニューに表示されません。

## 🎯 ユースケース

### 👨‍💻 開発者
- **クイックプロジェクトナビゲーション** - 異なるプロジェクトに即座にジャンプ
- **ビルドとテストワークフロー** - ワンクリック開発サイクル
- **Docker管理** - ホットキーでコンテナの開始/停止
- **Git操作** - 手元の一般的なgitコマンド
- **開発サーバー管理** - 開発サーバーの開始/停止
- **コード品質ツール** - リンター、フォーマッター、テストの実行

### 🛠️ DevOpsエンジニア
- **サーバー管理** - SSH接続とサーバーコマンド
- **監視ツール** - ログとメトリクスへのクイックアクセス
- **デプロイメントスクリプト** - 自動化されたデプロイメントワークフロー
- **データベース操作** - 一般的なデータベースコマンド
- **コンテナオーケストレーション** - DockerとKubernetes管理
- **インフラ監視** - システムリソーストラッキング

### 🎨 デザイナー
- **アセット最適化** - 画像処理と最適化
- **デザインシステムツール** - コンポーネント生成と更新
- **プロトタイプサーバー** - クイックデザインサーバー起動
- **デザインツール自動化** - バッチ処理とワークフロー

### 🔧 システム管理者
- **システム監視** - リアルタイムリソース監視
- **サービス管理** - システムサービスの開始/停止
- **バックアップ自動化** - スケジュールされたバックアップ操作
- **ネットワークツール** - ネットワーク診断と設定
- **セキュリティツール** - 脆弱性スキャンと評価
- **メンテナンスタスク** - システムクリーンアップと最適化

## 🔧 ソースからのビルド

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install) (最新の安定版)
- [Node.js](https://nodejs.org/) (v16以上)
- [pnpm](https://pnpm.io/) (推奨パッケージマネージャー)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites/)

### ビルド手順

```bash
# リポジトリをクローン
git clone https://github.com/s00d/switchshuttle.git
cd switchshuttle

# 依存関係をインストール
pnpm install

# 開発モード
pnpm run tauri dev

# 本番用ビルド
pnpm run tauri build
```

### プラットフォーム固有の注意事項

#### macOS
```bash
# 署名の問題が発生した場合
chmod +x /Applications/switch-shuttle.app
xattr -cr /Applications/switch-shuttle.app
codesign --force --deep --sign - /Applications/switch-shuttle.app
```

#### Windows
```bash
# RustとNode.jsをインストール
# 上記のビルド手順に従う
```

#### Linux
```bash
# システム依存関係をインストール
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# 上記のビルド手順に従う
```

## 🤝 貢献

貢献を歓迎します！以下の方法でお手伝いできます：

1. **フォーク** リポジトリをフォーク
2. **作成** 機能ブランチを作成 (`git checkout -b feature/amazing-feature`)
3. **コミット** 変更をコミット (`git commit -m 'Add amazing feature'`)
4. **プッシュ** ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. **オープン** プルリクエストを開く

### 開発ガイドライン

- 既存のコードスタイルに従う
- 新機能のテストを追加
- 必要に応じてドキュメントを更新
- クロスプラットフォーム互換性を確保
- パッケージ管理にpnpmを使用

### 開発セットアップ

```bash
# 依存関係をインストール
pnpm install

# 開発サーバーを起動
pnpm run tauri dev

# 型チェックを実行
pnpm run type-check

# 本番用ビルド
pnpm run tauri build
```

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](https://github.com/s00d/switchshuttle/blob/main/LICENSE)ファイルを参照してください。

## 🙏 謝辞

- 元の[Shuttle](https://github.com/fitztrev/shuttle)プロジェクトに触発
- クロスプラットフォームデスクトップアプリ用に[Tauri](https://tauri.app/)で構築
- UI powered by [Vue.js](https://vuejs.org/)
- [Tailwind CSS](https://tailwindcss.com/)でスタイリング

## 📞 サポート

- **Issues**: [GitHub Issues](https://github.com/s00d/switchshuttle/issues)
- **Releases**: [GitHub Releases](https://github.com/s00d/switchshuttle/releases)
- **Documentation**: [GitHub Wiki](https://github.com/s00d/switchshuttle/wiki)

---

<div align="center">
  <p>SwitchShuttleコミュニティによって❤️で作成</p>
  <p>⭐ このリポジトリが役立つと思ったら星をつけてください！</p>
</div>
