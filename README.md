# Rust mini project week 8
This Rust project demonstrates a simple CLI application that generates a random number between 1 and 5 and prints logs to the terminal. The logs have three different levels: info, warn, and trace. The project utilizes the Rust logging ecosystem, specifically the log crate for logging and env_logger for output.

## Prerequisites
To run this project, you will need the following:

1. Rust programming language (version 1.57 or newer)
2. Git for cloning the repository
## Install
1. Clone the repo:
```git clone https://github.com/nogibjj/IDS721_miniproj8```
```cd rustLog```

2. Build the project:
```cargo build --release```

## Usage
Run the CLI application by providing the desired log level (info, warn, or trace) as an argument:
```cargo run -- --level <Options>```
The options could be ```info```, ```trace```, or ```warn```.
## Screenshot
![alt text](/resource/img.png)

## Contributing
I appreciate all kinds of contributions to this project! Please feel free to open an issue to discuss your ideas or submit a pull request with your changes.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
