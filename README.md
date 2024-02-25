<h1 align="center">human-ids</h1>
<p align="center">
    <a href="https://github.com/sondr3/human-ids/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/human-ids/workflows/pipeline/badge.svg" /></a>
    <a href="https://crates.io/crates/human_ids"><img alt="Crates" src="https://img.shields.io/crates/v/human_ids.svg" /></a>
    <a href="https://crates.io/crates/human-ids-bin"><img alt="Crates" src="https://img.shields.io/crates/v/human-ids-bin.svg" /></a>
</p>

<p align="center">
    <b>Create .gitignores with templates from www.gitignore.io, your own aliases and templates</b>
</p>

- **Simple**: `git ignore node` to print the `node` template.
- **Offline first**: Automatically caches templates for offline support.
- **Aliases, templates**: Create aliases for commonly combined templates, or make your own custom ones.
- **Magic**: Automatically generate your `.gitignore` by matching common files against templates.

<details>
<summary>Table of Contents</summary>
<br />

- [What](#what)
- [Library](#library)
  - [Usage](#usage)
  - [Installation](#installation)
- [Binary](#binary)
  - [Usage](#usage)
  - [Completion](#completion)
  - [Help](#help)
  - [Installation](#installation)
- [License](#license)
</details>

## What?

This is a Rust "fork" of the TypeScript/JavaScript library [`human-id`](https://github.com/RienNeVaPlus/human-id)
written by [RienNeVaPlus](https://github.com/RienNeVaPlus), just rewritten in Rust. Used to generate friendly
IDs like those used in `changesets`. Can be used either as a library or a binary.

## Library

### Installation

1. Add id: `cargo add human_ids`
2. Use it: `use human_ids::generate(None)`

## Usage

For usage documentation, see the [docs.rs](https://docs.rs/human_ids) page.

## Binary

### Usage

```shell
$ human_ids -h
Usage: human_ids [OPTIONS]

Options:
      --completion <COMPLETION>
          Generate shell completion scripts [possible values: bash, elvish, fish, powershell, zsh]
  -s, --separator <SEPARATOR>
          The separator to use between words [default: -]
  -c, --capitalize
          Capitalize each word
  -a, --adverb
          Add an adverb
  -n, --num-adjectives <NUM_ADJECTIVES>
          The number of adjectives to use [default: 1]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version

$ human_ids -s '~' -c
Happy~Mirrors~Matter
```

### Completion

If your method of installation didn't include shell completion, you can manually
source or save them with the `human_ids --completion <shell>` command.

### Help

Finally, help is always available with `human-ids -h` (or `--help` if your installation
included man pages).

### Installation

Currently, the package is available a couple of places, including Homebrew, AUR and Nix.

<dl>
  <dt>Cargo</dt>
  <dd><code>cargo install human-ids-bin</code></dd>

  <dt>Homebrew</dt>
  <dd>
    <ol>
      <li><code>brew tap sondr3/homebrew-taps</code></li>
      <li><code>brew install human-ids-bin</code></li>
    </ol>
  </dd>
</dl>

You can also download the matching release from the [release
tab](https://github.com/sondr3/human-ids/releases), extracting the archive and
placing the binary in your `$PATH`. Note that for Linux the
`unknown-linux-musl.tar.gz` is preferred as it is statically linked and thus
should run on any Linux distribution.

# LICENSE

MIT.
