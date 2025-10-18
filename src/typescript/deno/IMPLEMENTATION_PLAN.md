# TypeScript (Deno版) 実装計画書

## 概要

TypeScript + Denoを使用したCLI版MML（Music Macro Language）パーサーとプレーヤーを実装します。

## 目標

Denoで動作するCLIアプリケーションとして、コマンドライン引数"cde"を受け取りドレミを音声で再生します。

## 使用技術

- **Deno**: TypeScriptランタイム
- **FFI (Foreign Function Interface)**: ネイティブオーディオライブラリとの連携
- **または WebAssembly**: オーディオ処理のため

## プロジェクト構造

```
src/typescript/deno/
├── deno.json
├── main.ts              # メインエントリーポイント
├── parser.ts            # MMLパーサー（ブラウザ版と共通化可能）
├── audio.ts             # オーディオ生成・再生
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

1. **deno.jsonの作成**

```json
{
  "tasks": {
    "dev": "deno run --allow-all main.ts",
    "play": "deno run --allow-all main.ts"
  },
  "compilerOptions": {
    "lib": ["deno.window", "deno.unstable"],
    "strict": true
  }
}
```

2. **ディレクトリ構造の確認**
   ```bash
   mkdir -p src/typescript/deno
   ```

### Phase 2: MMLパーサーの実装

**ファイル**: `src/typescript/deno/parser.ts`

```typescript
export interface Note {
    pitch: string;
    octave: number;
    duration: number;
    frequency: number;
}

function calculateFrequency(pitch: string, octave: number): number {
    // A4 = 440Hz を基準に計算
    const noteOffsets: Record<string, number> = {
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

### Phase 3: オーディオ生成の実装

**注意**: Denoでのネイティブオーディオ再生は複雑なため、いくつかのアプローチがあります。

#### アプローチ1: WAVファイル生成 + システムコマンド実行（最もシンプル）

**ファイル**: `src/typescript/deno/audio.ts`

```typescript
import { Note } from './parser.ts';

const SAMPLE_RATE = 48000;

export class AudioPlayer {
    async playNotes(notes: Note[]): Promise<void> {
        // WAVファイルを生成
        const wavData = this.generateWAV(notes);
        
        // 一時ファイルに書き込み
        const tempFile = await Deno.makeTempFile({ suffix: '.wav' });
        await Deno.writeFile(tempFile, wavData);
        
        // システムの音楽プレーヤーで再生
        await this.playWAV(tempFile);
        
        // クリーンアップ
        await Deno.remove(tempFile);
    }

    private generateWAV(notes: Note[]): Uint8Array {
        const samples: number[] = [];
        
        for (const note of notes) {
            const noteSamples = this.generateSineWave(note.frequency, note.duration);
            samples.push(...noteSamples);
        }
        
        return this.createWAVFile(samples);
    }

    private generateSineWave(frequency: number, duration: number): number[] {
        const numSamples = Math.floor(SAMPLE_RATE * duration);
        const samples: number[] = [];
        
        if (frequency === 0) {
            // 休符
            return new Array(numSamples).fill(0);
        }
        
        const attackSamples = Math.floor(SAMPLE_RATE * 0.01);
        const releaseSamples = Math.floor(SAMPLE_RATE * 0.05);
        
        for (let i = 0; i < numSamples; i++) {
            const t = i / SAMPLE_RATE;
            let value = Math.sin(2 * Math.PI * frequency * t);
            
            // エンベロープ
            let envelope = 1.0;
            if (i < attackSamples) {
                envelope = i / attackSamples;
            } else if (i > numSamples - releaseSamples) {
                envelope = (numSamples - i) / releaseSamples;
            }
            
            value *= envelope * 0.3;
            samples.push(value);
        }
        
        return samples;
    }

    private createWAVFile(samples: number[]): Uint8Array {
        const numChannels = 1;
        const bitsPerSample = 16;
        const byteRate = SAMPLE_RATE * numChannels * bitsPerSample / 8;
        const blockAlign = numChannels * bitsPerSample / 8;
        const dataSize = samples.length * blockAlign;
        
        const buffer = new ArrayBuffer(44 + dataSize);
        const view = new DataView(buffer);
        
        // RIFF header
        this.writeString(view, 0, 'RIFF');
        view.setUint32(4, 36 + dataSize, true);
        this.writeString(view, 8, 'WAVE');
        
        // fmt chunk
        this.writeString(view, 12, 'fmt ');
        view.setUint32(16, 16, true); // chunk size
        view.setUint16(20, 1, true); // PCM
        view.setUint16(22, numChannels, true);
        view.setUint32(24, SAMPLE_RATE, true);
        view.setUint32(28, byteRate, true);
        view.setUint16(32, blockAlign, true);
        view.setUint16(34, bitsPerSample, true);
        
        // data chunk
        this.writeString(view, 36, 'data');
        view.setUint32(40, dataSize, true);
        
        // PCM data
        let offset = 44;
        for (const sample of samples) {
            const s = Math.max(-1, Math.min(1, sample));
            view.setInt16(offset, s * 0x7FFF, true);
            offset += 2;
        }
        
        return new Uint8Array(buffer);
    }

    private writeString(view: DataView, offset: number, str: string): void {
        for (let i = 0; i < str.length; i++) {
            view.setUint8(offset + i, str.charCodeAt(i));
        }
    }

    private async playWAV(filePath: string): Promise<void> {
        const os = Deno.build.os;
        
        let command: string[];
        if (os === 'darwin') {
            command = ['afplay', filePath];
        } else if (os === 'linux') {
            command = ['aplay', filePath];
        } else if (os === 'windows') {
            command = ['powershell', '-c', `(New-Object Media.SoundPlayer '${filePath}').PlaySync()`];
        } else {
            throw new Error(`Unsupported OS: ${os}`);
        }
        
        const process = new Deno.Command(command[0], {
            args: command.slice(1),
            stdout: 'null',
            stderr: 'null',
        });
        
        const { code } = await process.output();
        
        if (code !== 0) {
            throw new Error(`Failed to play audio (exit code: ${code})`);
        }
    }
}
```

#### アプローチ2: FFI経由でportaudio使用（より高度）

```typescript
// 将来の拡張として、FFIを使ってportaudioなどのネイティブライブラリを呼び出す
// これは複雑なため、Phase 1ではアプローチ1を推奨
```

### Phase 4: メインプログラムの実装

**ファイル**: `src/typescript/deno/main.ts`

```typescript
import { parseMML } from './parser.ts';
import { AudioPlayer } from './audio.ts';

async function main(): Promise<void> {
    const args = Deno.args;
    
    if (args.length < 1) {
        console.error('使用法: deno run --allow-all main.ts <MML文字列>');
        console.error('例: deno run --allow-all main.ts "cde"');
        Deno.exit(1);
    }
    
    const mmlString = args[0];
    console.log(`MMLを再生します: ${mmlString}`);
    
    try {
        const notes = parseMML(mmlString);
        console.log(`パース成功: ${notes.length}個の音符`);
        
        const player = new AudioPlayer();
        await player.playNotes(notes);
        
        console.log('再生完了');
    } catch (error) {
        console.error(`エラー: ${error}`);
        Deno.exit(1);
    }
}

if (import.meta.main) {
    main();
}
```

### Phase 5: READMEの作成

**ファイル**: `src/typescript/deno/README.md`

```markdown
# TypeScript Deno MML Player

Music Macro Language (MML) parser and player for Deno.

## Requirements

- Deno 1.37+
- System audio player:
  - macOS: afplay (built-in)
  - Linux: aplay (ALSA)
  - Windows: PowerShell (built-in)

## Usage

\`\`\`bash
cd src/typescript/deno
deno run --allow-all main.ts "cde"
\`\`\`

## Features

- Simple MML parser supporting basic notes (c, d, e, f, g, a, b)
- Rest notes (r)
- WAV file generation
- Cross-platform audio playback
- TypeScript with Deno runtime

## Example

\`\`\`bash
deno run --allow-all main.ts "cdefgab"
\`\`\`

## Permissions Required

- \`--allow-read\`: For temporary file operations
- \`--allow-write\`: For WAV file generation
- \`--allow-run\`: For system audio player execution

Or use \`--allow-all\` for simplicity.

## How It Works

1. Parse MML string into notes
2. Generate sine waves for each note
3. Create a WAV file
4. Play using system audio player
5. Clean up temporary files
```

## 実行

```bash
# 実行
cd src/typescript/deno
deno run --allow-all main.ts "cde"

# タスクとして実行
deno task play "cde"
```

## 将来の拡張（オプション）

1. **FFI統合**: portaudioやminiaudioを使った直接的なオーディオ再生
2. **ストリーミング**: WAVファイルを経由せずにリアルタイム再生
3. **WebAssembly**: 効率的なDSP処理
4. **インタラクティブモード**: REPLのような対話的な使用

## 技術的な注意点

### WAVファイル方式の利点

- シンプルで確実に動作
- クロスプラットフォーム対応が容易
- 外部依存が少ない

### WAVファイル方式の欠点

- リアルタイム性に欠ける
- ディスクI/Oのオーバーヘッド
- 一時ファイルの管理が必要

### FFI方式への移行

将来的にはDenoのFFI機能を使って、portaudioなどのネイティブライブラリを直接呼び出すことで、より低レイテンシな実装が可能です。

## 推定工数

- **Phase 1**: 30分（セットアップ）
- **Phase 2**: 1時間（パーサー実装）
- **Phase 3**: 3時間（オーディオ実装 + WAV生成）
- **Phase 4**: 1時間（統合とテスト）
- **Phase 5**: 30分（ドキュメント）

**合計**: 約6時間

## 成功基準

- ✅ `deno run --allow-all main.ts "cde"` でドレミが再生される
- ✅ クロスプラットフォームで動作する
- ✅ エラーハンドリングが適切に実装されている
- ✅ TypeScriptの型チェックが通る
- ✅ Denoの標準的なプロジェクト構造に従っている
