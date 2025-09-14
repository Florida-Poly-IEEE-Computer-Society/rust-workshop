# Rust Programming Language Workshop

Welcome to the **Rust Programming Language Workshop**! This 3-day course will teach you the fundamentals of Rust, including scalar variables, functions, structs, enumerations, traits, and asynchronous programming. By the end, you'll build a fully functional web server using the **Rocket framework**.

This repository contains all the materials you need, including starter code, exercises, and the final project template. Follow the instructions below to set up your environment, use the repository, and understand the workshop structure.

---

## Helpful Tips When going through this Workshop

- Search For an Application (Mac: Command-Space bar | Windows: Windows Key)
- Terminal = Command Line = Powershell - All text interfaces.

## Installation

To participate in the workshop, you need to install Rust, a code editor, and the Rocket framework dependencies. Below are instructions for Windows, Linux, and macOS.

### Windows

1. **Install Rust**:
   - Download and run the Rust installer from [rust-installer](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
   - Follow the prompts to install the stable toolchain (default option).
   - Verify installation by running in powershell:

     ```bash
     rustc --version
     cargo --version
     ```

2. **Install Build Tools**:
   - Install the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). Select the "Desktop development with C++" workload.
3. **Install a Code Editor**:
   - Recommended: [Visual Studio Code](https://code.visualstudio.com/) with the **Rust Analyzer** extension.
   - Install VS Code, then add Rust Analyzer via the Extensions Marketplace.
4. **Install Rocket Dependencies**:
   - Rocket requires a nightly Rust toolchain. Set it up:

     ```bash
     rustup default stable
     ```

   - Install additional dependencies (e.g., OpenSSL) if needed for async features. See [Rocket's guide](https://rocket.rs/guide/master/getting-started/).

### Linux and MacOS

1. **Install Rust**:
   - Open a terminal and run:

     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - Follow the prompts to install the stable toolchain.
   - Add Rust to your PATH by adding this to `~/.bashrc` or `~/.zshrc`:

     ```bash
     export PATH="$HOME/.cargo/bin:$PATH"
     ```

   - Verify installation:

     ```bash
     rustc --version
     cargo --version
     ```

### Getting Started

- Test your setup by creating a new Rust project:

  ```bash
  cargo new my_project
  cd my_project
  cargo run
  ```

- You should expect to see "Hello World on your Console"

---

## How to Use the Repository

This repository contains all workshop materials, organized as follows:

### Steps to Use

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/<your-org>/rust-workshop.git
   cd rust-workshop
   ```

3. **Work on Projects**:
   - The `projects/` folder contains starter code for mini-projects and the final web server.
   - Navigate to a project folder and follow the `README.md` inside:

     ```bash
     cd projects/library-api
     cargo run
     ```

4. **Use Slides**:
   - Find PDF slide decks in the `slides/` folder for reference during lectures.
5. **Check Solutions**:
   - Solutions will be shared post-workshop in the `solutions/` folder. Avoid peeking during the workshop to maximize learning!

### Tips

- Use `cargo check` to verify syntax without compiling.
- Use `cargo fmt` to format code and `cargo clippy` for linting.
- If you modify dependencies, run `cargo build` to update.

---

## Workshop Structure

This workshop is going to be taught over 3 days

### Day 1: Rust Fundamentals and Calculator App

- **Topics**: Scalar variables, ownership, functions, structs, error handling (`Option`, `Result`).
- **Activities**:
  - Lectures with live coding.
  - Exercises on variables, functions, structs, enumerations, and implementations.
  - Mini-project: Build a command-line calculator app
- **Goals**: Understand Rust’s syntax, ownership model, and struct-based programming.

### Day 2: Async Rust with Tokio Runtime and Websockets

- **Topics**: Enumerations, traits, generics, asynchronous programming with `async`/`await` and `tokio`.
- **Activities**:
- **Goals**: Master Rust’s type system and starting with async programming.

### Day 3: Embedded Rust Programming

- TODO information

---

## Additional Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rocket Framework Documentation](https://rocket.rs/v0.5/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Async Runtime](https://tokio.rs/)

## How to Navigate Workshop Repository (The Folder you Installed)

You will use git to navigate through this code (BTW, you can now throw git as a key word on your resume :) )

### Helpful Git Commands for You

1. `git branch` view branches in repository.
1. `git status` view changes that you have added.
1. `git log --oneline` view commits in working branch.
1. `git checkout <branch-name` change working branch

### Brief guide to Git

What is git?

- Git is a version control system that tracks changes to files over time.

What is a repository?

- Stores the files that are maintained with versions.

What is a commit?

- a snapshot of changes made to files in a repository.
- Saved locally and each commit has a unique ID

What is a branch?

- A line of development.
- There is a main branch and divergent branches.
- A branch contains a list of commits
- You work on one branch at a time (working branch)
