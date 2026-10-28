# File Finder (dir-scan)

**File Finder is a fast and lightweight console utility written in Rust that helps you locate any file in your system.**

It takes a file name from the user, scans the directory, and searches through all folders and subfolders. It uses the `walkdir` library for fast folder traversal and safely skips any system folders that require administrator/root privileges without crashing.

---

## Features
* **Deep Search:** Recursively scans all subdirectories from a given starting point.
* **Crash-Proof:** Automatically bypasses `Permission Denied` errors on protected system folders.
* **Fast Performance:** Built on Rust's high-performance file management capabilities.

---

## Prerequisites

To compile and run this project from source, you need to have **Rust** and **Cargo** installed.
* Check if you have them installed by running:
  ```bash
  cargo --version
  ```

---

## Installation & Running

1. **Clone the repository:**
   ```bash
   git clone https://github.com/o0travaa/dir-scan.git
   cd dir-scan
   ```

2. **Run the application:**
   You can compile and run the project instantly using Cargo:
   ```bash
   cargo run
   ```

---

## How to Use

1. Start the program using `cargo run`.
2. Enter the exact name of the file you want to find when prompted (for example: `.bashrc` or `Cargo.toml`).
3. The program will scan the predefined directory and print the full path of the file as soon as it finds it.

---

## Project Dependencies
The project uses the following external crate:
* `walkdir` (v2.5.0) — used for efficient and safe recursive directory walking.
