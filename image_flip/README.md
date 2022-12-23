# Image to gray scale program
#### Usage
```bash
cargo run -p image_flip -- [OPTIONS] 

OPTIONS:
    -s               Run in sequential model
    -p               Run in parallel mode

```

#### Examples:
- Runs the program in sequential mode
    ```bash
        $ cargo run -p image_flip -- -s
    ```
- Runs the program in parallel mode
    ```bash
        $ cargo run -p image_flip -- -p
    ```
