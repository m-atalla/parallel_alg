# PI estimation Program

#### Usage
```bash
cargo run -p pi -- [OPTIONS] 

OPTIONS:
    -s               Run in sequential model                                   
    -p               Run in parallel mode
```
#### Examples:
- Runs the program in sequential mode
    ```bash
        $ cargo run -p pi -- -s
    ```
- Runs the program in parallel mode
    ```bash
        $ cargo run -p pi -- -p
    ```
