# Bitcoin CLI in Rust

This is an optiniated alternative implementation in Rust of the `bitcoin-cli`
tool to interact with Bitcoin Core's RPC interface. 

## Project Status

We are currently in the ideation phase, don't expect to find a tool now.

Check the issues and tell us your chores with `bitcoin-cli` and how it could be
improved.

## The problem

The main tool provided by [Bitcoin-core](https://github.com/bitcoin/bitcoin) is
`bitcoind`, a daemon that implements full node, wallet and mining logic.
`bitcoin-cli` is a command line RPC client to interact with the node.

The following came as a project suggestion on the [2024 Chaincode Labs FOSS Program](https://learning.chaincode.com/#FOSS).

Some of the shortcomings of `bitcoin-cli`:
- Nonstandard single dash long arguments, e.g., `-datadir`, when most other CLI
  tools would use `--datadir`.
- Argument values must be given with an equals sign, e.g., `-datadir=foo`, when
  most other tools would accept both `--datadir=foo` and `--datadir foo`.
- No colored help output, and help text formatting could be improved.
- By default, arguments are passed as JSON which is verbose and hard to get
  right, especially when values must be quoted.
- Passing arguments by name is not the default, when it probably should be.
- Arguments are not validated up-front, but instead passed along to the RPC
  server as-is, which leads to poor error messages.
- No completion script.
- `bitcoin-cli` is a bit long to type.
- Like the rest of Bitcoin Core, `bitcoin-cli` is written in C++, which deters
  potential contributors, increases review burden, and is error prone and
  unsafe.

Improvements:
- The new binary could be written in Rust, which has excellent libraries for
  argument parsing and JSON serialization, including Bitcoin-specific libraries,
  namely `rust-bitcoin` and `rust-bitcoincore-rpc`, for parsing bitcoin types
  and interacting with `bitcoind`'s RPC interface.
- Many people want to learn Rust, write Rust, and use Rust-written CLI tools.
  Adding a way for people to contribute to Bitcoin Core using Rust would open
  the door to a *lot* of new contributors, and a rewite of `bitcoin-cli` in
  particular would provide an extremely approachable way for them to do so.
- The new binary could be called `bitcoin` or even `btc`, so it's easier to
  type.
- Eventually, the binary could support a subcommand to invoke `bitcoind`, e.g.,
  `bitcoin daemon`, to improve the same issues with `bitcoind` that
  `bitcoin-cli` has, e.g., argument parsing, argument validation, help text
  formatting, and completion script support.
- Long arguments could be accepted with `--arg` and short arguments with `-a`,
  which is standard and familiar among the vast majority of CLI tools.
- Values could be passed as standard named arguments, e.g., `--address foo`,
  instead of as JSON. The ability to pass JSON arguments could be retained with
  a subcommand for making arbitrary RPC calls which takes a single json
  argument: `bitcoin rpc --name generateblock --json SOME_JSON_BLOB`.
- Arguments could be validated up-front, instead of being passed to the RPC
  server. This would allow the binary to report which argument failed,
  specifically, instead of receiving a somewhat inscrutable error from the RPC
  server.

## Design

The binary is called `bitcoin`. 

``` shell
Usage:  bitcoin [options] <method> [--arg value]...  Send command to Bitcoin Core with named arguments
or:     bitcoin [options] man                        Display list of supported methods
or:     bitcoin [options] man <method>               Display manual page for a method
```

We want to have a coherent user interface, so we define a semantic structure.

**Options** are information that influences the behavior of the tool and don't
have connection to Core's RPC interface methods.

**Methods** are the available commands known and exposed by the RPC server.

**Arguments** are input data required by the specified method.

The `man`ual method is used to show the manual page of a specified method.
