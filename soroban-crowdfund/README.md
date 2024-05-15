<!-- # Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └-─ Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included. -->

## Overview

This Rust code represents a smart contract for managing crowdfunded environmental projects on a blockchain platform. It allows users to create projects, donate funds to these projects, create milestones within projects, and mark milestones as completed.

## Features

1. **Project Creation**: Users can create new environmental projects by providing a description, a fundraising goal amount, and initial milestones.

2. **Donation of Funds**: Once a project is created, users can donate funds to it. The donated funds contribute to the project's total funds raised.

3. **Milestone Management**: Projects can be broken down into milestones, each with its description and funding target. Users can create milestones and mark them as completed once the goals are achieved.

## Usage

To use this smart contract:

1. **Deployment**: Deploy this smart contract on a compatible blockchain platform.

2. **Project Creation**: Call the `create_project` function to create a new environmental project, providing the required parameters such as the project description, fundraising goal, and initial milestones.

3. **Donation**: Use the `donate_funds` function to donate funds to a specific project. Provide the sender's address, the token address, the amount to donate, and the project ID.

4. **Milestone Management**: Call `create_milestone` to create milestones within a project, providing the project ID, milestone description, and funding target. Use `complete_milestone` to mark a milestone as completed, providing the project and milestone IDs.

## Requirements

- Compatible blockchain platform
- Rust toolchain
- `soroban_sdk` library

## License

This project is licensed under the [MIT License](LICENSE).
