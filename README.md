# Kaiten Sushi

Rust CLI version of Sushi Bot.

## Installation

### With cargo

```bash
cargo install kaiten-sushi
```

### Direct install

```bash
curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git Samemura/kaiten-sushi
```

if you already installed once and would update, remove installed file then try install again.
```
rm ~/.cargo/bin/kaiten-sushi
```

## How to use

Kaiten-sushi(回転すし)はネタを渡すと、ランダムに寿司を提供してくれます。

```
% kaiten-sushi hamurin soncho mochi jiro kenchan masa yoshi pikasigu koyacchi

へい　らっしゃい！

1皿目は、kenchan
2皿目は、masa
3皿目は、jiro
4皿目は、pikasigu
5皿目は、koyacchi
6皿目は、yoshi
7皿目は、mochi
8皿目は、hamurin
9皿目は、soncho

おあいそ！

```