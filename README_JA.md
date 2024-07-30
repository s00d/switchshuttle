もちろん、SwitchShuttleの紹介を日本語に翻訳いたします。

---

![SwitchShuttle](https://raw.githubusercontent.com/s00d/switchshuttle/main/icons/logo.webp)
![intro](https://github.com/s00d/switchshuttle/blob/main/icons/intro.gif?raw=true)

[[English](README.md)] [[中文](README_ZH)] [[Русский](README_RU.md)] [[Deutsch](README_DE.md)] [[日本語](README_JA.md)]

## SwitchShuttle

SwitchShuttleは、事前定義されたコマンドをさまざまなターミナルアプリケーションで実行できるクロスプラットフォームのシステムトレイアプリケーションです。macOS、Windows、Linuxをサポートし、頻繁に使用するコマンドを管理および実行するためのシンプルでカスタマイズ可能な方法を提供します。

## 概要

SwitchShuttleは、[Shuttle](https://github.com/fitztrev/shuttle)アプリケーションの再構築および拡張版です。ShuttleがmacOSでコマンドショートカットを管理するためのシンプルで効果的な方法を提供するのに対し、SwitchShuttleは複数のオペレーティングシステムとターミナルエミュレータのサポートを提供し、拡張された設定機能とユーザーのカスタマイズオプションを備えています。

## 機能

- 複数のターミナルアプリケーションをサポート: iTerm, Terminal, Warp。
- 異なるモードでコマンドを実行: 現在のウィンドウ、新しいタブ、新しいウィンドウ。
- 起動時に自動起動を切り替える。
- トレイメニューから直接設定を編集。
- トレイメニューから設定フォルダを開く。
- コマンドの整理に役立つサブメニューをサポート。
- コマンドの動的入力をサポート。
- **新機能**: ホットキーでコンテキストメニューをトリガー。

## 設定

設定はユーザーの設定ディレクトリにあるJSONファイルに保存されます。デフォルトのパスは、LinuxとmacOSでは`~/.config/switch-shuttle/`、Windowsでは`C:\Users\<Username>\AppData\Roaming\switch-shuttle\`です。このディレクトリには複数の設定ファイルを保存でき、それぞれが異なるコマンドや設定のセットを表します。

設定ファイルの例は以下の通りです：

```json
{
  "terminal": "iterm",
  "launch_in": "current",
  "theme": "Homebrew",
  "title": "New tab",
  "menu_hotkey": "Ctrl+Shift+M",
  "commands": [
    {
      "name": "Command",
      "inputs": null,
      "command": null,
      "commands": null,
      "hotkey": null,
      "submenu": [
        {
          "name": "Example Command",
          "inputs": null,
          "command": "echo Hello, world!",
          "commands": null,
          "submenu": null,
          "hotkey": "Ctrl+Shift+E"
        },
        {
          "name": "Example Multi-Command with input",
          "inputs": {
            "key1": "default1",
            "key2": "default2"
          },
          "command": null,
          "commands": [
            "export MY_VAR=$(echo 'Step 1: [key1]')",
            "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)",
            "echo Step 3: Finalize && echo $RESULT"
          ],
          "submenu": null,
          "hotkey": "Ctrl+Shift+M"
        },
        {
          "name": "Example Submenu",
          "inputs": null,
          "command": null,
          "commands": null,
          "submenu": [
            {
              "name": "Subcommand 1",
              "inputs": null,
              "command": "echo Subcommand 1",
              "commands": null,
              "submenu": null,
              "hotkey": "Ctrl+Shift+S"
            },
            {
              "name": "Subcommand 2",
              "inputs": null,
              "command": "echo Subcommand 2",
              "commands": null,
              "submenu": null,
              "hotkey": null
            }
          ],
          "hotkey": null
        }
      ]
    }
  ]
}
```

### 設定パラメータ

| パラメータ   | タイプ            | 説明                                             | 有効な値                                        |
|--------------|-------------------|-------------------------------------------------|------------------------------------------------|
| terminal     | String            | 使用するターミナルアプリケーション               | "iterm", "terminal", "warp"                    |
| launch_in    | String            | コマンドを起動する場所                           | "current", "new_tab", "new_window"             |
| theme        | String            | 使用するテーマ（ターミナルがサポートする場合）   | テーマを表す任意の文字列                        |
| title        | String            | ターミナルウィンドウ/タブのタイトル              | 任意の文字列                                    |
| menu_hotkey  | String (Optional) | コンテキストメニューをトリガーするグローバルホットキー | 有効なホットキーの組み合わせ, 例: "Ctrl+Shift+M" |
| commands     | Array             | コマンド設定のリスト                             | コマンドパラメータを参照                        |

### コマンドパラメータ

| パラメータ  | タイプ            | 説明                                         | 有効な値                                        |
|-------------|-------------------|----------------------------------------------|------------------------------------------------|
| name        | String            | コマンドまたはサブメニューの名前             | 任意の文字列                                    |
| inputs      | Object (Optional) | 入力のキーとデフォルト値                     | {"key1": "default1", "key2": "default2"}       |
| command     | String (Optional) | 実行するコマンド（コマンドの場合）           | コマンドを表す任意の文字列                      |
| commands    | Array (Optional)  | 順次実行するコマンドのリスト                 | 各コマンドが文字列である任意の配列              |
| submenu     | Array (Optional)  | サブコマンドのリスト（サブメニューの場合）   | コマンドパラメータを参照                        |
| hotkey      | String (Optional) | コマンドをトリガーするグローバルホットキー   | 有効なホットキーの組み合わせ, 例: "Ctrl+Shift+E" |

### コマンド実行ロジック

SwitchShuttleは、`command`パラメータを使用して単一のコマンドを定義する方法、`commands`パラメータを使用してコマンドのリストを定義する方法、またはその両方をサポートします。`command`と`commands`の両方が指定されている場合、単一のコマンドが最初に実行され、その後にリスト内のコマンドが順次実行されます。

#### 実行フローの例

1. **単一コマンド**: `command`のみが指定されている場合、そのコマンドが実行されます。
2. **複数コマンド**: `commands`のみが指定されている場合、リスト内の各コマンドが順次実行されます。
3. **コマンドとコマンドリストの両方**: `command`と`commands`の両方が指定されている場合、単一のコマンドが最初に実行され、その後リスト内の各コマンドが実行されます。

### 動的入力

SwitchShuttleでは、コマンドの実行前にユーザーからの入力を求める動的入力を定義できます。`inputs`パラメータを使用してコマンド設定に入力を定義できます。

#### 入力付き設定の例

```json
{
  "name": "Example Multi-Command with input",
  "inputs": {
    "key1": "default1",
    "key2": "default2"
  },
  "command": null,
  "commands": [
    "export MY_VAR=$(echo 'Step 1: [key1]')",
    "RESULT=$(echo 'Step 2: [key2]' && echo $MY_VAR)",
    "echo Step 3: Finalize && echo $RESULT"
  ],
  "submenu": null,
  "hotkey": "Ctrl+Shift+M"
}
```

### ホットキー

コマンド設定に`hotkey`パラメータを追加することで、コマンドにグローバルホットキーを割り

当てることができます。ホットキーの組み合わせは、修飾キー（Ctrl、Shift、Alt、Win）とキー（A-Z、0-9など）を組み合わせた形式でなければなりません。例えば、コマンドに「Ctrl+Shift+E」をホットキーとして設定するには：

```json
{
  "name": "Example Command",
  "command": "echo Hello, world!",
  "submenu": null,
  "hotkey": "Ctrl+Shift+E",
  "commands": null
}
```

ホットキーパラメータはオプションです。指定しない場合、そのコマンドにはグローバルホットキーが割り当てられません。

### ホットキーの使用方法

1. **ホットキーの割り当て**: 設定ファイルを編集して、トリガーしたいコマンドに`hotkey`パラメータを追加します。
2. **ホットキーの使用**: アプリケーションを再起動した後、設定したホットキーを使用して、フォーカスしているアプリケーションに関係なく対応するコマンドをトリガーします。

## 使用方法

1. **設定の編集**: トレイアイコンを右クリックして「設定の編集」を選択し、デフォルトのエディタで設定ファイルを開きます。必要に応じて設定を変更します。
2. **設定フォルダを開く**: トレイアイコンを右クリックして「設定フォルダを開く」を選択し、ファイルエクスプローラで設定ディレクトリを開きます。
3. **ログイン時に自動起動を切り替える**: トレイアイコンを右クリックして「ログイン時に自動起動を切り替える」を選択し、アプリケーションがログイン時に起動するかどうかを切り替えます。
4. **コマンドを実行**: トレイアイコンを左クリックしてメニューから実行したいコマンドを選択します。指定されたターミナルアプリケーションでコマンドが実行されます。

### サブメニューの作成

サブメニューを作成するには、`command`フィールドを`null`に設定し、`submenu`フィールドにサブコマンドのリストを提供します。サブコマンドも独自のサブメニューを持つことができ、ネストされたメニューが可能です。

## アプリケーションのビルド

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites/)

### 手順

1. リポジトリをクローンする:
   ```sh
   git clone https://github.com/yourusername/switchshuttle.git
   cd switchshuttle
   ```

2. アプリケーションをビルドする:
   ```sh
   cargo build --release
   ```

3. アプリケーションを実行する:
   ```sh
   cargo run
   ```

## ダウンロード

最新バージョンのSwitchShuttleは[GitHub Releases](https://github.com/s00d/switchshuttle/releases)ページからダウンロードできます。

### macOSの場合

アプリケーションを実行する前に、サインが必要な場合があります。以下の手順でサインを行ってください：

1. バイナリを実行可能にする:

```bash
chmod +x /Applications/switch-shuttle.app
```

2. 拡張属性をクリアし、バイナリにサインする:

```bash
xattr -cr /Applications/switch-shuttle.app && codesign --force --deep --sign - /Applications/switch-shuttle.app
```

## コントリビューション

貢献は大歓迎です！GitHubでプルリクエストを提出するか、イシューをオープンしてください。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

---

SwitchShuttleを使用してターミナルコマンドを簡単に管理しましょう！

---

この翻訳が役立つことを願っています。