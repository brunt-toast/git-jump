# brunt-toast/git-jump 

Quickly jump to any git repository on the file system. 

## Requirements 

* [fzf](https://github.com/junegunn/fzf)
* [plocate](https://www.kali.org/tools/plocate/) 

## Installation 

Install via cargo: 
```bash
cargo install --git https://github.com/brunt-toast/git-jump.git
```
## Setup 
For effective use, plocate must have indexed the file system. Use the `updatedb` command to index the file system. New repositories will not appear until the database is updated. Consider setting up a cron job to update the database periodically. 

## Usage 

To ensure the git alias is working correctly, ensure the installation is in your PATH. Create an alias or key binding for `cd "$(git jump)"` in your shell and run it. Selecting a repository will then navigate into its directory. 

## Configuration 

Coming... eventually! 
