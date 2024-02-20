# Vote Bank Program

This is a simple Rust program built using the `anchor_lang` crate to manage a vote bank. The program allows users to initialize a vote bank and submit votes for "yes" or "no."

## Table of Contents

- [Overview](#overview)
- [Usage](#usage)
- [Program Structure](#program-structure)
- [Accounts](#accounts)
- [Building and Deploying](#building-and-deploying)
- [License](#license)

## Overview

The program provides two main functionalities:

1. **Initialize Vote Bank (`init_vote_bank`):** Opens the vote bank for public voting.

2. **Give Vote (`gib_vote`):** Allows users to submit votes for "yes" or "no."

## Usage

To interact with the program, you can call the provided methods using the Anchor CLI or integrate it into your application.

### Initialize Vote Bank

```rust
anchor-cli run --program-id <program-id> init_vote_bank

anchor-cli run --program-id <program-id> gib_vote --vote_type <0 or 1>
Program Structure
The program is organized into a hello_world module, containing the initialization and vote functions. The main data structure is the VoteBank account.

Accounts
InitVote
vote_account: The account representing the initialized vote bank.
signer: The account signing the transaction.
system_program: The system program account.
GibVote
vote_account: The account representing the vote bank for submitting votes.
signer: The account signing the transaction.
VoteBank
is_open_to_vote: A boolean indicating whether the vote bank is open for voting.
yes: The count of "yes" votes.
no: The count of "no" votes.
Building and Deploying
Ensure you have the Anchor CLI installed:

cargo install --git https://github.com/project-serum/anchor --branch=main

anchor build
anchor deploy
