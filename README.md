# CLI Commands Repository

Welcome to the **CLI Commands** repository! This project was developed for educational purposes to learn the Rust programming language. The repository contains a collection of simple command-line utilities implemented in Rust, mimicking some of the functionalities of popular UNIX commands. These utilities include `echo`, `cat`, `ls`, and `wc`, each with various options for customization.

## Table of Contents

- [Overview](#overview)
- [Commands](#commands)
  - [echo](#echo)
  - [cat](#cat)
  - [ls](#ls)
  - [wc](#wc)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

This repository contains a set of CLI utilities developed in Rust to simulate the behavior of commonly used UNIX commands. These utilities were created as part of a personal educational project to gain hands-on experience with Rust programming and command-line application development.

## Commands

### echo

The `echo` command outputs the given arguments to the terminal. It supports the following options:

- `-e`: Enables interpretation of escape sequences (e.g., `\n` for newline, `\t` for tab).
- `-E`: Disables interpretation of escape sequences (treats them as normal text).

### cat

The `cat` command concatenates and displays the contents of one or more files. It supports the following options:

- `-n`: Number all output lines.
- `-e`: Display non-printable characters as escape sequences.
- `-b`: Number non-empty output lines.

### ls

The `ls` command lists the files and directories in the specified directory. It supports the following options:

- `-a`: Include hidden files and directories in the listing.
- `-l`: Use a long listing format, displaying additional information such as permissions, owner, size, and modification date.

### wc

The `wc` command counts the number of lines, words, characters, and bytes in a given file. It supports the following options:

- `-l`: Count only the lines.
- `-w`: Count only the words.
- `-c`: Count only the characters.
- `-m`: Count only the bytes.

## Installation

To use these CLI utilities, follow these steps:

1. Make sure you have Rust installed on your system. If not, you can download it from [Rust's official website](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/ambadidi/cli_commands.git
   ```

3. Navigate to the cloned directory:

   ```bash
   cd cli_commands
   ```

4. Build the utilities using Cargo (Rust's package manager and build tool):

   ```bash
   cargo build --release
   ```

## Usage

After building the utilities, you can run them from the command line. Here are some examples:

```bash
# Run the echo command
./target/release/echo Hello, world!

# Run the cat command with options
./target/release/cat -n file.txt

# Run the ls command with options
./target/release/ls -a

# Run the wc command with options
./target/release/wc -l file.txt
```

Replace `file.txt` with the actual file path you want to use.



---

Thank you for your interest in the **CLI Commands** repository. 
