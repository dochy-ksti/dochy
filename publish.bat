cd dochy_json5
cargo publish
timeout /t 10 > nul
cd ../dochy_compaction
cargo publish
timeout /t 10 > nul
cd ../dochy_core
cargo publish
timeout /t 10 > nul
cd ../dochy_archiver
cargo publish
timeout /t 10 > nul
cd ../dochy_diff
cargo publish
timeout /t 10 > nul
cd ../dochy_fs
cargo publish
timeout /t 10 > nul
cd ../dochy_intf
cargo publish
timeout /t 10 > nul
cd ../
cargo publish
timeout /t 10 > nul