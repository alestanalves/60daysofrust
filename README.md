# 60daysofrust
my rust learning 

## Instalando o Rust (Ubuntu WSL)

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install cargo

## Criando meu projeto com Rust

cargo new project1

## Construindo os binarios

cd project1

cargo build 

# Output
#   Compiling data-with-rust v0.1.0 (/Users/karim/tests/project1)
#    Finished dev [unoptimized + debuginfo] target(s) in 0.51s

./target/debug/data-with-rust

# Output
# Hello, world! 

## Executando meu arquivo Rust

cargo run 