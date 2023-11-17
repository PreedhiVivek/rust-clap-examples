# rust-clap-examples
Contains sample code snippets to help understand the usage of Rust's clap crate

Pre-requisites:
- Install Rust and Cargo(package manager)
Please note: Cargo comes pre-installed with the official Rust installer.
- Setup an environment variable for clap_example2
`export APP_PASSWORD=<StringOfYourChoice>`

Run the clap examples:
1. Move to the root directory that contains the binary i.e `code-snippets`
2. Execute command : `cargo run --bin <binaryFilename> <positionalArguments> -- <optionName> <optionValue>`
sample run commands:
for options with arguments
`cargo run --bin clap_example1 -- -n 5`
for positional arguments
`cargo run --bin clap_example5 2`
for positional arguments and boolean options
`cargo run --bin clap_example5 5 --square`