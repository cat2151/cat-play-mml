# Agent Instructions for cat-play-mml Project

このドキュメントは、cat-play-mmlプロジェクトで作業するAIエージェント向けの指示書です。

## 重要な前提条件

### ターゲット環境: Windows専用

**このプロジェクトはWindows環境を対象としています。**

- **MacOSやLinuxに関する記述は一切含めないでください**
- macOSやLinuxのパッケージマネージャー（brew、apt、yumなど）の使用を提案しないでください
- macOSやLinuxのパス表記（/usr/local、/opt など）を使用しないでください
- Windows固有の機能とツールを優先してください

これは、エージェントのハルシネーション（不正確な情報の生成）を防ぐための明示的な制約です。

## スクリプトの記述方針

### Python scriptを優先

**すべての自動化タスクはPythonスクリプトで実装してください。**

- ✅ **使用推奨**: Pythonスクリプト（.py）
- ❌ **使用禁止**: 
  - BATファイル（.bat、.cmd）
  - PowerShellスクリプト（.ps1）
  - シェルスクリプト（.sh）

#### 理由

1. **クロスプラットフォーム互換性**: Pythonは本質的にクロスプラットフォーム
2. **可読性**: BATやPowerShellより読みやすく保守しやすい
3. **一貫性**: プロジェクトの主要実装言語がPython
4. **機能性**: 複雑な処理もPythonなら簡潔に記述可能

#### 例

タスクの自動化が必要な場合:

```python
# scripts/build.py
import subprocess
import sys

def main():
    """Build the project"""
    print("Building cat-play-mml...")
    
    # Run tests
    result = subprocess.run([sys.executable, "-m", "pytest"], cwd=".")
    if result.returncode != 0:
        sys.exit(1)
    
    # Check code quality
    subprocess.run([sys.executable, "-m", "ruff", "check", "src/", "tests/"])
    
    print("Build completed successfully!")

if __name__ == "__main__":
    main()
```

実行方法:
```cmd
python scripts/build.py
```

## Windows開発環境のセットアップ

### 必要なツール

1. **Python 3.8以上**
   - 公式サイトからインストール: https://www.python.org/downloads/
   - インストーラーで「Add Python to PATH」を必ずチェック

2. **pip** (Pythonに同梱)
   - パッケージ管理に使用

3. **Git for Windows**
   - https://git-scm.com/download/win
   - コマンドプロンプトまたはPowerShellから使用可能

4. **Visual Studio Code** (推奨)
   - https://code.visualstudio.com/
   - Python拡張機能をインストール

### 環境構築手順

```cmd
# リポジトリのクローン
git clone https://github.com/cat2151/cat-play-mml.git
cd cat-play-mml

# Python依存関係のインストール
python -m pip install --upgrade pip
pip install -r requirements.txt
```

## プロジェクト構造

```
cat-play-mml/
├── docs/                      # ドキュメント
│   ├── AGENT_INSTRUCTIONS.md  # このファイル
│   ├── QUICKSTART.md          # クイックスタートガイド
│   └── PROJECT_SETUP_COMPLETION.md
├── src/
│   ├── python/                # Python実装（メイン）
│   │   ├── play_mml.py       # CLIエントリーポイント
│   │   ├── mml_parser.py     # MMLパーサー
│   │   └── audio_player.py   # オーディオ再生
│   ├── rust/                  # Rust実装（予定）
│   ├── go/                    # Go実装（予定）
│   └── typescript/            # TypeScript実装（予定）
├── tests/                     # テストファイル
├── requirements.txt           # Python依存関係
├── pytest.ini                # pytest設定
└── ruff.toml                 # コード品質ツール設定
```

## 開発ワークフロー

### 1. コード編集

Visual Studio Codeまたはお好みのエディタでコードを編集します。

### 2. コード品質チェック

```cmd
# フォーマット
python -m ruff format src/ tests/

# リントチェック
python -m ruff check src/ tests/

# 自動修正
python -m ruff check --fix src/ tests/
```

### 3. テスト実行

```cmd
# すべてのテストを実行
python -m pytest

# 詳細出力
python -m pytest -v

# 特定のテストファイルのみ
python -m pytest tests/test_specific.py
```

### 4. アプリケーション実行

```cmd
# MMLを再生
python src/python/play_mml.py "cde"

# より複雑な例
python src/python/play_mml.py "t120 o4 cdefgab>c"
```

## コーディング規約

### Python

1. **PEP 8準拠**: ruffが自動チェック
2. **型ヒント推奨**: 主要な関数には型アノテーションを付ける
3. **docstring**: モジュール・クラス・関数に説明を記述
4. **命名規則**:
   - クラス: PascalCase (例: `MmlParser`)
   - 関数・変数: snake_case (例: `parse_mml`)
   - 定数: UPPER_SNAKE_CASE (例: `SAMPLE_RATE`)

### ファイル構成

- 各モジュールは単一責任の原則に従う
- 大きなファイルは適切に分割する
- テストファイルは `tests/` ディレクトリに配置

## よくあるタスク

### 新しい機能の追加

1. 該当するモジュールファイルを編集
2. テストを追加（`tests/` ディレクトリ）
3. コード品質チェック実行
4. テスト実行
5. コミット

### バグ修正

1. バグを再現するテストを作成
2. コードを修正
3. テストが通ることを確認
4. コミット

### 依存関係の追加

新しいPythonパッケージが必要な場合:

```cmd
# インストール
pip install <package-name>

# requirements.txtに追記
pip freeze | findstr <package-name> >> requirements.txt
```

または、requirements.txtを直接編集:

```txt
# requirements.txt
sounddevice>=0.4.6
numpy>=1.24.0
<新しいパッケージ>>=<バージョン>
```

## トラブルシューティング

### sounddeviceのインストールエラー

Windowsでは通常そのままインストールできますが、エラーが出る場合:

```cmd
# Visual Studio Build Toolsのインストールが必要な場合があります
# https://visualstudio.microsoft.com/downloads/ から
# "Build Tools for Visual Studio" をダウンロードしてインストール
```

### オーディオが再生されない

1. Windowsのサウンド設定を確認
2. オーディオデバイスが正しく接続されているか確認
3. 他のアプリケーション（メディアプレーヤーなど）で音が出るか確認
4. Pythonスクリプトを管理者権限で実行してみる

### Pythonがパスに見つからない

```cmd
# Pythonのパスを確認
where python

# パスが表示されない場合、環境変数PATHに追加が必要
# システムのプロパティ > 環境変数 から設定
```

## エージェントとして作業する際の注意点

### 必須事項

1. **Windows環境を前提とする**
   - すべてのコマンド例はWindows（cmd）で動作するもの
   - パス区切りはバックスラッシュ（`\`）またはスラッシュ（`/`、Pythonで互換性あり）

2. **Pythonスクリプトを優先**
   - 自動化タスクはPythonで実装
   - シェルスクリプトやバッチファイルは作成しない

3. **既存のコード規約を遵守**
   - ruffの設定に従う
   - 既存コードのスタイルを踏襲

4. **最小限の変更**
   - 必要最小限の変更に留める
   - 動作しているコードは不用意に変更しない

### 推奨事項

1. **コミット前にチェック**
   ```cmd
   python -m ruff format src/ tests/
   python -m ruff check src/ tests/
   python -m pytest
   ```

2. **明確なコミットメッセージ**
   - 何を変更したか明確に記述
   - 日本語または英語で記述

3. **テストの追加**
   - 新機能には必ずテストを追加
   - バグ修正には再現テストを追加

## 参考リンク

### プロジェクト内ドキュメント

- [README.md](../README.md) - プロジェクト概要
- [QUICKSTART.md](QUICKSTART.md) - クイックスタートガイド
- [PROJECT_SETUP_COMPLETION.md](PROJECT_SETUP_COMPLETION.md) - プロジェクト初期設定報告書

### 各言語の実装計画

- [Python実装](../src/python/README.md)
- [Rust実装計画](../src/rust/IMPLEMENTATION_PLAN.md)
- [Go実装計画](../src/go/IMPLEMENTATION_PLAN.md)
- [TypeScript(Browser)実装計画](../src/typescript/browser/IMPLEMENTATION_PLAN.md)
- [TypeScript(Deno)実装計画](../src/typescript/deno/IMPLEMENTATION_PLAN.md)

### 外部リソース

- [Python公式ドキュメント](https://docs.python.org/ja/3/)
- [Ruff公式ドキュメント](https://docs.astral.sh/ruff/)
- [pytest公式ドキュメント](https://docs.pytest.org/)

## まとめ

このプロジェクトで作業する際は:

1. ✅ **Windows環境を対象とする**
2. ✅ **Pythonスクリプトを使用する**（BAT/sh/PowerShellは使用しない）
3. ❌ **macOSやLinuxの情報は含めない**（ハルシネーション防止）

これらのガイドラインに従うことで、一貫性のある高品質なコードベースを維持できます。
