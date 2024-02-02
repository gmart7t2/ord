![photo_2023-12-29 20 46 29](https://github.com/the-moto/xord-0.13.1/assets/140389355/c27aba47-f5e3-4ec9-b1a7-a20c066263fa)

`xord`, an advanced iteration of `ord`, introduces a suite of powerful new features designed to streamline and enrich the experience of interacting with Ordinals and digital artifacts on the Bitcoin blockchain. Building upon the foundational capabilities of `ord`, `xord` extends functionality with innovative indexing options and enhanced query capabilities, catering to a more sophisticated and diverse range of use cases for metaprotocol operations. Below is a brief introduction to the key enhancements incorporated into `xord`:

### New Indexing Flags

`xord` elevates the indexing process with additional flags, offering users more control and specificity in tracking Ordinal transactions and inscriptions. These new flags include:

- **--index-transfer-history**: Allows for the compilation of comprehensive transfer histories for individual Ordinals, providing a detailed view of their transaction journey.

- **--index-only-first-transfer**: Limits indexing to only the initial transfer of each Ordinal, offering a streamlined view of first ownership changes.

- **--filter-metaprotocol \<metaprotocol\>**: A specialized filter to index Ordinals based on specific metaprotocol criteria, shrinking the total size of your ord index.redb.

### JSON-RPC Metaprotocol Field Query

`xord` introduces the ability to query the "metaprotocol" field within Ordinal envelopes through JSON-RPC requests. This feature allows users to retrieve and interact with metaprotocol-specific data, enabling deeper insights and interactions with digital assets governed by these protocols.

### Default "First Inscription Height" now 820569

In a significant default setting adjustment, `xord` sets the "first inscription height" to align with the first deployment of a CBRC-20 token, specifically the 'BORD' token, at block height 820569. This change is aimed at enhancing the relevance and immediacy of data for users engaging with CBRC-20 tokens, providing a more intuitive starting point for tracking and analysis.

How to Run `xord`
------------

`xord` introduces a range of custom commands to enhance your interaction with the Bitcoin blockchain and Ordinals. Here's how to utilize these new features effectively:

### Example Command Breakdown

Let's dissect an example command to understand how to use `xord`'s custom features:

```sh
./ord --bitcoin-data-dir /yourpathtobitcoindata --bitcoin-rpc-pass themoto --bitcoin-rpc-user lovesyou --data-dir /yourpath/xord_test --index-only-first-transfer --filter-metaprotocol @ --filter-metaprotocol cbrc-20 server --http-port 4444 -j
```

#### 1. Bitcoin Data Directory
`--bitcoin-data-dir /yourpathtobitcoindata`: This flag specifies the directory where your Bitcoin node data is stored. In the example, it's set to `/Users/THEMOTO/Library/Application Support/Bitcoin`.

#### 2. Bitcoin RPC Authentication
- `--bitcoin-rpc-pass themoto`: Sets the password for Bitcoin RPC authentication.
- `--bitcoin-rpc-user lovesyou`: Sets the username for Bitcoin RPC authentication.

#### 3. `xord` Data Directory
`--data-dir /yourpath/xord_test`: This flag sets the directory for `xord` data. Here, it's `/Users/THEMOTO/xord`.

#### 4. Indexing Flags
- `--index-only-first-transfer`: Activates indexing for only the first transfer of each Ordinal, offering a focused view of initial ownership changes.
- `--filter-metaprotocol @ --filter-metaprotocol cbrc-20`: Filters the indexing process to only include Ordinals that are associated with the specified metaprotocol, in this case, `cbrc-20` and the addressage format `@`.

#### 5. Running the Server
`server`: This command starts the `xord` server.

#### 6. HTTP Port Configuration
`--http-port 4444`: Sets the HTTP port for the `xord` server. In this example, port 4444 is used.

#### 7. JSON-RPC Activation
`-j`: Enables JSON-RPC functionality, allowing for RPC interactions with `xord`.

Installation
------------

`ord` is written in Rust and can be built from
[source](https://github.com/ordinals/ord). Pre-built binaries are available on the
[releases page](https://github.com/ordinals/ord/releases).

You can install the latest pre-built binary from the command line with:

```sh
curl --proto '=https' --tlsv1.2 -fsLS https://ordinals.com/install.sh | bash -s
```

Once `ord` is installed, you should be able to run `ord --version` on the
command line.

Building
--------

On Debian and Ubuntu, `ord` requires `libssl-dev` when building from source:

```
sudo apt-get install libssl-dev
```

You'll also need Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To build `ord` from source:

```
git clone https://github.com/ordinals/ord.git
cd ord
cargo build --release
```

Once built, the `ord` binary can be found at `./target/release/ord`.

`ord` requires `rustc` version 1.67.0 or later. Run `rustc --version` to ensure you have this version. Run `rustup update` to get the latest stable release.

### Homebrew

`ord` is available in [Homebrew](https://brew.sh/):

```
brew install ord
```

### Debian Package

To build a `.deb` package:

```
cargo install cargo-deb
cargo deb
```

Syncing
-------

`ord` requires a synced `bitcoind` node with `-txindex` to build the index of
satoshi locations. `ord` communicates with `bitcoind` via RPC.

If `bitcoind` is run locally by the same user, without additional
configuration, `ord` should find it automatically by reading the `.cookie` file
from `bitcoind`'s datadir, and connecting using the default RPC port.

If `bitcoind` is not on mainnet, is not run by the same user, has a non-default
datadir, or a non-default port, you'll need to pass additional flags to `ord`.
See `ord --help` for details.

`bitcoind` RPC Authentication
-----------------------------

`ord` makes RPC calls to `bitcoind`, which usually requires a username and
password.

By default, `ord` looks a username and password in the cookie file created by
`bitcoind`.

The cookie file path can be configured using `--cookie-file`:

```
ord --cookie-file /path/to/cookie/file server
```

Alternatively, `ord` can be supplied with a username and password on the
command line:

```
ord --bitcoin-rpc-user foo --bitcoin-rpc-pass bar server
```

Using environment variables:

```
export ORD_BITCOIN_RPC_USER=foo
export ORD_BITCOIN_RPC_PASS=bar
ord server
```

Or in the config file:

```yaml
bitcoin_rpc_user: foo
bitcoin_rpc_pass: bar
```

Logging
--------

`ord` uses [env_logger](https://docs.rs/env_logger/latest/env_logger/). Set the
`RUST_LOG` environment variable in order to turn on logging. For example, run
the server and show `info`-level log messages and above:

```
$ RUST_LOG=info cargo run server
```
