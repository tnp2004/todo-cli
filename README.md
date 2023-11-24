# Todo CLI
Command line application for you to have todo list in your terminal

## Checklist
- [x] show
- [x] add
- [x] status
- [x] update
- [x] remove
- [x] import from csv
- [x] export to csv

## Usage example
### Get help
```
cargo run -- --help
```
### Change file
```
cargo run -- -p "path_to_file.csv" [command]
```
### Add todo
```
cargo run -- add "drink milk"
```

#### You can change default path in Config.toml