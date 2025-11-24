# Installation

## Using Homebrew (macOS/Linux)

```bash
brew tap davelet/gim
brew install git-intelligence-message
```
or
```bash
brew install davelet/gim/git-intelligence-message
```

## Using Scoop (Windows)

```bash
scoop bucket add davelet https://github.com/davelet/scoop-gim.git
scoop install git-intelligence-message
```

## Downloading the Executable (Windows)

You can download the pre-built executable (gim.exe in `gim-{version}.zip`) from the [Releases](https://github.com/davelet/git-intelligence-message/releases) page.


## Using Cargo (Windows/Linux/macOS)

```bash
cargo install git-intelligence-message
```

## Build from source

```bash
git clone https://github.com/davelet/git-intelligence-message.git
cd git-intelligence-message
cargo install --path .
```
