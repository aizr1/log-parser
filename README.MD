# Log Parser for `fitbit-cli` logs

Use regex patterns and classification of certain `fitbit-cli` log lines, then export select data as clean csv.

# Usage

If you have the rust toolchain installed on your system (with rustup)

    cargo run --release -- -n SomeNameOfDataRow -l Test.log

If you want to use the binary directly...

    log-parser -n SomeNameOfDataRow -l path/to/logfile.log
