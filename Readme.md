# Rust Filesystem Simulation

This Rust program simulates a basic filesystem environment where users can navigate directories, create, read, write, and manipulate files and directories.

Written to fulfill the Final Project for Operating System Course, Universitas Pendidikan Indonesia
under the guidance of Dr. Rasim Hermanto, M.T.
## Features

- **Navigation**: Users can navigate through directories using commands such as `cd` (change directory) and `ls` (list directory contents).
- **File Operations**: Users can create, read, write, append to, and remove files using commands like `mkfile`, `read`, `write`, `append`, and `rmfile`.
- **Directory Operations**: Users can create and remove directories with `mkdir` and `rmdir` commands.
- **Mounting**: Users can mount other filesystems into the current filesystem to access their contents.
- **Other Operations**: Additional operations such as copying files (`cpfile`), renaming files (`renmfile`), copying directories (`cpdir`), and renaming directories (`renmdir`) are also supported.

## Usage

1. **Compile**: Compile the program using the Rust compiler.
   ```
   $ rustc main.rs
   ```

2. **Run**: Run the compiled program.
   ```
   $ ./main
   ```

3. **Commands**: Use the following commands to interact with the filesystem:
   - `cd <directory>`: Change current directory.
   - `ls`: List contents of the current directory.
   - `mkfile <filename>`: Create a new file.
   - `read <filename>`: Read the content of a file.
   - `write <filename> <content>`: Write content to a file.
   - `append <filename> <content>`: Append content to a file.
   - `rmfile <filename>`: Remove a file.
   - `mkdir <dirname>`: Create a new directory.
   - `rmdir <dirname>`: Remove a directory.
   - `cpfile <file> <newname>`: Copy a file.
   - `renmfile <file> <newname>`: Rename a file.
   - `cpdir <directory> <newname>`: Copy a directory.
   - `renmdir <directory> <newname>`: Rename a directory.
   - `mount <mountpoint>`: Mount a filesystem into the current filesystem.

4. **Exit**: Use the `exit` command to exit the program.

## Authors

- [Jason Rafif P.S.] (2204524)
- [Defrizal Yahdiyan R.] (2206131)
