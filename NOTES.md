# NOTES

## Start

```shell
$ pnpm i
$ pnpm build
$ pnpm tauri
```

## Debug Rust

1. add breakpoint to `format!("Hello, {}!", name)`
2. add breakpoint to `greetMsg = await invoke('greet', { name });`

3. launch `Tauri Development Debug` this will launch tauri app in debug mode
4. launch `Lauch Tauri in Browser` this will launch tauri app ts in a new broser windown and in debug mode
