Last updated: 2025-12-01

# Project Overview

## プロジェクト概要
- Music Macro Language (MML) 形式のテキストから音楽を生成し、リアルタイムで再生するWindows用CLIツールです。
- 簡単なコマンドライン入力（例: `cat-play-mml cde`）で、すぐに音楽（ドレミ）を演奏できます。
- 低レイテンシでの演奏と、バックグラウンドでのサーバーモードによる柔軟な利用が特徴です。

## 技術スタック
- フロントエンド: コマンドラインインターフェース (CLI) として動作し、ユーザーからのMML文字列入力を受け付けます。
- 音楽・オーディオ:
    - **Music Macro Language (MML)**: 音楽をテキストで記述するための言語です。
    - **cpal**: Rust製のクロスプラットフォームオーディオライブラリで、生成された音声をリアルタイムで出力するために使用されます。
    - **Nuked-OPM (関連)**: 直接の利用は明記されていませんが、関連プロジェクトで言及されており、将来的な音源生成の基盤となり得ます。
    - **Standard MIDI File (SMF)**: MMLから変換される中間表現として使用され、音楽データの確認や開発のしやすさに寄与します。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語です。
    - **tree-sitter**: MMLテキストを解析し、抽象構文木 (AST) を生成するためのパーサーフレームワークです。
    - **cargo**: Rustの公式パッケージマネージャー兼ビルドシステムです。
    - **Windows**: 主なターゲットOSおよび開発環境です。
- テスト:
    - **TDD agent**: テスト駆動開発を支援するためのエージェントです。
    - **Linux runner + ALSA SDK**: ヘッドレス環境（Linux）でのテスト実行を可能にし、オーディオ関連のTDDをサポートします。
- ビルドツール:
    - **cargo**: Rustプロジェクトのビルド、依存関係管理、テスト実行などを担当します。
- 言語機能:
    - **Rust**: 高性能で安全なコード記述を可能にする、モダンなシステムプログラミング言語です。メモリ安全性や並行処理のサポートが特徴です。
- 自動化・CI/CD:
    - **GitHub Actions**: GitHub上でワークフローを自動化し、READMEの自動生成などに利用されています。
- 開発標準:
    - **.editorconfig**: 異なるエディタやIDE間で一貫したコードスタイルを維持するための設定ファイルです。
    - **.gitignore**: バージョン管理システムが追跡しないファイルやディレクトリを指定します。
    - **.vscode/settings.json**: Visual Studio Codeエディタのプロジェクト固有の設定を定義します。

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
-   `.editorconfig`: 異なるエディタやIDE間で一貫したコードスタイルを維持するための設定ファイルです。
-   `.gitignore`: Gitがバージョン管理の対象としないファイルやディレクトリを指定します。
-   `.vscode/`: Visual Studio Codeエディタ用の設定を格納するディレクトリです。
    -   `extensions.json`: プロジェクト推奨のVS Code拡張機能を定義します。
    -   `settings.json`: プロジェクト固有のVS Code設定を定義します。
-   `Cargo.lock`: `Cargo.toml`で指定された依存関係の正確なバージョンとそのハッシュ値を記録し、ビルドの再現性を保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイルです。プロジェクト名、バージョン、著者、依存クレートなどを定義します。
-   `LICENSE`: プロジェクトのライセンス情報（MIT License）を記述したファイルです。
-   `README.ja.md`: プロジェクトの日本語版概要、使い方、特徴、技術詳細などを説明するドキュメントです。
-   `README.md`: プロジェクトの英語版概要、使い方、特徴、技術詳細などを説明するドキュメントです。
-   `_config.yml`: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルである可能性があります。
-   `generated-docs/`: 自動生成されたドキュメント（例: 開発状況レポート）を格納するディレクトリです。
-   `issue-notes/`: 開発中の課題や検討事項に関するノートを格納するディレクトリです。
    -   `31.md`: 特定の課題（Issue #31）に関する詳細なメモや議論が記述されています。
-   `src/`: プロジェクトの主要なソースコードを格納するディレクトリです。
    -   `app.rs`: アプリケーションのコアロジックや状態管理に関連するモジュールです。
    -   `cli.rs`: コマンドラインインターフェースの解析と処理を担当するモジュールです。
    -   `client_manager.rs`: サーバーモードとクライアントモード間の通信を管理するモジュールです。
    -   `converter.rs`: MML文字列を内部の音楽データ構造やSMF形式に変換するロジックを含むモジュールです。
    -   `input.rs`: ユーザーからのMML入力やその他のコマンドライン入力の処理に関連するモジュールです。
    -   `main.rs`: プロジェクトのエントリポイント（実行開始点）となるファイルです。

## 関数詳細説明
-   **`main` (src/main.rs)**
    -   役割: プログラムのエントリポイントであり、アプリケーションの起動と全体的な制御を行います。CLI引数の解析、初期化、主要な処理（MMLの解析・再生、サーバーの起動・停止など）の呼び出しを担当します。
    -   引数: 通常はOSからのコマンドライン引数（内部で取得）。
    -   戻り値: プログラムの終了ステータス（成功/失敗）。
-   **`parse_mml` (src/converter.rs など)**
    -   役割: MML形式の文字列を解析し、内部の音楽データ構造（抽象構文木や中間表現）に変換します。`tree-sitter`を利用して構文解析を行います。
    -   引数: MML形式の文字列。
    -   戻り値: 解析された音楽データ構造、またはエラー情報。
-   **`generate_audio` (src/converter.rs / src/app.rs など)**
    -   役割: 内部の音楽データ構造から、再生可能なオーディオ波形データを生成します。音符の周波数、音長、音量などを計算し、デジタル音声データに変換します。
    -   引数: 音楽データ構造、テンポ、音量などのパラメータ。
    -   戻り値: オーディオ波形データ。
-   **`play_audio` (src/app.rs / src/client_manager.rs など)**
    -   役割: 生成されたオーディオデータをコンピュータのオーディオデバイスに出力し、実際に音を鳴らします。`cpal`ライブラリを通じてシステムオーディオと連携します。
    -   引数: オーディオ波形データ、再生デバイス情報など。
    -   戻り値: なし、または再生処理の成功/失敗を示す値。
-   **`handle_cli_args` (src/cli.rs / src/main.rs など)**
    -   役割: コマンドラインから渡された引数を解析し、それに応じて実行すべきアクション（MMLの再生、サーバーの起動、停止、シャットダウンなど）を決定します。
    -   引数: コマンドライン引数のリスト。
    -   戻り値: 実行すべきコマンドを示す内部データ構造。
-   **`start_server` (src/client_manager.rs / src/app.rs など)**
    -   役割: MMLの音楽再生をバックグラウンドで行うためのサーバープロセスを起動します。これにより、コマンドラインがすぐに解放され、他の操作が可能になります。
    -   引数: なし、またはサーバーの設定オプション。
    -   戻り値: サーバープロセスの起動結果（成功/失敗）。
-   **`send_to_server` (src/client_manager.rs など)**
    -   役割: 既に起動しているバックグラウンドサーバーに対して、MMLデータや制御コマンド（例: 停止、シャットダウン）を送信します。
    -   引数: 送信するMMLデータ文字列、または制御コマンド。
    -   戻り値: サーバーへの送信結果（成功/失敗）。
-   **`stop_playback` (src/client_manager.rs / src/app.rs など)**
    -   役割: 現在サーバーで再生中の音楽を停止するコマンドを送信、または直接停止処理を行います。
    -   引数: なし。
    -   戻り値: 停止処理の成功/失敗。
-   **`shutdown_server` (src/client_manager.rs / src/app.rs など)**
    -   役割: バックグラウンドで動作しているMML再生サーバープロセスを安全に終了させます。
    -   引数: なし。
    -   戻り値: シャットダウン処理の成功/失敗。

## 関数呼び出し階層ツリー
```
main (src/main.rs)
├── handle_cli_args (src/cli.rs)
│   └── (条件分岐: 引数に応じて実行)
│       ├── start_server (src/client_manager.rs)  // サーバーモード起動
│       │   └── (内部的に) parse_mml (src/converter.rs)
│       │       └── generate_audio (src/converter.rs)
│       │           └── play_audio (src/app.rs/client_manager.rs)
│       ├── send_to_server (src/client_manager.rs) // 既存サーバーへMMLやコマンド送信
│       │   └── (サーバー側で受信後) parse_mml (src/converter.rs)
│       │       └── generate_audio (src/converter.rs)
│       │           └── play_audio (src/app.rs/client_manager.rs)
│       ├── stop_playback (src/client_manager.rs)  // サーバーの再生停止
│       └── shutdown_server (src/client_manager.rs) // サーバーのシャットダウン
└── (直接MML再生の場合)
    ├── parse_mml (src/converter.rs)
    ├── generate_audio (src/converter.rs)
    └── play_audio (src/app.rs/client_manager.rs)

---
Generated at: 2025-12-01 07:03:19 JST
