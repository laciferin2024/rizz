set shell := ["sh", "-c"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set allow-duplicate-recipe
#set positional-arguments
#set dotenv-load
set dotenv-filename := ".env"
set export

default: install


install:
  poetry install --root

solana-config:
  solana config set --url https://api.devnet.solana.com
  solana config set --keypair ./scripts/my-keypair.json

airdrop:
  solana airdrop 5

fund: airdrop

setup:
  rustup component add rustfmt 
  cargo install seahorse-lang
  cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
  seahorse -- version

seahorse *ARGS:
  seahorse {{ARGS}}

sbuild:  
  seahorse build 
  
build:
  anchor build

b: build


test:
  anchor test --skip-deploy