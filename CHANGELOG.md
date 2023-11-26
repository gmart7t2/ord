Changelog
=========

[0.8.2-gm7](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm7) - 2023-11-25
--------------------------------------------------------------------------------

### Changed
Fixed coin selection algorithm (#2723).

[0.8.2-gm6](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm6) - 2023-08-31
--------------------------------------------------------------------------------

### Added
-- Add `--wait-after-commit` flag to `wallet inscribe`. This causes the reveal transactions not to be broadcast until the commit transaction has been confirmed. Use in conjunction with `--dump` to make sure you have a copy of the reveal transactions to prevent loss of funds in the event that the ord command gets interrupted before the commit transaction confirms.

[0.8.2-gm5](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm5) - 2023-08-31
--------------------------------------------------------------------------------

### Added
-- Add `--no-signature` flag to `wallet inscribe`. It leaves the reveal transaction unsigned, making it smaller, cheaper, and trivially easy to steal. Do not use!

[0.8.2-gm4](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm4) - 2023-08-25
--------------------------------------------------------------------------------

### Changed
-- Fix `wallet inscriptions` so it can list multiple inscriptions on the same sat.
-- Fix help string for `wallet inscriptions --address`.
-- Add `--allow-missing-outputs` flag to allow ord to work on wallets with old outputs.
-- Create savepoint if it has been `SAVEPOINT_INTERVAL` blocks since the last one.

[0.8.2-gm3](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm3) - 2023-08-24
--------------------------------------------------------------------------------

### Changed
-- Fix `--single-key` when inscribing the same file multiple times.
-- Add logging to the new server endpoints.

[0.8.2-gm2](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm2) - 2023-08-23
--------------------------------------------------------------------------------

### Changed
-- Store 4 bytes per outpoint not 3, to reduce the number of collisions.

[0.8.2-gm1](https://github.com/gmart7t2/ord/releases/tag/0.8.2-gm1) - 2023-08-22
--------------------------------------------------------------------------------

### Changed
-- Merged 0.8.2 from upstream.
-- Small fix to reorg logging.

[0.8.2](https://github.com/ordinals/ord/releases/tag/0.8.2) - 2023-08-17
---------------------------------------------------------------------

### Added

- Allow setting custom postage (#2331)
- Make retrieving inscriptions in block fast (#2333)
- JSON API for `/inscription`, `/inscriptions` and `/output` (#2323)
- Ignore invalid content type header values (#2326)
- Add reorg resistance  (#2320)
- Add JSON API endpoint `/sat/<SAT>` (#2250)
- Add `amount` field to `wallet inscriptions` output. (#1928)

### Changed

- Only fetch inscriptions that are owned by the ord wallet (#2310)
- Inform user when redb starts in recovery mode (#2304)
- Select multiple utxos (#2303)


### Fixed

- Use `--fee-rate` when sending an amount (#1922)
- Fix typos in documentation (#2328)
- Fix dust limit for padding in `TransactionBuilder` (#1929)
- Fix remote RPC wallet commands (#1766)

[0.8.1-gm23](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm23) - 2023-08-21
----------------------------------------------------------------------------------

### Changed
- Use redb v1.1.0 to fix compact panic.

[0.8.1-gm22](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm22) - 2023-08-20
----------------------------------------------------------------------------------

### Added
- Add `--allow-reveal-rbf` and `--unfunded-reveal` to `wallet inscribe`.

[0.8.1-gm21](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm21) - 2023-08-17
----------------------------------------------------------------------------------

### Added
- Add `--ignore-utxo-inscriptions` flag to `wallet inscribe` to allow inscribing on utxos that already contain inscriptions.

[0.8.1-gm20](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm20) - 2023-08-16
----------------------------------------------------------------------------------

### Added
- Add `--cursed-utxo` flag to `wallet inscribe` to specify which utxo to use for the first input to the cursed reveal tx.

[0.8.1-gm19](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm19) - 2023-08-16
----------------------------------------------------------------------------------

### Added
- Add `--cursed-destination` flag to `wallet inscribe` to specify where the cursing reveal input should be sent.

[0.8.1-gm18](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm18) - 2023-08-09
----------------------------------------------------------------------------------

### Added
- Add `--skip-empty-outputs` flag to avoid indexing any 0-sat utxos. This is experimental and could well break things. Or it might make the index smaller or faster or both. Let's see, shall we?

[0.8.1-gm17](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm17) - 2023-08-09
----------------------------------------------------------------------------------

### Changed
- Fix tracking of sats 458610000000000 and 459060000000000 from the duplicate coinbase blocks. There are two pairs of duplicate coinbase txs (see blocks 91722, 91880 and 91812, 91842). Now the overshadowed sats are moved to the lost sats list.

[0.8.1-gm16](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm16) - 2023-08-08
----------------------------------------------------------------------------------

### Added
- Add `--commit` flag to set how often to write changes to disk when indexing.

[0.8.1-gm15](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm15) - 2023-08-08
----------------------------------------------------------------------------------

### Added
- Add `--show-height` flag to `ord find`.

[0.8.1-gm14](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm14) - 2023-08-07
----------------------------------------------------------------------------------

### Added
- Add `--date-format` flag to `ord find`.

### Changed
- Allow `ord find` to work when part of the range hasn't been mined yet.

[0.8.1-gm12](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm12) - 2023-08-06
----------------------------------------------------------------------------------

### Added
- Add flags `--show-date`, and `--show-value` to `ord find` to add fields to the results.

[0.8.1-gm11](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm11) - 2023-08-06
----------------------------------------------------------------------------------

### Added
- Add flags `--show-address`, `--show-blockhash`, `--show-name`, `--show-time` to `ord find` to add fields to the results.

### Changed
- Store 3 bytes per outpoint not 2, to reduce the number of collisions.

[0.8.1-gm10](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm10) - 2023-08-06
----------------------------------------------------------------------------------

### Added
- Add `compact` subcommand to attempt compacting the database.
- Add `--file` flag to `find` to allow specifying sats and ranges to search for in a file.
- Add `--ignore` flag to `find` to ignore unmined sats and ranges while searching.

### Changed
- Check the reveal transaction weights against the mempool limit after signing them.

[0.8.1-gm9](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm9) - 2023-08-05
--------------------------------------------------------------------------------

### Added
- Add `--address` flag to `wallet inscriptions` to filter inscriptions by address.
- Add an index to speed up `ord find`. Enable it with `--index-utxos` when creating `index.redb`.

[0.8.1-gm8](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm8) - 2023-08-03
--------------------------------------------------------------------------------

### Changed
- Renamed the old `--cursed` flag that attempted to make a cursed inscription using OP_66 to `--cursed66`. The inscriptions it creates are ignored by ord's indexer.

### Added
- Add `--check-index` subcommand to passively check the index file.
- Add `--cursed` flag to create cursed inscriptions by inscribing in the 2nd reveal input instead of the 1st.

The addition of the `--cursed` flag was requested and funded by mone (https://x.com/Luckycharm_NFT). He inscribed a cursed inscription and then reinscribed the same sat with a recursive non-cursed inscription. See https://ordinals.com/sat/1931140000000000 for the two inscriptions.

[0.8.1-gm7](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm7) - 2023-07-29
--------------------------------------------------------------------------------

### Changed
- Move the check for being able to write a file to before we send the commit tx.
- Merge from upstream.

[0.8.1-gm6](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm6) - 2023-07-25
--------------------------------------------------------------------------------

### Changed
- Merge from upstream. `wallet inscriptions` now labels the postage amount as `postage` not as `amount`.

[0.8.1-gm5](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm5) - 2023-07-25
--------------------------------------------------------------------------------

### Added
- Tell the user if their database is corrupted before starting to repair it.

[0.8.1-gm4](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm4) - 2023-07-25
--------------------------------------------------------------------------------

### Added
- Add `/stats` endpoint to get the highest block indexed and the range of inscriptions that exist.

[0.8.1-gm3](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm3) - 2023-07-25
--------------------------------------------------------------------------------

### Added
- Add request logging to the server when RUST_LOG=INFO is set.
- Add `--no-progress-bar` flag to inhibit the display of the progress bar.

[0.8.1-gm2](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm2) - 2023-07-24
--------------------------------------------------------------------------------

### Changed
- Add `transfer` subcommand for modifying the transfer log table.

[0.8.1-gm1](https://github.com/gmart7t2/ord/releases/tag/0.8.1-gm1) - 2023-07-23
--------------------------------------------------------------------------------

### Changed
- Merged 0.8.1 from upstream.
- Recognize and display "text/plain" content type as text inscriptions.
- Don't try indexing new blocks when there are no new blocks
- Sleep 10 seconds between checking for new blocks

[0.8.1](https://github.com/ordinals/ord/releases/tag/0.8.1) - 2023-07-23
---------------------------------------------------------------------

### Added

- Add retry to fetcher (#2297)
- Add satpoint and address to index export (#2284)
- Don't create default data directory if --index overrides it (#1991)
- Implement clean index shutdown to prevent index corruption (with clippy updates for Rust 1.71) (#2275)
- Set lower max age for not found (#2240)

### Changed

- Fix justfile recipe (#2299)
- Clean up deploy scripts (#2298)
- Update redb (#2294)
- Update bitcoin dependencies (#2281)
- Fix ordering for reinscriptions and show all reinscriptions for sat (#2279)
- Modify `ord list` output to include the end of each range (#1998)

### Documentation

- Fix docs inconsistency (#2276)
- Add contributing section (#2261)

[0.8.0-gm19](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm19) - 2023-07-21
----------------------------------------------------------------------------------

### Changed
- Merged master branch from upstream.

[0.8.0-gm18](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm18) - 2023-07-21
----------------------------------------------------------------------------------

### Changed
- Avoid fetching the same tx twice.

[0.8.0-gm17](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm17) - 2023-07-21
----------------------------------------------------------------------------------

### Changed
- Increase the limit on /inscriptions_json from 100 to 1000.
- Remove `timestamp` from the `sat` object. It's too slow to look up.

[0.8.0-gm16](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm16) - 2023-07-21
----------------------------------------------------------------------------------

### Changed
- Use `getblockstats` instead of `getblockhash | getblock` to look up a block's time.

[0.8.0-gm15](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm15) - 2023-07-20
----------------------------------------------------------------------------------

### Changed
- Increase the limit on /inscriptions_json from 100 to 1000.
- Attempt to stop blocking the server when fetching a lot of inscriptions.

[0.8.0-gm14](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm14) - 2023-07-17
----------------------------------------------------------------------------------

### Added
- Add `/inscriptions_json/:start/:end` server endpoint.

[0.8.0-gm12](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm12) - 2023-07-15
----------------------------------------------------------------------------------

### Added
- Write to `error.txt` in the current directory when there's a reorg detected.

### Changed
- Wait 1 second between each check for new blocks in `ord server`.

[0.8.0-gm11](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm11) - 2023-07-14
----------------------------------------------------------------------------------

### Added
- Added a server page listing the sent inscriptions per block.

[0.8.0-gm10](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm10) - 2023-07-14
----------------------------------------------------------------------------------

### Changed
- Merge ordinals/master to get clippy fixes.

[0.8.0-gm9](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm9) - 2023-07-14
--------------------------------------------------------------------------------

### Changed
- Show an error message when there's no inscription envelope to decode.
- Decode multiple envelopes to multiple files.

[0.8.0-gm8](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm8) - 2023-07-14
--------------------------------------------------------------------------------

### Changed
- Add filenames, commit weight, and reveal weights to `--dump` file (#7)

[0.8.0-gm7](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm7) - 2023-07-14
--------------------------------------------------------------------------------

### Changed
- Force graceful shutdown on server error - another index corruption fix (#2270)

[0.8.0-gm6](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm6) - 2023-07-13
--------------------------------------------------------------------------------

### Changed
- Gracefully shutdown update thread on main run error - another index corruption fix (#2270)

[0.8.0-gm5](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm5) - 2023-07-13
--------------------------------------------------------------------------------

### Changed
- Join update thread after main run completes - another index corruption fix (#2270)

[0.8.0-gm4](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm4) - 2023-07-13
--------------------------------------------------------------------------------

### Changed
- Merged victorkirov's PR to stop `ord server` corrupting the index file (#2270)

[0.8.0-gm3](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm3) - 2023-07-07
--------------------------------------------------------------------------------

### Changed
- Have `ord inscriptions` list negative numbered inscriptions too

[0.8.0-gm2](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm2) - 2023-07-07
--------------------------------------------------------------------------------

### Changed
- Don't fail when listing unbound inscriptions

[0.8.0-gm1](https://github.com/gmart7t2/ord/releases/tag/0.8.0-gm1) - 2023-07-01
--------------------------------------------------------------------------------

### Changed
- Merged 0.8.0 from upstream

[0.8.0](https://github.com/ordinals/ord/releases/tag/0.8.0) - 2023-07-01
---------------------------------------------------------------------

### Added

- Dev server deploy script (#2228)
- Set DB cache size (#2224)
- Update redb from 0.13.0 to 1.0.2 (#2141)
- Fix typo in BIP (#2220)

[0.7.0-gm2](https://github.com/gmart7t2/ord/releases/tag/0.7.0-gm2) - 2023-06-28
--------------------------------------------------------------------------------

### Changed
- Skip updating the index completely when `--height-limit` is 0.

[0.7.0-gm1](https://github.com/gmart7t2/ord/releases/tag/0.7.0-gm1) - 2023-06-24
--------------------------------------------------------------------------------

### Changed
- Merged 0.7.0 from upstream

[0.7.0](https://github.com/ordinals/ord/releases/tag/0.7.0) - 2023-06-23
---------------------------------------------------------------------

### Added
- Tweak publish recipe (#2212)
- Handle cursed inscriptions edge cases (#2209)
- Add export command for <INSCRIPTION_NUMBER_TO_INSCRIPTION_ID> table (#2208)
- Add Markdown media type (#2206)
- Add blob urls to Content Security Policy headers (#2203)
- Check inscribe destination address network (#2189)

[0.6.2-gm1](https://github.com/gmart7t2/ord/releases/tag/0.6.2-gm1) - 2023-06-22
--------------------------------------------------------------------------------

### Changed
- Allow `ord inscriptions` to display unbound inscriptions with positive inscription numbers.
- Merged 0.6.2 from upstream
- Check inscribe destination address network (#2189)
- Improve error messages when parsing a broken CSV file
- Complain if the user tries to use --coin-control or --utxo when sending cardinals

[0.6.2](https://github.com/ordinals/ord/releases/tag/0.6.2) - 2023-06-15
---------------------------------------------------------------------

### Added
- Recursive endpoints: `/blockhash, /blockheight, /blocktime` (#2175)
- Document recursion (#2174)
- Add CSS and JavaScript media types (#2173)
- Recursive Inscriptions (#2167)

### Misc
- Update ord dependency in lockfile (#2168)

[0.6.1-gm5](https://github.com/gmart7t2/ord/releases/tag/0.6.1-gm5) - 2023-06-19
--------------------------------------------------------------------------------

### Changed
- When making inscriptions, write any reveal transactions that fail to broadcast to a file for manual broadcast later.

[0.6.1-gm4](https://github.com/gmart7t2/ord/releases/tag/0.6.1-gm4) - 2023-06-14
--------------------------------------------------------------------------------

### Changed
- Merged "Recursive Inscriptions (#2167)" from upstream.

[0.6.1-gm3](https://github.com/gmart7t2/ord/releases/tag/0.6.1-gm3) - 2023-06-14
--------------------------------------------------------------------------------

### Changed
- Add `--single-key` flag to `wallet inscribe` to use the same temporary private key for all inscriptions.
- Remove the `--wait-after-commit` flag from `wallet inscribe`. It's too hard to use safely. Instead, use `--dump --no_broadcast --no_backup`, save the output to a file, and broadcast the transactions manually.

[0.6.1-gm2](https://github.com/gmart7t2/ord/releases/tag/0.6.1-gm2) - 2023-06-14
--------------------------------------------------------------------------------

### Changed
- Give a reasonable error message when `ord inscriptions --id <inscriptionid>` fails to find the inscription.

[0.6.1-gm1](https://github.com/gmart7t2/ord/releases/tag/0.6.1-gm1) - 2023-06-07
--------------------------------------------------------------------------------

### Changed
- Merged in releases 0.6.0 and 0.6.1 from upstream.

[0.6.1](https://github.com/ordinals/ord/releases/tag/0.6.1) - 2023-06-06
---------------------------------------------------------------------

### Changed
- Fix sat index test and unbound assignment (#2154)
- Updated install.sh for new repo name (#2155)

[0.6.0](https://github.com/ordinals/ord/releases/tag/0.6.0) - 2023-06-04
---------------------------------------------------------------------

### Added
- Cursed Inscriptions [1/n] (#2145)
- Authenticate to bitcoin using a username and password (#1527)
- Add example config file (#2044)

### Changed
- Unbind inscriptions from zero-sat transactions (#2107)

### Documentation
- Tweak doc: Inscriptions made on first sat of input (#2148)
- `OP_PUSH` instead of `OP_1` in inscription docs (#2135)
- Document bitcoind RPC authentication options (#2056)
- Fix typo in Sparrow Wallet docs (#2077)
- Update donate.md for inscriptions donations. (#2125)
- Promote raphjaph to lead maintainer 🫡 (#2119)
- Improve donation page (#2034)

### Misc
- Switch CI back to stable clippy (#2108)
- Update dependencies (#2068)
- Use struct variants in Origin enum (#2067)
- Fix test name typos(#2043)
- Switch to nightly clippy (#2037)

[0.5.2-gm9](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm9) - 2023-05-30
--------------------------------------------------------------------------------

### Changed
- Check that at least one filename is provided when inscribing.
- Merged in latest master branch from casey's repo, including "Unbind inscriptions from zero-sat transactions (#2107)".

[0.5.2-gm8](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm8) - 2023-05-24
--------------------------------------------------------------------------------

### Added
- Add `--allow-reinscribe` to `wallet inscribe` to allow reinscribing on a sat that already has an inscription. `ord` won't recognize such an inscription yet but apparently there are plans to assign such inscriptions negative inscription numbers. It's best to store such inscriptions in a separate wallet until `ord` recognizes them to avoid accidentally spending them. At the time of writing not even cursedordinals.com recognizes inscriptions on sats that are already inscribed.

[0.5.2-gm7](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm7) - 2023-05-24
--------------------------------------------------------------------------------

### Added
- Add `--cursed` to `wallet inscribe` to create an inscription that won't be recognized by current versions of ord, but that will be recognized by some indexers as "cursed".

[0.5.2-gm6](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm6) - 2023-05-23
--------------------------------------------------------------------------------

### Changed
- Speed wallet commands up by only fetching relevant inscriptions from the database.

[0.5.2-gm5](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm5) - 2023-05-17
--------------------------------------------------------------------------------

### Added
- Add `--change` flag to `wallet send` and `wallet inscribe` to say where any change will be sent.
- Merged in latest master branch from casey's repo, including new flags `--bitcoin-rpc-user` and `--bitcoin-rpc-pass`.

### Changed
- Ignore BOM (byte order marker) in .csv files.

[0.5.2-gm4](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm4) - 2023-04-26
--------------------------------------------------------------------------------

### Changed
- Shorten the `--destination-csv` flag for `wallet inscribe` to just `--csv`.
- If no `--destination` or `--csv` flag is provided to `wallet inscribe`, generate a different destination address for each inscription.
- Allow multiple `--destination` flags to `wallet inscribe`. If there are less destinations than files to inscribe, cycle through the destinations.

[0.5.2-gm3](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm3) - 2023-04-25
--------------------------------------------------------------------------------

### Added
- Add `--destination-csv` flag to `wallet inscribe` to provide a file containing a list of `<destination>,<filename>` pairs when inscribing.

### Changed
- Don't have `--no-broadcast` imply `--no-backup` in `wallet inscribe`..

[0.5.2-gm2](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm2) - 2023-04-21
--------------------------------------------------------------------------------

### Added
- Add `amount` and `content-type` fields to `ord inscriptions --number` and `ord inscriptions --id` output.

[0.5.2-gm1](https://github.com/gmart7t2/ord/releases/tag/0.5.2-gm1) - 2023-04-19
--------------------------------------------------------------------------------

### Added
- Add `--id` and `--number` flags to `ord inscriptions` to list specific inscriptions by inscriptionid or by inscription number.

[0.5.2](https://github.com/ordinals/ord/releases/tag/0.5.2) - 2023-04-17
---------------------------------------------------------------------

### Added
- Add `ord wallet cardinals` command to list the cardinal outputs (#1904)

### Changed
- Shut down immediately after two interrupts (#2008)
- Mandatory fee rate for inscribe (#1897)
- Add error when a satpoint's offset exceeds the size of its output (#1857)

### Fixed
- Fix fee-spent inscription tracking (#1971)
- Label change and receive addresses correctly (#1847)
- Correct reveal tx fee calculation (#1853)

### Misc
- Misc changes (#2025)
- Misc doc fixes (#2021)
- Typo in sparrow wallet guide (#1947)
- Miscellaneous design improvements (#1968)
- Update miniscript dependency to 9.0.1 (#1966)
- Skip indexing inscriptions when below first inscription also for `--index-sats`(#1828)
- Better interrupt message (#1874)
- Fix colored coins link in BIP (#1856)
- Added cozy pair programming twitch link to README.md (#1827)
- Create rpc client after updating index (#1731)
- Add additional err msg to build from source for users who's arch falls outside of the list (#1792)
- Add note on default build location (#1625)
- Minor copy fixes (#1730)
- Typo (#1815)

[0.5.1-gm18](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm18) - 2023-04-18
----------------------------------------------------------------------------------

### Changed
- Drop the bitcoin connection while waiting for commit to confirm.

[0.5.1-gm17](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm17) - 2023-04-18
----------------------------------------------------------------------------------

### Changed
- While waiting for commit tx to confirm, don't fail if we are temporarily unable to talk to the bitcoin client. Attempt reconnecting to it.

[0.5.1-gm16](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm16) - 2023-04-17
----------------------------------------------------------------------------------

### Changed
- Close the index database before starting to wait for the commit tx to confirm. This allows other instances of ord to run while we're waiting.

[0.5.1-gm15](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm15) - 2023-04-16
----------------------------------------------------------------------------------

### Added
- Add `--max-inputs` flag to `wallet send` (for inscriptions and satpoints) and `wallet inscribe` to limit the number of inputs in the transactions they make.

### Changed
- Only add the `sat` field to the output of `wallet inscriptions` if the full sat index is in use.

[0.5.1-gm14](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm14) - 2023-04-14
----------------------------------------------------------------------------------

### Added
- Add `--wait-after-commit` flag to `wallet inscribe` to have ord wait for the commit transaction to confirm before sending reveal transaction(s).

Note that it could take a long time for the commit transaction to commit, and the reveal transactions aren't broadcast until it does. If you are running ord over an ssh connection and the connection times out while it is waiting the ord process will probably be killed and the reveal transactions will never be broadcast, leaving the commit outputs stranded.

To prevent this, you can use `--dump` to cause all the raw transactions to be printed to standard output before anything is broadcast. Redirect the output to a file to capture it:

`ord wallet inscribe --dump --fee-rate 10 --wait-for-commit > ord.log`

If you are using --dump like that you can also use `--no-backup` to avoid the slow process of saving all the recovery keys to the wallet because they will be written to standard output.

Also, if running ord using an ssh connection you might consider running it inside a `screen` session so that it will survive even if the ssh connection times out.

[0.5.1-gm13](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm13) - 2023-04-14
----------------------------------------------------------------------------------

### Added
- Add flag `--order-by-sat` to list in order of inscribed sat number.
- Add flags to allow limiting output in various ways (`--max-sat`, `--max-number`, `--max-height`).

### Changed
- Change default behavior of `ord inscriptions` to list in order of inscription number.
- Modify it to work without the full sats index
- Renamed `--max` flag to `--limit`.

[0.5.1-gm12](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm12) - 2023-04-12
----------------------------------------------------------------------------------

### Added
- Add the inscription number and the satoshi number to the `wallet inscriptions` output.
- Add `--no-broadcast` flag to `wallet inscribe` to prevent it broadcasting any transactions.
- Add `--coin-control` flag to `wallet inscribe` and `wallet send` to have those commands only spend utxos given with the `--utxo` flag.
- Add `decode` command to dump the contents of an inscription to a file, even if it isn't confirmed yet.

### Changed
- Alter the behavior of `--dump` in `wallet inscribe`. It no longer inhibits transaction broadcast.
- Fixed a couple of typos.

[0.5.1-gm11](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm11) - 2023-04-07
----------------------------------------------------------------------------------

### Added
- Add `decode` subcommand to parse inscription data from specified tx input.
- Check the commit transaction weight against the standardness limit when inscribing.

[0.5.1-gm10](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm10) - 2023-04-06
----------------------------------------------------------------------------------

### Changed
- Have `wallet inscribe --dump` also display the recovery descriptors.

[0.5.1-gm9](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm9) - 2023-04-06
--------------------------------------------------------------------------------

### Added
- Add `--dump` flag to `wallet inscribe` to have it dump the raw hex transactions rather than broadcasting them.

[0.5.1-gm8](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm8) - 2023-04-05
--------------------------------------------------------------------------------

### Changed
- Allow multiple files to be inscribed by a single `wallet inscribe` command.

[0.5.1-gm7](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm7) - 2023-04-03
--------------------------------------------------------------------------------

### Changed
- Add `--outpoint` flag to `find` to allow searching in specific outpoints.
- Add the end of the range and the offset into the outpoint to the output of the `list` subcommand.
- Add `--uncommon` flag to `inscriptions` to only list inscriptions on sats that are uncommon or rarer.

[0.5.1-gm6](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm6) - 2023-04-01
--------------------------------------------------------------------------------

### Added
- Add `inscriptions` command to list all inscriptions. (#1996)

[0.5.1-gm5](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm5) - 2023-03-31
--------------------------------------------------------------------------------

### Changed
- Don't try to create the default data directory if `--index` overrides it. Create the specified directory instead. (#1991)

### Added
- Add flags to allow setting the postage amount for `wallet inscribe` and `wallet send`. (#1994)

[0.5.1-gm4](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm4) - 2023-03-30
--------------------------------------------------------------------------------

### Added
- Accept a second parameter to `find` to allow finding ranges of sats, not just single sats. (#1992)

[0.5.1-gm3](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm3) - 2023-03-26
--------------------------------------------------------------------------------

### Misc
- Replaced the `--unconfirmed` flag for `wallet send` and `wallet inscribe` with `--utxo` to allow providing multiple unconfirmed UTXOs for use when sending or inscribing. It is the user's responsibility to ensure the provided UTXOs contain no inscriptions. `ord` won't check that for you.

[0.5.1-gm2](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm2) - 2023-03-25
--------------------------------------------------------------------------------

### Added
- Add `--alignment` flag to `wallet send` and `wallet inscribe` to allow setting which address to send the alignment padding to.
- Add `--unconfirmed` flag to `wallet send` and `wallet inscribe` to allow sending from and inscribing on satpoints in unconfirmed utxos.
- Calculate fees more accurately by not rounding the vsize of transactions up to an integer before multiplying by the fee rate.

### Changed
- Fix a bug in the tracking of inscriptions that have been spent as fees (#1841)

[0.5.1-gm1](https://github.com/gmart7t2/ord/releases/tag/0.5.1-gm1) - 2023-03-18
--------------------------------------------------------------------------------

### Added
- Add `wallet cardinals` command to list the cardinals (#1904)
- Include the value in sats of inscriptions in `wallet inscriptions` (#1928)

### Changed
- Improve input selection when inscribing (#1858)
- Require users manually specify a `--fee-rate` for `wallet inscribe` (#1897)
- Allow `wallet send` to send the last inscription from an output (#1927)

### Documentation
- Fix typo in Sparrow Wallet Guide (#1947)

### Misc
- Use receiving addresses to receive, and change addresses for change (#1847)
- Use `--fee-rate` to set the fee when sending cardinals (#1922)
- Fix an obscure error when sending satpoint `x:x:300` to a segwit bech32 address (#1929)

[0.5.1](https://github.com/ordinals/ord/releases/tag/0.5.1) - 2023-02-21
---------------------------------------------------------------------

### Performance
- Batch tx requests and re-enable skipping transactions (#1759)

### Added
- Add option to set inscription destination address (#1536)
- Allow supplying passphrase for `ord wallet create` and `ord wallet restore` (#1669)
- Add `--config-dir` option (#1697)

### Changed
- Require users manually specify a `--fee-rate` for `wallet send` (#1755)

### Documentation
- Add Sparrow Wallet Guide to Handbook (#1742)

### Misc
- Handle block count RPC error gracefully (#1637)
- Fix typos in overview.md (#1715)
- Typo fix (#1682)
- README typo fix (#1716)
- Fix changelog dates: 2022 → 2023 (#1700)
- Bump version number (#1695)

[0.5.0](https://github.com/ordinals/ord/releases/tag/0.5.0) - 2023-02-11
---------------------------------------------------------------------

### Breaking Changes
- Upgrade to redb 0.13.0 (#1513)
- Update redb to 0.12.1 (#1329)
- Display inscription genesis fee (#1381)

### Added
- Add support for `.glb` inscriptions (#1689)
- Add --no-limit flag to bypass MAX_STANDARD_TX_WEIGHT check to allow four meggers (#1571)
- Add `--commit-fee-rate` for setting inscribe commit transaction fee rate (#1490)
- Allow viewing but not creating AVIF inscriptions (#1428)
- Support STL inscriptions (#1492)
- Support MP4 inscriptions (#1419)
- Preview JSON and YAML inscriptions as text (#1449)
- Display inputs on /tx (#1433)
- Support PGP signature inscriptions (#1413)
- Add config (#1392)
- Add paging to /inscriptions (#1279)

### Changed
- Increase deployment mempool size to 1024 megabytes (#1587)
- Increase number of inscriptions in RSS feed (#1567)
- Link to block from /inscription (#1395)
- Use favicon as icon for Twitter preview (#1425)
- Allow data URIs in content security policy (#1422)
- Raise server open file limit (#1408)
- Remove HTTP to HTTPS redirect (#1414)
- Use JSON for more command output (#1367)
- Use JSON for `wallet` command output (#1359)

### Misc
- Set rustc version in Cargo.toml & README (#1615)
- Disable Prettier format-on-save (#1593)
- Add build instructions to README (#1573)
- Ensure wallet commands load wallet (#1524)
- Improve error messages related to cookie file (#1537)
- Include inscription ID in text inscription decode error (#1540)
- Lazily load iframes (#1456)
- Log recoverable errors as warnings
- Add alert pop-up example (#1498)
- Use custom Discord invite link in handbox (#1506)
- Note that bounty 3 requires sat index (#1509)
- Link donation addresses to mempool.space (#1510)
- Add linebreak to donate page (#1500)
- Add donate page to handbook (#1499)
- Moderation guide typo: wiht → with (#1483)
- Add moderation guide (#1473)
- Add collecting guide to docs (#1474)
- Add missing dependencies to shell.nix (#1463)
- Mute and autoplay video inscriptions (#1420)
- Throw an error Bitcoin Core wallet and ord index are out of sync (#1459)
- Typo: managment -> management (#1441)
- Fix README.md grammar (#1406)
- Typo: Aritacts -> Artifacts (#1434)
- Update justfile to use unproxied domains (#1391)
- Typo: sat -> sats (#1411)
- Docs: `ord wallet utxos` -> `ord wallet outputs` (#1405)
- Round expected sat timestamps (#1386)
- Remove ellipsis (#1376)
- Hide overflowing ordered lists (#1384)
- Compress responses (#1366)
- Avoid listening on 0.0.0.0 in tests (#1365)
- Rename `GitHub` nav link to `Wallet` (#1360)

[0.4.2](https://github.com/ordinals/ord/releases/tag/0.4.2) - 2023-01-24
---------------------------------------------------------------------

### Changed
- Fetch transactions below first inscription height

### Fixed
- Fix install script directory (#1356)

### Misc
- Fix guide typo: getblockchount -> getblockcount (#1354)

[0.4.1](https://github.com/ordinals/ord/releases/tag/0.4.1) - 2023-01-24
---------------------------------------------------------------------

### Added
- Support video inscriptions (#1349)
- Support PDF Inscriptions (#1352)
- Display lost sats on /output (#1336)
- Show explorer URLs in `ord wallet inscriptions` (#1308)

### Changed
- Display timestamps as UTC (#1348)
- Enable pointer events on inscription page iframes (#1344)
- Exclude inscribed utxos when calculating wallet balance (#1341)

### Misc
- Activate nav arrows on single tap on iOS Safari (#1347)
- Ignore keyboard events search box has focus (#1346)
- Cache content responses (#1318)
- Show unordered list decorations (#1353)
- Update inscriptions guide for mainnet (#1342)
- Hide list overflow and break dl overflow between words (#1343)
- Add white on black fish eye logo (#1325)
- Un-reverse thumbnail row order (#1321)
- Deploy branches other than master to mainnet (#1319)
- Add Just recipe to deploy to all chains (#1313)
- Display newest inscriptions on right (#1311)
- Allow publishing arbitrary revisions with publish recipe (#1307)
- Make genesis clock mark orange and add tooltip to height (#1312)
- Serve favicon as PNG to Safari and SVG others (#1302)
- Use sans-serif font for height on clock (#1300)

[0.4.0](https://github.com/ordinals/ord/releases/tag/0.4.0) - 2023-01-19
---------------------------------------------------------------------

### Added
- Support searching for inscription IDs (#1294)
- Add RSS feed (#1229)
- Add --dry-run-flag (#1265)
- Support recovering wallet from mnemonic (#1215)
- Audio inscriptions (#1241)
- Allow using custom fee rate (#1150)
- Show timestamp on /inscription (#1200)
- Add prev and next links to /inscription (#1193)
- Show address on /inscription (#1187)
- Add --limit to `ord wallet transaction` (#1049)
- Add `ord preview` (#1089)
- Add `ord wallet balance` (#1047)
- Support HTML and SVG inscriptions (#1035)
- Display genesis height on inscription page (#1026)
- Support more image types (#1020)
- Support GIFs (#1013)

### Changed
- Poll Bitcoin Core less frequently (#1268)
- Automatically load wallet (#1210)
- Ignore inscriptions on sat after first (#1214)
- Allow right-click to save image inscriptions (#1218)
- Scale text inscriptions to fit preview (#1222)
- Convert `ord wallet inscriptions` to JSON (#1224)
- Improve error when preview fails to launch bitcoind (#1243)
- Output inscription ID from `ord wallet inscribe` (#1208)
- Allow arbitrary wallet names (#1207)
- Use distinct inscription IDs (#1201)
- Remove ordinal addresses (#1197)
- Create taproot-only wallets (#1158)
- Check schema when opening index (#1127)
- Teach `ord wallet send` to send cardinal sats (#1137)
- Rename `ord wallet utxos` → `ord wallet outputs` (#1148)
- Swap arguments to ord wallet send (#1142)
- Rename --index-satoshis → --index-sats (#993)

### Fixed
- Fix preview for inscriptions with no body (#1287)
- Bail if reveal transaction is too large (#1272)
- Increase commit transaction output to pay for reveal transaction (#1242)
- Fix inscription thumbnail links (#1199)
- Use outpoint value table correctly and cache values in memory(#1172)
- Fix install script targets (#1120)

### Misc
- Use examples in core preview test (#1289)
- Use array for transaction builder change addresses (#1281)
- Fuzz test TransactionBuilder (#1283)
- Adopt Fish Eye logo (#1270)
- Split library and binary (#1273)
- Fix preview kill on drop (#1260)
- Add warning to readme (#1213)
- Run ignored tests in `ci` recipe (#1259)
- Add Bitcoin Core test job to CI (#1191)
- Add digital artifacts page to handbook (#1165)
- Use numbers in page titles (#1221)
- Set strict transport security header (#1216)
- Simplify BIP (#1226)
- Document required Bitcoin Core version for inscribing (#1225)
- Index lost sat ranges (#1227)
- Link to /block from /sat (#1228)
- Print index path in `ord info` (#1232)
- Add backlinks from /output and /transaction (#1235)
- Don't check lockfile on CI (#1209)
- Redirect HTTP to HTTPS (#1198)
- Test that inscriptions in additional envelopes and outputs are ignored (#1190)
- Use "sat" throughout codebase (#1184)
- Enable firewall on deployments (#1186)
- Request bech32m addresses in preview command (#1183)
- Use mainnet in tests (#1185)
- Move wallet tests into submodules (#1182)
- Link to /sat from /inscription (#1176)
- Match inscription preview and site background colors (#1175)
- Test that envelopes not starting with OP_FALSE are ignored (#1171)
- Update changelog (#1177)
- Remove mainnet wall restrictions (#1170)
- Ordinal addresses (#1163)
- Link outputs and inscriptions (#1162)
- Remove mainnet ord-dev index (#1164)
- Preview all inscriptions inside iframes (#1132)
- Remove inscriptions subcommand struct (#1151)
- Limit transaction count limit to u16::Max (#1152)
- Tweak homepage (#1124)
- Track fee-spent and lost inscriptions (#1125)
- Use InscriptionId in Reference (#1135)
- Avoid push_scriptint (#1136)
- Check Bitcoin Core version before inscribing (#1048)
- Display alpha in navbar on mainnet (#1122)
- Make PageHtml generic over PageContent type (#1123)
- Track inscriptions at offset and vout other than first (#1108)
- Unrecognized even fields are invalid (#1107)
- Add short flags (#1102)
- Document Debian dependencies (#1110)
- Add first testnet inscription height (#1109)
- Remove CORS headers (#1103)
- Don't wrap text inscriptions (#1100)
- Upgrade to redb 0.11.0 (#1099)
- Add quickstart script for macos (#1096)
- Remove text inscription anchor tag text decoration (#1084)
- Display inscription content on /inscriptions (#1077)
- Make inscription text white on inscription page (#1079)
- Move templates into root module (#1090)
- Show text inscriptions on homepage (#1058)
- Add white background to inscriptions (#1054)
- Show rare sat locations on /sat (#1029)
- Add first signet inscription height (#1016)
- Improve inscription style (#1025)
- Improve ord-dev recipes (#1022)
- Move inscription content above details (#1017)
- Style latest inscriptions (#1018)
- Print server URLs on startup (#1015)
- Add inscription page preview image (#1010)
- Show most recent inscriptions first on homepage and inscriptions page (#1011)
- Display graphical inscriptions on homepage (#1008)
- Add inscriptions page (#1009)
- Minimize transaction fetching (#1002)
- Rename `ord wallet satoshis` to `ord wallet sats` (#1004)
- Update introduction.md (#1000)
- Improve latest inscriptions style (#999)
- Show latest inscriptions on home page (#996)
- Add link to docs in readme (#995)
- Add inscription docs (#994)
- Fix softprops/actions-gh-release version (#992)
- Fuzz test transaction builder with multiple UTXOs (#1291)

[0.3.0](https://github.com/ordinals/ord/releases/tag/0.3.0) - 2022-12-16
---------------------------------------------------------------------

- Update CI dependencies (#986)
- Add /content endpoint (#976)
- Display content type and size /inscription (#975)
- Use "sat" in place of "ordinal" (#979)

[0.2.1](https://github.com/ordinals/ord/releases/tag/0.2.1) - 2022-12-14
---------------------------------------------------------------------

- Revise inscription guide after mainnet walkthrough (#968)

[0.2.0](https://github.com/ordinals/ord/releases/tag/0.2.0) - 2022-12-14
---------------------------------------------------------------------

- Add `ord wallet create` (#958)
- Add chain flags (#961)
- Make inscription parser more lenient (#956)
- Add `ord wallet transactions` (#951)
- Update dependencies (#955)
- Show inscription on reveal transaction page (#954)
- Mention that wallet may not need to be loaded (#953)
- Document install script (#952)
- Revise homepage (#950)
- Add content to guide page (#945)
- Mention incentive to run full node in FAQ (#948)
- Expand FAQ (#944)
- Change --index-ordinals to --index-satoshis (#942)
- Improve wording (#938)
- Add help text to subcommands (#934)
- Merge CI jobs into one workflow (#935)
- Add install script (#940)
- Build MacOS ARM Binaries (#941)
- Add inscription guide (#912)
- Allow inscribing without specifying a satpoint (#919)
- Add `ord wallet inscriptions` (#917)
- Add `ord wallet utxos` (#911)
- Add `ord wallet recieve` (#909)
- Fix signet block explorer link (#908)
- Opt wallet transactions into RBF (#901)
- Avoid `as` conversions (#903)
- Save commit transaction recovery key (#885)
- Refuse to send inscriptions by satpoint (#898)
- Limit inscription content to 1024 bytes on signet and testnet (#896)
- Extend bounty 3 (#897)
- Make inscription type more flexible (#892)
- Update dependencies (#894)
- Refuse to inscribe UTXOs with additional inscriptions (#880)
- Make inscriptions support backwards-compatible extension (#888)
- Refuse to send additional inscriptions (#881)
- Enable Windows tests on CI (#846)
- Refuse to inscribe sats that have already been inscribe (#878)
- Send by inscription ID (#877)
- Test commands which return errors when not tracking rare ordinals (#875)
- Don't store serialized inscriptions (#872)
- Do not select inscribed sats as cardinal utxos (#835)
- Make ord info work without ordinal index (#874)
- Improve subcommand names (#867)
- Calculate TXIDs in background thread (#866)
- Track inscription satpoints (#860)
- Add type aliases index for array types (#859)
- Index inscriptions when not indexing ordinals (#857)
- Use satpoints instead of ordinals in wallet commands (#849)
- Only request transactions if indexing ordinals (#851)
- Make analyzing index easier (#850)
- Add `ord list-ranges <OUTPOINT>` (#848)
- Conditionally disable ordinal index dependent server features (#845)
- Update redb (#832)
- Compress downloaded logs (#836)
- Only index ordinal ranges if `--index-ordinals` is passed (#837)
- Record commit block count and timestamp in index (#826)
- Add build-snapshots recipe (#831)
- Add minimum system requirements to readme (#829)
- Abort update if another has run concurrently (#830)
- Add benchmark-revision recipe (#827)
- Retry get_block_hash as well as get_block (#820)
- Update dependencies (#823)
- Add inscription page (#817)
- Add PNG inscriptions (#800)
- Disable inscriptions on mainnet (#814)
- Add benchmark recipe (#810)
- Display chain in header if not on mainnet (#809)
- Show difficulty target on block page (#750)
- Deduct fee before calculating reveal transaction signature (#780)
- Remove redundant wallet balance check (#764)
- Add `ord wallet inscribe` command (#658)
- Remove outdated runes and inscriptions (#760)
- Prevent progress bar from flickering when synced (#759)
- Fix graph command to work with new format (#755)
- Track ordinal ranges (#756)
- Use HTTP connection reusing `rust-jsonrpc` (#754)
- Extend bounty 3 by one difficulty adjustment period (#753)
- Replace binary search in epoch construction (#723)
- Search for ordinals in TSV using `ord wallet identify` (#729)
- Don't create acme cache dir (#727)
- Split up ci into test and lint workflows (#728)
- Enable CI for Windows (#603)
- Add bounty 3 (#725)
- Fetch blocks in background (#495)
- Don't call `apt-get update` in CI workflow (#719)
- Remove old recipes from justfile (#718)
- Update roadmap (#722)

[0.1.0](https://github.com/ordinals/ord/releases/tag/0.1.0) - 2022-10-25
---------------------------------------------------------------------

- Add index updater (#703)
- Speed up rarity check while indexing (#702)

[0.0.6](https://github.com/ordinals/ord/releases/tag/0.0.6) - 2022-10-25
---------------------------------------------------------------------

- Switch to ord-bitcoincore-rpc (#707)
- Start error messages with lowercase character (#693)
- Ensure addresses are valid for network (#698)
- Link videos from docs (#696)
- Restrict `ord wallet send` on mainnet (#687)
- Improve progress bar (#694)
- Note bounty 2 has been claimed (#700)
- Don't opt-in to RBF (#685)
- Don't unintentionally send rare ordinals (#683)
- Enforce transaction construction output address invariants (#682)
- Use worst-case fee estimates (#681)
- Add encoding to clock SVG (#678)
- Add helpers to make transaction builder tests more concise (#679)
- Don't use UTXOs with rare ordinals as cardinal inputs (#680)
- Improve not enough cardinal UTXOs error message (#675)
- Pad initial output to be above dust limit (#674)
- Start indexing progress bar at current height (#673)
- Add additional postage when necessary (#671)
- Check transaction fees in transaction builder (#669)
- Display progress bar when indexing (#668)
- Send ordinal first in recipient output (#666)
- Add doc-comment to transaction builder (#663)
- Change feerate to 1 sat/vbyte (#664)
- Strip excess postage from end of output (#662)
- Download logs to tempdir (#656)
- Improve transaction builder checks (#661)
- Use redb's two-phase write strategy in production (#660)
- Replace `Result<()>` with `Result` (#657)
- Add fee when sending (#655)
- Make table names more explicit (#654)
- Fix race condition in commit test (#651)
- Reform `ord wallet send` (#648)
- Use https://signet.ordinals.com as default signet publish URL (#649)
- Append network to data dir (#650)
- Only commit when necessary (#647)
- Make rarity text white (#644)
- Link to ordinal from rune (#643)
- Show inscriptions on /ordinal (#645)
- Document search (#646)
- Check that RPC server is on correct network (#642)
- Add /input page (#639)
- Expand search box to fill available space (#633)
- Add `ord rune publish` command (#637)
- Add links to docs (#635)
- Use docs for name of workflow and directory (#632)
- Remove multilingual book config key (#631)
- Add `ord wallet send` (#618)
- Streamline roadmap (#628)
- Improve styling (#626)
- Fix book publish directory (#625)
- Convert docs from Zola to mdBook (#623)
- Add nav bar (#614)
- Add status header to homepage (#620)
- Update roadmap (#617)
- Use reduced database durability during tests (#621)
- Add /rare.txt (#619)
- Embellish block page (#605)
- Refactor server error handling (#607)
- Profile tests (#608)
- Display ranges with an en dash (#606)
- Display more information homepage (#610)
- Remove prime trait (#612)
- Sort ordinal properties (#609)
- Add dark mode (#611)
- Add more help text to CLI (#613)
- Expand ordinal hunting guide (#600)
- Embellish transaction page (#602)
- Add `ord wallet list` command (#601)
- Ignore temporary directory (#594)
- Add ordinal hunting how-to docs page (#596)
- Fix bounty example links (#595)

[0.0.5](https://github.com/ordinals/ord/releases/tag/0.0.5) - 2022-10-02
---------------------------------------------------------------------

- Add bitcoin.conf (#592)
- Add uncommon ordinal bounty (#588)
- Show output size on output page (#590)
- Implement `wallet identify` (#586)
- Report integration test times (#587)
- Show message when output couldn't be listed because it was spent (#585)
- Add server integration test (#583)
- Use constants from rust-bitcoin (#564)
- Update dependencies (#582)
- Move bounties into subpages (#576)
- Convert last find integration test to unit test (#580)
- Make index::custom_index_size test faster (#579)
- Make info::basic test faster (#578)
- Convert list unit tests to inegration tests (#572)
- Add prime trait (#563)
- Rename workflow jobs (#575)
- Convert some find integration tests to unit tests (#571)
- Remove /clock.svg route (#573)
- Fix test bitcoin core rpc server compilation (#570)
- Move test Bitcoin Core RPC server into sub-crate (#569)
- Remove spent output test (#568)
- Remove find-by-slot tests (#567)
- Remove BDK wallet (#566)
- Show if a reorg has happened on /status (#518)
- Convert block and transaction integration tests to unit tests (#560)
- Fix release script (#562)

[0.0.4](https://github.com/ordinals/ord/releases/tag/0.0.4) - 2022-09-26
---------------------------------------------------------------------

- Add more links and labels to clocks (#552)
- Add script to deploy dev server on production machines (#550)
- Update redb to 0.7.0 (#559)
- Don't block server on index (#551)
- Allow searching for block hashes, txids, and outputs things (#549)
- Convert more integration tests to unit tests (#548)
- Make range integration tests faster (#547)
- Add roadmap (#546)
- Convert some integration tests to unit tests (#544)
- Sync index on `Index::open` (#545)
- Make some tests faster (#543)
- Add search-by-path endpoint at /search/QUERY (#521)
- Note why unit tests should use regtest network (#539)
- Use --chain regtest to speed up unit tests (#538)
- Add attributes to search box (#520)
- Fix off-by-some --height-limit bug (#526)
- Count total number of outputs traversed when building index (#525)
- Use boilerplate 0.2.0 (#531)
- Add favicon to docs.ordinals.com (#530)
- Move docs to GitHub Pages (#515)
- Bounty 1 claimed! (#529)
- Use fixed-size index keys and values. (#516)
- Update dependencies (#519)
- Log retry interval (#509)
- Retry with exponential backoff on RPC errors during indexing (#508)
- Include outpoint in missing outpoint message (#506)
- Link to clock from home page (#499)
- Pass benchmark dir name in justfile recipe (#498)
- Improve benchmark (#497)
- Commit every 1000 blocks instead of every block (#496)
- Improve benchmark script (#493)
- Add colors and tooltips to clock (#476)
- Block height to clock (#477)
- Add benchmark script (#488)
- Add flamegraph recipe (#486)
- Fix degree parsing (#485)
- Add search box to homepage (#479)
- Add shell.nix (#475)
- Fix indentation in test-deploy recipe (#474)
- Document how to turn on logging (#464)
- Add contribution advice to readme (#460)
- Increase default maximum index size for non-regtest chains (#448)
- Remove old NFT mint and verify commands (#418)
- Update readme (#399)
- Allow serving HTTP and HTTPS simultaneously (#359)
- Prevent ordinals that are being sent from being spent as fees (#369)
- Add error on None case for special_ordinals (#382)
- Guard against invalid percentiles (#380)
- Add percentile representation (#378)
- Make --acme-contact optional (#379)
- Improve names for a couple of properties (#377)
- [bin/graph] Skip previous syncs (#376)
- Add graph recipe (#375)
- Log ord by default (#374)
- Don't write to OUTPOINT_TO_TXID table (#373)
- Change just recipe to log main instance by default (#372)
- Add bounty 1 (#370)
- Don't hardcode cookie file in deploy script (#367)
- Remove comments from service files (#368)
- Add special ordinal protection (#357)
- Add defaults for --acme-cache and --acme-domain (#364)
- Read cookie file from --bitcoin-data-dir (#365)
- Pass network to deploy scripts (#366)
- Put .hushlogin in correct location (#363)
- Pass domain to deploy scripts (#361)
- Suppress login messages (#360)
- Disable password auth on deploy (#358)
- Improve deploy scripts (#342)
- Tick tock next block (#355)
- Add `ord wallet identify` (#304)
- Note bounty #0 has been claimed (#356)
- Remove unused CSS font-family (#354)
- Use rustl-acme acceptor (#289)
- Display hashes, ranges, and outputs in monospace (#353)
- Improve <ol> style (#352)
- Add temporary favicon (#351)
- Make deploys faster (#350)
- Color blocks on homepage by rarity (#349)
- Rarity-color ranges in outputs and link to first ordinal in ranges (#348)
- Remove slide deck (#346)
- Switch to one-at-a-time bounties (#347)
- Add better message for spent outputs (#345)
- Use <ol> for homepage (#343)
- Remove GitHub pages directory (#344)
- Rename / page from "root" to "home" (#341)
- Remove sleeps from server tests (#340)
- Add space around nav items (#338)
- Style links (#337)
- Add FAQ and bounty (#339)
- Add links to homepage (#335)
- Styling (#333)
- Remove fluff from BIP (#336)
- Remove old comment from bitcoind.service (#334)
- Add viewport meta tag (#332)
- Add rarity colors (#330)
- Don't let ordinals become telephone numbers (#331)
- Add next and prev links to /ordinal (#329)
- Fix broken link (#328)
- Add header to /range (#325)
- Fix off by one bug in index::blocks (#326)
- Add header to /output (#324)
- Limit blocks (#320)
- Add header to /tx (#322)
- Add header to /block/HASH (#321)
- Convert / to boilerplate template (#317)
- Return BlockHash from Index::all (#319)
- Don't warn about installing bitcoind in deploy/setup (#318)
- Improvements (#298)
- Update rust toolchain when deploying (#311)
- Fix forbidden word check (#313)
- Don't run integration tests on MacOS CI (#316)
- Disable redb checksums (#315)
- Pay a fixed fee when sending transactions (#314)
- Refactor duplicate blockchain code in purse (#312)
- Add `ord wallet send` (#305)
- Add wallet balance subcommand (#271)
- Add wallet utxos subcommand (#259)
- Use bitcoin core node for integration tests (#263)
- List transaction outputs (#292)
- Add `/output/:outpoint` endpoint (#293)
- Add /range/:start/:end endpoint (#291)
- Move /list endpoint to /api/list (#288)
- List block transactions at `/block/:hash` (#286)
- Display ordinals at `/ordinal/:ordinal` (#287)
- Wait for bitcoind and ord to become available (#285)
- List blocks on root page (#276)
- Remove user-facing list page (#275)
- Add network option (#274)
- Serve HTTPS with ACME certs (#256)
- Remove unused functionality (#270)
- Revise homepage (#268)
- Link to blog post (#267)
- Use hour, minute, second, and third terminology (#262)
- Allow passing ordinals in degree and decimal notation (#261)
- Update dependencies (#258)
- Make genesis sat mythic (#260)
- Add wallet (#233)
- Overhaul traits (#255)
- Clarify duplicate transaction rule in BIP (#254)
- Purge LMDB (#231)
- Add justfile with commands for moving ordinals around manually (#238)
- Add links to discord server (#237)
- Make `nft verify` take input as argument (#235)
- Add --version flag (#236)
- Bump version: 0.0.2 → 0.0.3 (#234)
- Change deploy target in recipe (#232)
- Use default port and set ambient capabilities in ord service (#230)
- Test deploy on vagrant (#229)
- Update slide deck (#227)
- Add link to video (#226)
- Separate deck pages (#225)
- Fix docs HTML (#224)
- Add side deck (#223)
- Change slot notation to AxBxCxD (#222)
- Improve NFT encoding (#221)
- Remove use of sha256d in signature algorithm (#219)
- Use standard formats (#218)
- Use CBOR for serialization/deserialization (#217)
- Add nix flake (#214)
- Build binaries for releases (#213)

[0.0.1](https://github.com/ordinals/ord/releases/tag/0.0.1) - 2022-06-05
---------------------------------------------------------------------

- Add commands to mint and verify NFTs (#211)
- Add legendary sat location hints (#208)
- Re-implement find (#206)
- Add explanation to bounty page (#205)
- Change bounty dir to bounties (#204)
- Add ordinal bounty page (#203)
- Add drawbacks section to BIP (#202)
- Remove log spam (#200)
- Don't reopen LMDB databases (#201)
- Add serve recipe (#199)
- Continuously index ranges (#198)
- Add about page to website (#197)
- Put script tag in <head> (#195)
- Add list form (#194)
- Run server command (#193)
- Remove find command and KEY_TO_SATPOINT table (#192)
- Make checkout script check out correct branch (#191)
- Add server subcommand (#185)
- Use anyhow to add context to error messages (#184)
- Automate deployment (#187)
- Add ordinals.com website source (#186)
- Add LMDB database backend (#177)
- Link to project board in readme (#176)
- Test null outputs and inputs (#169)
- Log transaction indexing (#168)
- Remove the acknowledgements section since it's still a draft (#164)
- Add index size to info subcommand (#162)
- Document duplicate txid behavior (#161)
- Update redb 0.0.5 (#160)
- Document terminology and notation (#158)
- Describe dust output avoidance workaround (#156)
- Improve readme (#154)
- Improve find height check (#150)
- Use index for find queries (#149)
- Note that LN cannot be used to transfer individual ordinals (#147)
- Print block transaction count (#146)
- Use clap instead of structopt (#145)
- Incremental indexing (#141)
- Use human readable byte values for info (#144)
- Add info subcommand (#138)
- Accept human readable --index-size values (#142)
- Use redb::TableDefinition (#143)
- Work with live Bitcoin Core RPC API (#140)
- Use JSON RPC API  instead of blocksdir(#139)
- Test mining and spending transactions in the same block (#136)
- Don't recreate db every run (#131)
- Fix off by one error in log message (#135)
- Improve index performance (#134)
- Reference independent invention (#133)
- Decode block header only in Index::index_blockfiles (#132)
- Add index benchmark (#111)
- Mention physical transfer of ordinals (#130)
- Reorder BIP sections (#129)
- Add applications section to BIP (#127)
- Add initial draft of BIP (#117)
- Test that index handles out-of-order blockfiles (#124)
- Test fee assignment (#122)
- Test underpaying subsidy (#121)
- Allow setting index size (#120)
- Use redb 0.0.4 (#114)
- Add duplicate transaction range test (#113)
- Split up Index::index_blockfiles (#96)
- Allow invalid ordinals (#95)
- Don't hardcode genesis block (#91)
- Rename index_blockfile to index_blockfiles (#90)
- Pin redb to GitHub revision to avoid panic (#89)
- Log progress while indexing (#88)
- Index all files in blocksdir (#87)
- Fix crash when indexing a block with no transactions (#86)
- Refactor test API (#82)
- More integration test cleanup (#70)
- Refactor test block creation (#68)
- Improve index (#60)
- Add `index.redb` to gitignore (#58)
- Make find command print satpoints instead of outpoints (#57)
- Improve transfer algorithm pseudocode (#53)
- Add epoch trait (#51)
- Use strong types (#48)
- Add Index struct (#47)
- Use ordinal number terminology (#46)
- Number satoshis in ascending order (#45)
- Use default location if `--blocksdir` is not provided (#42)
- Update dependencies (#40)
- Create illusive and cursed traits (#36)
- Add character trait (#35)
- Add open questions to readme (#34)
- Use descending numbering scheme (#33)
- Handle out-of-bound values (#30)
- Add yet more traits (#29)
- Add shiny trait (#28)
- Add command to find satoshi with a given name (#27)
- Add more traits (#25)
- Add traits (#24)
- Add readme and refactor code (#22)
- Rename to sat-tracker (#21)
- Start new sat-based implementation (#20)
- Add justfile and catalog recipe (#12)
- Organize code (#10)
- Add supply command (#9)
- Track atom locations (#2)
- Add Rust binary and CI workflow (#1)
- Add readme
