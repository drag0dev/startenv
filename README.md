# Startenv
startenv is a small script that starts a process with env vars that are in a file
## Why
Instead of using a dev dependency to plug env vars at runtime or write all env vars in terminal when starting the process, which can be quite messy, just preface whatever process you are running with ```startenv``` and it will automatically pull all env vars from a file.
## Install
```
cargo install --path ./
````  
## Usage
in case there are poorly formatted vars user is prompted whether to continue  
you can use startenv two different ways:  
1. without any flags in which case it will try to open .env file in the current directory
```
startenv my-app [...]
```
2.  with flag -f (--file) where you specify path to the env var file (it does not have to be named .env)
```
startenv -f env/example.env my-app [...]
startenv --file env/example.env my-app [...]
```

## Parsing vars
Each var should be in a new line, with format NAME=value. Name should only consists of letters, numbers and _. Both line and inline comments are supported.
