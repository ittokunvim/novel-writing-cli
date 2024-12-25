# novel writing cli

`novel-writing-cli`は、プログラミング言語`Rust`で書かれた小説をCLIに書き出すプログラムです。

## 使い方

リポジトリをクローンして、`cargo run`を実行することでプログラムを動かせます。

## 小説の追加方法

1. `novels`ディレクトリ下に小説を追加します。ファイル名は`NUMBER.txt`とします。

2. `data.json`に以下の形式で追加した小説のデータを追加します。

```json
[
    {
        "title": "小説のタイトル",
        "path": "小説が保存されているファイルのパス"
    }
]
```

## クレジット

開発者 [ittokunvim](https://github.com/ittokunvim)

ユーザーインプット [dialoguer](https://github.com/console-rs/dialoguer)

ターミナルフォーマッター [console](https://github.com/console-rs/console)

シリアライザー（JSON） [serde](https://github.com/serde-rs/serde)
