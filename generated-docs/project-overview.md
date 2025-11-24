Last updated: 2025-11-25

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) テキストをリアルタイムで音楽として再生するWindows向けCLIツールです。
- シンプルなコマンドライン操作で、"cde"のようなMMLをドレミの音階として即座に出力します。
- Rustで開発されており、低レイテンシなオーディオ再生とバックグラウンドでの演奏機能を提供します。

## 技術スタック
- フロントエンド: コマンドラインインターフェース (CLI) によるユーザー操作。`src/cli.rs`で引数のパースを定義し、ユーザーからの入力MMLを受け付けます。
- 音楽・オーディオ: Music Macro Language (MML) を入力として使用。内部的にはNuked-OPMを音源として利用し、cpalライブラリでオーディオ出力を行います。バックグラウンド演奏には`ym2151-log-play-server`ライブラリを活用しています。
- 開発ツール: 主な開発言語はRustです。MMLテキストの解析にはtree-sitterを使用し、コンパイルにはZig ccが用いられています。
- テスト: 開発プロセスではTDD agentが活用されており、ヘッドレス環境でのオーディオテストのためにLinux runnerとALSA SDKが利用されています。
- ビルドツール: Rustプロジェクトのビルドと依存関係管理にはCargoが使用されています。
- 言語機能: Rustの強力な型システム、メモリ安全性、並行処理機能が活用され、信頼性の高いパフォーマンスを実現しています。
- 自動化・CI/CD: GitHub ActionsがREADMEの自動翻訳などの定型作業に利用され、開発効率を高めています。
- 開発標準: コードの品質と統一性を保つため、.editorconfigでエディタの設定を、.vscode/settings.jsonでVisual Studio Codeの開発環境設定を共有しています。

## ファイル階層ツリー
```
📄 .editorconfig
📄 .gitignore
📁 .vscode/
  📊 extensions.json
  📊 settings.json
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📁 generated-docs/
📁 issue-notes/
  📖 31.md
📁 src/
  📄 app.rs
  📄 cli.rs
  📄 client_manager.rs
  📄 converter.rs
  📄 input.rs
  📄 main.rs
```

## ファイル詳細説明
- **`.editorconfig`**: コードエディタの各種設定（インデントスタイル、文字コードなど）をプロジェクト全体で統一するための設定ファイルです。
- **`.gitignore`**: Gitバージョン管理システムが追跡しないファイルやディレクトリを指定します。
- **`.vscode/extensions.json`**: Visual Studio Codeでこのプロジェクトを開く際に推奨される拡張機能のリストです。
- **`.vscode/settings.json`**: Visual Studio Codeのプロジェクト固有の設定（ワークスペース設定）を定義します。
- **`Cargo.lock`**: プロジェクトのビルド時に使用された全ての依存クレートの正確なバージョンを記録します。
- **`Cargo.toml`**: Rustプロジェクトのビルド設定ファイルであり、プロジェクト名、バージョン、依存関係、ターゲットなどを定義します。
- **`LICENSE`**: このプロジェクトのライセンス情報（MIT License）が記載されています。
- **`README.ja.md`**: プロジェクトの日本語での概要、使い方、技術詳細などを説明するドキュメントです。
- **`README.md`**: プロジェクトの英語での概要、使い方、技術詳細などを説明するドキュメントです（`README.ja.md`から自動生成）。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルとして使用されることがあります。
- **`generated-docs/`**: 自動生成されたドキュメントやレポートを格納するためのディレクトリです。
- **`issue-notes/31.md`**: 特定のイシュー（問題点や機能要望）に関する詳細なメモや検討事項を記述したファイルです。
- **`src/app.rs`**: アプリケーションの主要なビジネスロジックをカプセル化するモジュールです。CLIからのコマンドに応じた処理の分岐と実行を管理します。
- **`src/cli.rs`**: コマンドラインインターフェースの引数解析に関する定義を行うモジュールです。`cat-play-mml`コマンドが受け付けるオプションや引数を構造化します。
- **`src/client_manager.rs`**: サーバーモードとクライアントモード間の通信を管理するモジュールです。バックグラウンドで動作する演奏サーバーへのMMLデータ送信などを担当します。
- **`src/converter.rs`**: Music Macro Language (MML) の文字列を解析し、内部の音楽データ構造に変換するロジックを格納するモジュールです。tree-sitterパーサーがここで利用されます。
- **`src/input.rs`**: ユーザーからのMML文字列入力に関する処理を行うモジュールです。
- **`src/main.rs`**: プログラムのエントリーポイントとなるファイルです。`src/cli.rs`で定義された引数をパースし、その結果に基づいて`src/app.rs`などの主要な処理を呼び出します。

## 関数詳細説明
- **`main` 関数**: プログラムの実行開始点です。コマンドライン引数をパースし、その内容に応じてアプリケーションをクライアントモード、サーバーモード、停止コマンド、シャットダウンコマンドのいずれかで起動・制御します。
- **`parse_arguments` 関数（仮称）**: コマンドラインから渡された引数を解析し、実行すべきアクション（MML再生、サーバー起動、停止など）とMMLデータ（存在する場合）を抽出します。
- **`execute_mml_playback` 関数（仮称）**: 解析されたMMLデータを音楽データの中間表現に変換し、それをオーディオ波形として生成・再生する一連の処理を実行します。クライアントモードからの要求を受け、サーバーに転送する場合もあります。
- **`start_background_server` 関数（仮称）**: MMLの解析とオーディオ再生をバックグラウンドで処理するサーバープロセスを起動します。これにより、コマンドラインがすぐに解放され、ユーザーは他の操作を行うことができます。
- **`send_mml_to_server` 関数（仮称）**: 既に起動しているバックグラウンドサーバーに対して、MML文字列を送信し、演奏の開始を指示します。
- **`stop_current_playback` 関数（仮称）**: 実行中のMML演奏を停止するコマンドをサーバーに送信するか、直接停止処理を実行します。
- **`shutdown_application_server` 関数（仮称）**: バックグラウンドで実行されているサーバープロセスを安全に終了させます。
- **`convert_mml_to_music_data` 関数（仮称）**: 入力されたMML文字列を、アプリケーション内部で処理可能な音楽データ構造（例えば、音符のリストやタイミング情報など）に変換します。この中でtree-sitterによる構文解析が行われます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層はプロジェクト情報からは詳細に分析できませんでしたが、一般的なRust CLIアプリケーションとして以下の主要な流れが想定されます。

- `main`
  - `parse_arguments`
  - (条件分岐: MML再生/サーバー起動/停止/シャットダウン)
    - `execute_mml_playback`
      - `send_mml_to_server` (クライアントモードの場合)
        - `convert_mml_to_music_data`
        - (オーディオ生成・再生処理)
    - `start_background_server`
    - `stop_current_playback`
    - `shutdown_application_server`

---
Generated at: 2025-11-25 07:03:14 JST
