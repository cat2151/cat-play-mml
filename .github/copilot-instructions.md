# cat-play-mml 向け AI コーディングエージェント指示書

## プロジェクト概要
`cat-play-mml` は、Windows に特化した MML (Music Macro Language) から音声への変換パイプラインで、`"cde"` のようなテキストをリアルタイム音声再生（「ド・レ・ミ」）に変換します。アーキテクチャは、各段階に独立したクレートを持つ **4 層変換パイプライン** に従っています。

## アーキテクチャとデータフロー
アプリケーションは、複数の Git リポジトリにわたる **モジュラー変換パイプライン** を使用しています：
1. **MML → SMF**: `mmlabc-to-smf-rust` (4 パス: parser → AST → events → MIDI)
2. **SMF → YM2151 Log**: `smf-to-ym2151log-rust`
3. **YM2151 Log → Audio**: `ym2151-log-play-server` (Nuked-OPM synthesis)
4. **Client Orchestration**: このリポジトリがパイプラインを調整

`src/` 内の主要コンポーネント：
- `app.rs`: 出力制御のための `VerbosityConfig` を持つメインオーケストレーター
- `converter.rs`: パイプライン統合 (`generate_json_from_input`)
- `input.rs`: ファイル形式検出 (MML/MIDI/JSON)
- `client_manager.rs`: 名前付きパイプ経由でのサーバー通信
- `process_manager.rs`: デタッチされたサーバーモードでの Windows プロセス生成

## 重要な開発パターン

### サーバーアーキテクチャ
- **デュアルモード動作**: クライアントコマンド vs. バックグラウンドサーバー (`--server`)
- **自動サーバー起動**: サーバーが動作していない場合、クライアントが自動的にサーバーを開始
- **Windows 固有プロセス処理**: `CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS` を使用

### エラーハンドリング戦略
- 全体を通して `anyhow::Result` を使用
- サーバー接続エラーは `client_manager.rs` 内の自動起動ロジックを起動
- Windows ロケールサポートのための日本語エラーメッセージ検出

### ビルドと依存関係管理
- **Git 依存関係**: すべての音楽処理クレートは Git サブモジュール
- **Windows ファースト設計**: Linux サポートはエージェント TDD ワークフローのみ

## 主要な開発コマンド
```powershell
# 開発ワークフロー
cargo run --release cde          # 基本再生テスト
cargo install --path .           # ローカルインストール
cat-play-mml --server --verbose  # デバッグサーバーモード

# バックグラウンドサーバー制御
cat-play-mml cde                 # 自動サーバー開始＋再生
cat-play-mml --stop             # 再生停止
cat-play-mml --shutdown         # サーバー終了
```

## 入力処理規約
`input.rs` モジュールは拡張子検出により **4 つの入力タイプ** を処理します：
- 生 MML 文字列（デフォルトフォールバック）
- `.mml` ファイル → MML コンテンツ
- `.mid` ファイル → SMF バイナリデータ
- `.json` ファイル → 直接 YM2151 ログ形式

処理前に **必ず** `detect_input_type()` を使用してください - 入力形式を仮定しないでください。

## プロジェクト固有の規約

### 詳細度制御
出力抑制のために `VerbosityConfig::from_args()` パターンを使用：
- サーバーモードは `--verbose` でない限り出力を抑制
- すべてのユーザー向けメッセージは `verbosity.println()` を通します

### 複数リポジトリ調整
- **中間形式としての SMF**: 検証/デバッグを可能にします
- **YM2151 シンセシス**: Nuked-OPM によるハードウェア正確な FM シンセシス

## 一般的な落とし穴
1. **`VerbosityConfig` を迂回しない** - コンソール出力に関して
2. **Windows パス処理**: セルフスポーンには `std::env::current_exe()` を使用
3. **サーバーライフサイクル**: サーバー状態を仮定する前に常に接続を確認
4. **JSON パイプライン**: SMF → YM2151 ログ変換は静かに失敗する場合があります - エラーコンテキストを確認
5. **入力検証**: 空の MML 文字列は適切にエラーを出すべきです

## テストとデバッグ
- パイプラインステップを追跡するには `--verbose` フラグを使用
- サーバーログ出力は音声シンセシス問題のデバッグに役立ちます
- SMF 中間ファイルは外部 MIDI ツールで検査可能
- YM2151 ログ JSON 形式は手動音声検証を可能にします

## 関連プロジェクトとの統合
エコシステム全体で作業する際：
- **mmlabc-to-smf-rust**: パーサーと MIDI 生成
- **smf-to-ym2151log-rust**: MIDI からシンセシスイベントへ
- **ym2151-log-play-server**: リアルタイム音声再生

## userからの指示
- このプロジェクトは、ライブラリとして **ym2151-log-play-server** を使用しています。
  - それを用いて、サーバーモードでの起動、クライアントとしてサーバーへの接続、を実現しています。
  - クライアント・サーバー機能で問題があったときは、まず、 **ym2151-log-play-server** について調査してください。
  - サーバーモード時は、 **ym2151-log-play-server** の機能により、受信メッセージに従ってインタラクティブモードと非インタラクティブモードを切り替えることができます。
