# Fox

![Bud Fox and Gordon Gekko](https://pophistorydig.com/wp-content/uploads/2010/02/bud-fox-gordon-gekko-310.jpg)

*The mother of all evil is speculation.*
- Gordon Gekko to Bud Fox

### Installation

Welcome to my crypto speculation bot.
You must have Rust installed.
You must also have your DYDX API key, passphrase, secret, STARK private key, and Ethereum address in your .env file or as an environment variable.

1. Clone the repo
2. Run `cargo build --release`
3. Execute the binary at <path_to_repo>/target/release/fox
    - Usage `./fox --strategy <Strategy>`

This is a work in progress. There are a lot of values hard-coded in that will need to be changed by the user.

Yes, the name is basically the same as the python Bitcoin trading bot [gekko](https://github.com/askmike/gekko). The name was too good to pass up.
