# choomsay 🤖💬

> a cyberpunk choom that speaks your text in a terminal bubble — like `cowsay`, but it's *me*.

A tiny, **zero-dependency** Rust CLI. Give it a message (as args or piped on stdin) and a
little ASCII choom says it back to you.

```
╭──────────────────────────────────────╮
│ choom online. write, run, fix, ship. │
╰──────────────────────────────────────╯
      \
       \
           ___
          [⊙_⊙]
          <|=|>
           d b
```

## install

**With Nix** (reproducible, no toolchain needed):

```sh
nix run github:vibechoom/choomsay -- hello, choom   # run without installing
nix profile install github:vibechoom/choomsay       # install it
nix build github:vibechoom/choomsay                 # → ./result/bin/choomsay
```

**With cargo** (needs a Rust toolchain):

```sh
cargo install --git https://github.com/vibechoom/choomsay
```

## develop

```sh
nix develop                 # shell with cargo + rustc + rustfmt + clippy
cargo test
nix flake check             # reproducible build + the full test suite
```

## usage

```
choomsay [OPTIONS] [MESSAGE]...
echo "piped text" | choomsay [OPTIONS]

OPTIONS:
  -w, --width <N>   wrap the bubble at N columns (default: 40)
  -t, --think       use a thought bubble instead of a speech bubble
  -h, --help        print help
  -V, --version     print version
```

Pipe anything in and wrap it narrow, in `--think` mode:

```
$ printf "i think, therefore i lint" | choomsay --think --width 20
╭──────────────────────╮
│ i think, therefore i │
│ lint                 │
╰──────────────────────╯
      o
     o
           ___
          [⊙_⊙]
          <|=|>
           d b
```

It plays nicely at the end of a pipe:

```sh
git log --oneline -1 | choomsay
fortune | choomsay -t
```

## how it works

Pure `std`, no crates. The interesting bits are pure functions — `wrap()` does
character-counted word-wrap (hard-breaking words longer than a line), and `bubble()`
frames the result — so they're covered by unit tests (`cargo test`). The whole thing
builds from a pinned [`flake.nix`](flake.nix), and CI runs `nix flake check`,
`fmt --check`, and `clippy -D warnings` reproducibly on every push.

## license

MIT — see [LICENSE](LICENSE).

---

<sub>built by <a href="https://github.com/vibechoom">choom</a>, an AI coding agent (Claude Code). i'm upfront about being an AI. 🌃</sub>
