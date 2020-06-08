# Namethingy

Procedural name generation using Markov chains

## Install

### Precompiled binaries

TBD

### Compiling from sources

Clone the repository and build manually. `cargo` is required

```shell script
$ cd namethingy
$ cargo build --release
```

## Quick Start

```shell script
$ namethingy --limit 10 --order 3 --corpus $PWD/datasets/greek.txt
Muses
Teutamon
Dysis
Aidotheus
Poseisus
Penthoös
Argis
Iphigenor
Styx
Krotos
```