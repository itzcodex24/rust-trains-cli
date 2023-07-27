# Train Times Rust CLI 🦀

- Need to know about upcoming train times?
- Use this Rust CLI to do just that!

## How to install

1. Run `git clone https://github.com/itzcodex24/rust-trains-cli train-cli`
2. Run `cd train-cli`
3. Run `cargo build --release`

## BOOM 💥

- Wanna add it to your path?

### Using ZSH 👨‍💻

- Run `nvim ~/.zshrc`
- Add the release to your path: `export PATH="$HOME/path_to_train_cli/target/release/":$PATH`
- Done! Refresh the config file: `source ~/.zshrc`
- And RUN! `train-cli --from {station} --to {station}`

### Does your station name include whitespace? 🚝

- Simply specify the station name while surrounding it in speech marks `""`
- `train-cli --from "{station}" --to "{station}"`
