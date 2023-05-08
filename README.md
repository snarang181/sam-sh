# sam_sh

## Description </br>

**sam_sh** is a program written in the Rust programming language. It is a basic shell wrapper program that can execute built in shell commands and also support piping of multiple commands. It also supports the following built in commands: </br>
1. `ls` - List files and directories in the current working directory. </br>
2. `clear` - Clear the terminal screen. </br>
3. `grep` - Search for a pattern in a file. </br>
4. `touch` - Create a new file. </br>
5. `rm` - Remove a file. </br>
6. `cat` - Display the contents of a file. </br>
7. `bash` - Execute a bash script. </br>

## Design </br>

The program loops infinitely and waits for user input. Then, the parsing of the user input is done and the next steps depend on the type of command. If the command is exit or quit, the loop is broken and the program ends. In other cases, we check if the user input has a pipe ( | ) in their command. If no pipe exists, the bash built in command is executed by Rust's `Command` module. If a pipe exists, the user input is split by the piped commands and an iterator is created. Then, the first command is executed and the output is piped to the next command. This process is repeated until all the commands are executed. </br>


## Example Usage </br>

1. **Execute a bash script** </br>

sam_sh> vi test_bash.sh

**A Vim editor should open up and enter the following in the file:** </br>

```bash
#!/bin/bash
echo "Hello World"
```

sam_sh> bash test_bash.sh

**See the following output:** </br>
```bash
Hello World
```

2. **List files and directories in the current working directory** </br>
sam_sh> ls

**See the following output:** </br>
```bash
Cargo.lock      Cargo.toml      README.md       src             target  
```

3. **Use piping to execute multiple commands** </br>

a. sam_sh> ls -laF | grep Cargo

**See the following output:** </br>
```bash
-rw-r--r--   1 samarthnarang  staff   152 Apr 13 18:27 Cargo.lock
-rw-r--r--   1 samarthnarang  staff   177 Apr 13 18:23 Cargo.toml
```

b. sam_sh> ls -laF | grep Cargo | tail -n 1

**See the following output:** </br>
```bash
-rw-r--r--   1 samarthnarang  staff   177 Apr 13 18:23 Cargo.toml
```





