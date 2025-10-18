# TypeScript (ブラウザ版) 実装計画書

## 概要

TypeScript + Web Audio APIを使用したブラウザで動作するMML（Music Macro Language）パーサーとプレーヤーを実装します。

## 目標

ブラウザ上で動作し、テキスト入力またはボタンクリックで"cde"を再生するシンプルなWebアプリケーションを作成します。

## 使用技術

- **TypeScript**: 型安全な実装
- **Web Audio API**: ブラウザネイティブの音声生成・再生
- **Vite**: モダンなビルドツール・開発サーバー

## プロジェクト構造

```
src/typescript/browser/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── index.html
├── src/
│   ├── main.ts          # メインエントリーポイント
│   ├── parser.ts        # MMLパーサー
│   ├── audio.ts         # Web Audio API ラッパー
│   └── style.css        # スタイル
├── README.md
└── IMPLEMENTATION_PLAN.md (このファイル)
```

## データ構造

```typescript
interface Note {
    pitch: string;        // 'c', 'd', 'e', 'f', 'g', 'a', 'b', 'r'
    octave: number;       // 4 (default)
    duration: number;     // 0.5 (秒)
    frequency: number;    // 計算された周波数 (Hz)
}
```

## 実装ステップ

### Phase 1: プロジェクトセットアップ

1. **package.jsonの作成**

```json
{
  "name": "cat-play-mml-browser",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "devDependencies": {
    "typescript": "^5.0.0",
    "vite": "^5.0.0"
  }
}
```

2. **tsconfig.jsonの作成**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "moduleResolution": "bundler",
    "strict": true,
    "skipLibCheck": true,
    "esModuleInterop": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true
  },
  "include": ["src"]
}
```

### Phase 2: MMLパーサーの実装

**ファイル**: `src/typescript/browser/src/parser.ts`

```typescript
export interface Note {
    pitch: string;
    octave: number;
    duration: number;
    frequency: number;
}

function calculateFrequency(pitch: string, octave: number): number {
    // A4 = 440Hz を基準に計算
    const noteOffsets: { [key: string]: number } = {
        c: -9, d: -7, e: -5, f: -4,
        g: -2, a: 0, b: 2
    };

    if (pitch === 'r') {
        return 0; // 休符
    }

    const offset = noteOffsets[pitch.toLowerCase()] ?? 0;
    const semitones = (octave - 4) * 12 + offset;
    return 440 * Math.pow(2, semitones / 12);
}

export function parseMML(mml: string): Note[] {
    const notes: Note[] = [];
    const currentOctave = 4;
    const currentDuration = 0.5;

    for (const char of mml.toLowerCase()) {
        if ('cdefgabr'.includes(char)) {
            const frequency = calculateFrequency(char, currentOctave);
            notes.push({
                pitch: char,
                octave: currentOctave,
                duration: currentDuration,
                frequency
            });
        }
    }

    return notes;
}
```

### Phase 3: Web Audio APIラッパーの実装

**ファイル**: `src/typescript/browser/src/audio.ts`

```typescript
import { Note } from './parser';

export class AudioPlayer {
    private audioContext: AudioContext;

    constructor() {
        this.audioContext = new AudioContext();
    }

    async playNotes(notes: Note[]): Promise<void> {
        // ユーザーインタラクション後にAudioContextを再開
        if (this.audioContext.state === 'suspended') {
            await this.audioContext.resume();
        }

        let startTime = this.audioContext.currentTime;

        for (const note of notes) {
            await this.playNote(note, startTime);
            startTime += note.duration;
        }

        // 最後の音が終わるまで待つ
        const totalDuration = notes.reduce((sum, note) => sum + note.duration, 0);
        return new Promise(resolve => {
            setTimeout(resolve, totalDuration * 1000);
        });
    }

    private async playNote(note: Note, startTime: number): Promise<void> {
        if (note.frequency === 0) {
            // 休符は何もしない
            return;
        }

        const oscillator = this.audioContext.createOscillator();
        const gainNode = this.audioContext.createGain();

        oscillator.type = 'sine';
        oscillator.frequency.value = note.frequency;

        // エンベロープ（ADSR）
        const attackTime = 0.01;
        const releaseTime = 0.05;
        const sustainLevel = 0.3;

        gainNode.gain.setValueAtTime(0, startTime);
        gainNode.gain.linearRampToValueAtTime(sustainLevel, startTime + attackTime);
        gainNode.gain.setValueAtTime(sustainLevel, startTime + note.duration - releaseTime);
        gainNode.gain.linearRampToValueAtTime(0, startTime + note.duration);

        oscillator.connect(gainNode);
        gainNode.connect(this.audioContext.destination);

        oscillator.start(startTime);
        oscillator.stop(startTime + note.duration);
    }

    close(): void {
        this.audioContext.close();
    }
}
```

### Phase 4: UIとメインプログラムの実装

**ファイル**: `src/typescript/browser/index.html`

```html
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MML Player - Browser</title>
</head>
<body>
    <div id="app">
        <h1>🎵 MML Player</h1>
        <div class="container">
            <input
                type="text"
                id="mml-input"
                placeholder="MMLを入力 (例: cde)"
                value="cde"
            />
            <button id="play-button">再生</button>
        </div>
        <div id="status"></div>
    </div>
    <script type="module" src="/src/main.ts"></script>
</body>
</html>
```

**ファイル**: `src/typescript/browser/src/main.ts`

```typescript
import { parseMML } from './parser';
import { AudioPlayer } from './audio';
import './style.css';

let player: AudioPlayer | null = null;

function updateStatus(message: string): void {
    const statusEl = document.getElementById('status');
    if (statusEl) {
        statusEl.textContent = message;
    }
}

async function playMML(): Promise<void> {
    const input = document.getElementById('mml-input') as HTMLInputElement;
    const mml = input.value;

    if (!mml) {
        updateStatus('MML文字列を入力してください');
        return;
    }

    updateStatus(`MMLを再生します: ${mml}`);

    try {
        const notes = parseMML(mml);
        updateStatus(`パース成功: ${notes.length}個の音符`);

        if (!player) {
            player = new AudioPlayer();
        }

        await player.playNotes(notes);
        updateStatus('再生完了');
    } catch (error) {
        updateStatus(`エラー: ${error}`);
        console.error(error);
    }
}

document.addEventListener('DOMContentLoaded', () => {
    const playButton = document.getElementById('play-button');
    if (playButton) {
        playButton.addEventListener('click', playMML);
    }

    const input = document.getElementById('mml-input') as HTMLInputElement;
    if (input) {
        input.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                playMML();
            }
        });
    }
});
```

**ファイル**: `src/typescript/browser/src/style.css`

```css
:root {
    font-family: system-ui, -apple-system, sans-serif;
    line-height: 1.5;
    color: #213547;
    background-color: #ffffff;
}

body {
    margin: 0;
    display: flex;
    place-items: center;
    min-width: 320px;
    min-height: 100vh;
}

#app {
    max-width: 600px;
    margin: 0 auto;
    padding: 2rem;
    text-align: center;
}

h1 {
    font-size: 2.5em;
    margin-bottom: 1em;
}

.container {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
}

input {
    flex: 1;
    padding: 0.75rem;
    font-size: 1rem;
    border: 2px solid #ccc;
    border-radius: 4px;
}

button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    background-color: #646cff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

button:hover {
    background-color: #535bf2;
}

#status {
    padding: 1rem;
    min-height: 2rem;
    color: #666;
}
```

### Phase 5: READMEの作成

**ファイル**: `src/typescript/browser/README.md`

```markdown
# TypeScript Browser MML Player

Music Macro Language (MML) parser and player for web browsers.

## Requirements

- Node.js 18+
- npm or yarn

## Setup

\`\`\`bash
cd src/typescript/browser
npm install
\`\`\`

## Development

\`\`\`bash
npm run dev
\`\`\`

Then open http://localhost:5173 in your browser.

## Build

\`\`\`bash
npm run build
\`\`\`

Output will be in the \`dist/\` directory.

## Usage

1. Enter an MML string in the input field (e.g., "cde")
2. Click the "再生" button or press Enter
3. Listen to the music!

## Features

- Simple MML parser supporting basic notes (c, d, e, f, g, a, b)
- Rest notes (r)
- Web Audio API-based audio synthesis
- Interactive browser UI
- TypeScript for type safety

## Browser Support

Modern browsers with Web Audio API support:
- Chrome/Edge 80+
- Firefox 75+
- Safari 14+
```

## ビルドと実行

```bash
# 依存関係のインストール
cd src/typescript/browser
npm install

# 開発サーバー起動
npm run dev

# ブラウザで http://localhost:5173 を開く

# 本番ビルド
npm run build
```

## 将来の拡張（オプション）

1. **視覚化**: リアルタイムで波形やスペクトラムを表示
2. **プリセット**: よく使うMML文字列をボタンで選択
3. **録音機能**: WAVファイルとしてダウンロード
4. **オクターブ・音長制御**: 拡張MML構文のサポート
5. **複数トラック**: 和音や多声部の再生

## 技術的な注意点

### Web Audio APIの制約

- ユーザーインタラクション（クリック等）後でないとAudioContextが開始できない
- ブラウザによって微妙に動作が異なる可能性がある

### TypeScriptの利点

- 型安全性によるバグの早期発見
- IDEサポートによる開発効率向上

## 推定工数

- **Phase 1**: 30分（セットアップ）
- **Phase 2**: 1時間（パーサー実装）
- **Phase 3**: 2時間（Web Audio API実装）
- **Phase 4**: 1時間（UI実装）
- **Phase 5**: 30分（ドキュメント）

**合計**: 約5時間

## 成功基準

- ✅ ブラウザで"cde"を入力して再生できる
- ✅ レスポンシブなUI
- ✅ エラーハンドリングが適切に実装されている
- ✅ TypeScriptの型チェックが通る
- ✅ モダンブラウザで動作する
