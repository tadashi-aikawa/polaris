# Vigilancia

[![release](https://img.shields.io/github/release/tadashi-aikawa/vigilancia)](https://github.com/tadashi-aikawa/vigilancia/releases/latest)

![](./src-tauri/icons/128x128@2x.png)

Vigilancia is a desktop application for Slack core user who desires to find beneficial messages in every channel as well as they can.

**This product is early pre-alpha. So I use Japanese for writing commit messages and release notes. Additionally, I will not write documentation for the time being.**

## Technology stacks 

- [Tauri](https://github.com/tauri-apps/tauri)
- [Svelte](https://svelte.dev/)
- [Carbon Design System](https://github.com/carbon-design-system)

## User Token Scopes

- `emoji:read`
- `search:read`
- `usergroups:read`
- `users.profile:read`
- `users:read`

## Configuration

`~/.vigilancia.json`
```js
{
  "slack_token": "xoxp-....................", // 【必須】OAuthトークン
  "interval_sec": 600, // 【必須】条件ごとに巡回する間隔のデフォルト値 (秒)
  "conditions": [  // 【必須】条件 (それぞれが1タブに相当)
    {
      "query": "@tadashi-aikawa", // 【必須】Slackの検索クエリ
      "title": "ガチメンション", // 【任意】タブタイトル (省略時はqueryの値を使う)
      "color": "red", // 【任意】バッジの色. 選択肢は後で (省略時はcyan)
      "should_notify": true, // 【任意】新たなメッセージがあったときデスクトップに通知するか (省略時はfalse)
      "include_me": true, // 【任意】自分のメッセージを検索対象に含めるか (省略時はfalse),
      "interval_sec": 60 // 【任意】検索の定期間隔(秒) (省略時は親のinterval_secを使う)
    },
    {
      ...
    }
  ]
}
```

### colorの選択肢

https://carbon-components-svelte.onrender.com/components/Tag#tag-types を参照。

## Development

[Task](https://github.com/go-task/task) is required.

```
# Once
npm i
task build
```

```
# Development build
task dev
```
