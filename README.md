# Kaiten Sushi

Rust CLI version of Sushi Bot.

## Installation

### With cargo

```bash
cargo install kaiten-sushi
```

### Direct install

Need rustc.

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
% kaiten-sushi maguro tamago engawa kappa ikura salmon uni hamachi

へい　らっしゃい！

1皿目は、tamago
2皿目は、hamachi
3皿目は、maguro
4皿目は、kappa
5皿目は、engawa
6皿目は、uni
7皿目は、ikura
8皿目は、salmon

おあいそ！

```
