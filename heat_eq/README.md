# Heat equation
Source code is located in `./heat_eq/src/main.rs`

#### Usage
```bash
cargo run -p heat_eq -- [OPTIONS] 

OPTIONS:
    -s | -seq                              Run in sequential model          (default mode)                               
    -p | -par  | --parallel                Run in parallel mode
    -i | -iter | --iterations              Number of iterations to run      (default = 1000)
```
#### Examples:
- Runs the program in sequential mode for 5000 iterations
    ```bash
        $ cargo run -p heat_eq -- -s -i 5000
    ```
- Runs the program in parallel mode for 5000 iterations
    ```bash
        $ cargo run -p heat_eq -- -p -i 5000
    ```
