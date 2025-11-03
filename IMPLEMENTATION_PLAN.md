# 実装計画書: cat-play-mml Windows実行ファイル

## 概要

本文書は、3つの既存Rustクレートを統合し、MML（Music Macro Language）から音声再生までの完全なパイプラインを提供する単一のWindows実行ファイルを作成するための実装計画を示します。

## アーキテクチャ

### 処理パイプライン

アプリケーションは以下の順次処理フローに従います：

```
MMLテキスト入力（例: "cde"）
    ↓
[mmlabc-to-smf] → Standard MIDI File（メモリ内）
    ↓
[smf-to-ym2151log] → YM2151レジスタログ（JSON）
    ↓
[ym2151-log-player] → 音声再生
```

### コンポーネントリポジトリ

1. **mmlabc-to-smf-rust**
   - リポジトリ: https://github.com/cat2151/mmlabc-to-smf-rust
   - 目的: tree-sitterを使用してMMLテキストを解析し、Standard MIDI File形式に変換
   - 現在のインターフェース: バイナリ実行ファイル
   
2. **smf-to-ym2151log-rust**
   - リポジトリ: https://github.com/cat2151/smf-to-ym2151log-rust
   - 目的: SMFをYM2151レジスタ書き込みログ（JSON形式）に変換
   - 現在のインターフェース: バイナリ実行ファイル + ライブラリ（`smf_to_ym2151log`）
   
3. **ym2151-log-player-rust**
   - リポジトリ: https://github.com/cat2151/ym2151-log-player-rust
   - 目的: Nuked-OPMエミュレータを使用してYM2151レジスタログを再生
   - 現在のインターフェース: バイナリ実行ファイル

## 実装戦略

### 1. プロジェクト構造

このリポジトリに新しいRustプロジェクトを作成します：

```
cat-play-mml/
├── Cargo.toml          # メインワークスペース設定
├── src/
│   └── main.rs        # Windows実行ファイルのエントリーポイント
├── README.md
├── README.ja.md
└── IMPLEMENTATION_PLAN.md（本文書）
```

### 2. 依存関係の設定

メインの`Cargo.toml`で、3つのクレートをgit依存関係として参照します：

```toml
[package]
name = "cat-play-mml"
version = "0.1.0"
edition = "2021"
authors = ["cat2151"]
license = "MIT"
description = "Music Macro Language (MML) Parser and Player"

[[bin]]
name = "cat-play-mml"
path = "src/main.rs"

[dependencies]
# GitHubリポジトリから3つのクレートを参照
# 注意: 本番環境では、再現可能なビルドのために特定のタグ/コミットを指定すること
# 例: { git = "...", tag = "v0.1.0" } または { git = "...", rev = "abc123" }
mmlabc-to-smf = { git = "https://github.com/cat2151/mmlabc-to-smf-rust" }
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
ym2151-log-player-rust = { git = "https://github.com/cat2151/ym2151-log-player-rust" }

# 追加の依存関係
anyhow = "1.0"
clap = { version = "4.5", features = ["derive"] }
```

### 3. ライブラリ公開の要件

統合を機能させるためには、各クレートがライブラリとして機能を公開する必要があります：

#### mmlabc-to-smf-rust
**現在の状態**: バイナリのみ（`[[bin]]`）
**必要な変更**: 
- `Cargo.toml`に`[lib]`セクションを追加
- MML解析とSMF変換関数を公開する`src/lib.rs`を作成
- API例:
  ```rust
  pub fn convert_mml_to_smf(mml_text: &str) -> Result<Vec<u8>, Error>
  ```

#### smf-to-ym2151log-rust
**現在の状態**: バイナリとライブラリの両方を既に保持
**必要な変更**: なし（既に`src/lib.rs`を含む`[lib]`あり）
**期待されるAPI**:
  ```rust
  pub fn convert_smf_to_ym2151log(smf_data: &[u8]) -> Result<String, Error>
  ```

#### ym2151-log-player-rust
**現在の状態**: バイナリのみ
**必要な変更**:
- `Cargo.toml`に`[lib]`セクションを追加
- 再生関数を公開する`src/lib.rs`を作成
- API例:
  ```rust
  pub fn play_ym2151_log(json_log: &str) -> Result<(), Error>
  // または、より細かい制御のため:
  // pub fn play_ym2151_log_blocking(json_log: &str) -> Result<(), Error>
  ```
- 注意: API設計では再生時間とブロッキング動作を考慮すること

### 4. メインアプリケーションの実装

`src/main.rs`のメインアプリケーションは以下を行います：

1. **コマンドライン引数の解析**（`clap`を使用）
   - MMLテキストを引数として受け取る（例: "cde"）
   - デバッグ/ファイル出力用のオプションフラグをサポート

2. **パイプラインを順次呼び出す**：
   ```rust
   fn main() -> anyhow::Result<()> {
       // コマンドライン解析
       let mml_input = parse_args();
       
       // ステップ1: MML → SMF
       let smf_data = mmlabc_to_smf::convert_mml_to_smf(&mml_input)?;
       
       // ステップ2: SMF → YM2151ログ
       let ym2151_log = smf_to_ym2151log::convert_smf_to_ym2151log(&smf_data)?;
       
       // ステップ3: YM2151ログを再生
       ym2151_log_player_rust::play_ym2151_log(&ym2151_log)?;
       
       Ok(())
   }
   ```

3. **エラー処理**
   - エラー伝播に`anyhow`を使用
   - ユーザーフレンドリーなエラーメッセージを提供

### 5. Windows固有の考慮事項

- **コンソールウィンドウ**: コマンドライン使用にはデフォルトのコンソールサブシステムが適切（特別な属性は不要）
- **オーディオバックエンド**: `ym2151-log-player-rust`は`cpal`を使用し、Windows上ではWASAPIをサポート
- **ビルドターゲット**: `x86_64-pc-windows-msvc`または`x86_64-pc-windows-gnu`
- **依存関係**: Nuked-OPM CコードをビルドするためにCコンパイラ（MSVCまたはMinGW）が利用可能であることを確認

## 開発フェーズ

### フェーズ1: リポジトリセットアップ（本Issue）
- [x] 実装計画書の作成
- [ ] 関係者との計画レビュー

### フェーズ2: 上流ライブラリの準備
- [ ] `mmlabc-to-smf-rust`にライブラリインターフェースを追加
- [ ] `smf-to-ym2151log-rust`のライブラリインターフェースを確認
- [ ] `ym2151-log-player-rust`にライブラリインターフェースを追加

### フェーズ3: 統合実装
- [ ] git依存関係を含む`Cargo.toml`を作成
- [ ] 順次パイプラインを持つ`src/main.rs`を実装
- [ ] コマンドライン引数解析を追加
- [ ] エラーとエッジケースを処理

### フェーズ4: テスト
- [ ] "cde"入力（ドレミ）でテスト
- [ ] 様々なMML入力でテスト
- [ ] Windows環境でテスト
- [ ] リアルタイム再生が正常に機能することを確認

### フェーズ5: ドキュメント
- [ ] 使用方法を含むREADME.mdを更新
- [ ] 使用方法を含むREADME.ja.mdを更新（日本語）
- [ ] Windows向けビルドプロセスを文書化

## 想定される使用方法

```bash
# ドレミを再生
cat-play-mml.exe cde

# より複雑なMML
cat-play-mml.exe "t120 o4 l4 cdefgab>c"
```

## 技術的考慮事項

### メモリ内処理
- すべてのデータはメモリ内で受け渡し（中間ファイルなし）
- SMFデータは`Vec<u8>`として
- YM2151ログはJSON `String`として

### パフォーマンス
- 初回再生時はコンパイル/初期化によるレイテンシが発生する可能性
- 以降の音符はリアルタイムで最小限のレイテンシで再生

### エラーシナリオ
- 無効なMML構文
- SMF変換エラー
- オーディオバックエンド初期化失敗
- オーディオデバイスが利用不可

## 検討した代替アプローチ

### アプローチA: プロセスベース統合（不採用）

**説明**: 各ツールを個別のプロセスとして起動し、一時ファイルまたはパイプでデータを転送

**メリット**:
- 既存のバイナリをそのまま使用可能
- 各コンポーネントの独立性が高く、個別にデバッグしやすい
- クラッシュ時の影響範囲が限定される
- 言語の壁を越えた統合が可能（将来的に他言語実装への移行が容易）

**デメリット**:
- プロセス起動のオーバーヘッドが大きい
- 一時ファイル管理の複雑さ（作成・削除・エラー時の後処理）
- プロセス間のエラー処理が複雑化
- メモリ効率が悪い（データのシリアライズ/デシリアライズが必要）
- 全体的な実行速度が遅い
- プロセス間通信のデバッグが困難

**不採用理由**: リアルタイム再生を目標とするため、プロセス起動とファイルI/Oのオーバーヘッドは許容できない

### アプローチB: モノリシック書き直し（不採用）

**説明**: すべてのコードを単一リポジトリにコピーして統合

**メリット**:
- 依存関係管理が不要
- ビルドが単純化される
- コンポーネント間のインターフェースを自由に最適化可能
- デバッグが容易（すべてのコードが一箇所に）
- バージョン整合性の問題が発生しない

**デメリット**:
- DRY原則違反（コードの重複）
- 3つのリポジトリのgit履歴が失われる
- 上流の改善が自動的に反映されない（手動マージが必要）
- メンテナンスコストが大幅に増加
- コードレビューの負荷が高い
- 各コンポーネントの独立した進化が困難
- テストの重複管理が必要

**不採用理由**: 長期的なメンテナンス性とコードの再利用性を重視するため、この方法は持続可能ではない

### アプローチC: Gitサブモジュール（不採用）

**説明**: Cargo git依存関係の代わりにgit submodulesを使用

**メリット**:
- 明示的なバージョン管理（特定のコミットに固定）
- 開発者が全ソースコードをローカルに持てる
- オフラインでのビルドが可能（一度クローンすれば）
- git標準機能の使用

**デメリット**:
- ユーザーが`git submodule update --init --recursive`を実行する必要がある
- サブモジュールの更新手順が複雑
- クローン時に`--recursive`オプションを忘れやすい
- Cargoが自動的に処理するビルド統合を手動で管理する必要
- 初心者にとってサブモジュールの概念が分かりにくい
- ビルドエラー時の原因特定が困難

**不採用理由**: Cargoのgit依存関係機能がこのユースケースに最適化されており、ユーザー体験が優れているため

### 採用アプローチ: Cargo Git依存関係（採用）

**説明**: Cargoの標準的なgit依存関係機能を使用して3つのクレートを統合

**メリット**:
- Rustエコシステムの標準的な方法
- `cargo build`だけで自動的に依存関係を取得・ビルド
- 上流リポジトリの独立性を保持
- タグやコミットハッシュで特定バージョンを指定可能
- Cargoが依存関係の解決とキャッシュを自動管理
- 上流の改善を容易に取り込める（バージョン更新だけ）
- 学習コストが低い（Rustプログラマーにとって馴染みがある）

**デメリット**:
- 初回ビルド時にgit経由でダウンロードが必要（ネットワーク依存）
- 上流リポジトリにライブラリインターフェースの追加が必要
- 上流の破壊的変更の影響を受ける可能性（タグ/コミット固定で緩和可能）

**採用理由**: メリットがデメリットを大きく上回り、Rustエコシステムのベストプラクティスに準拠しているため

## 成功基準

以下のすべてが達成されたときに実装が成功したと見なされます：

1. 単一のWindows実行ファイル（`cat-play-mml.exe`）
2. コマンドライン引数としてMMLテキストを受け取る
3. "cde"をリアルタイムでドレミとして再生
4. 中間ファイルが作成されない
5. エラーメッセージが明確で対処可能
6. Nuked-OPMを介したYM2151エミュレーションによる音声再生

## 将来の機能拡張（スコープ外）

- より長いMML楽曲のファイル入力サポート
- SMFファイル出力オプション
- リアルタイムMIDI出力
- GUIインターフェース
- エフェクト処理

## 参考資料

- [mmlabc-to-smf-rustリポジトリ](https://github.com/cat2151/mmlabc-to-smf-rust)
- [smf-to-ym2151log-rustリポジトリ](https://github.com/cat2151/smf-to-ym2151log-rust)
- [ym2151-log-player-rustリポジトリ](https://github.com/cat2151/ym2151-log-player-rust)
- [Cargoブック: Gitからの依存関係指定](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories)
- [cpalオーディオライブラリ](https://github.com/RustAudio/cpal)

## 注意事項

- これはエージェント指示に記載されている通り「ふわっとした」テストIssueです
- 実際の実装は3つのリポジトリの現在の状態に応じて調整が必要になる可能性があります
- 統合の前に上流リポジトリにライブラリインターフェースを追加する必要があります
