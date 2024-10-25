# brunt-toast/git-jump 

Quickly jump to any version-controlled repository on the file system. 

## Requirements 

* [fzf](https://github.com/junegunn/fzf)
* [plocate](https://www.kali.org/tools/plocate/) 

## Installation 

Install via cargo: 
```bash
cargo install git-jump
```
## Setup 
For effective use, plocate must have indexed the file system. Use the `updatedb` command to index the file system. New repositories will not appear until the database is updated. Consider setting up a cron job to update the database periodically. 

## Usage 

To ensure the git alias is working correctly, ensure the installation is in your PATH. Create an alias or key binding for `cd "$(git jump)"` in your shell and run it. Selecting a repository will then navigate into its directory. 

## Configuration 
git-jump will search for configuration settings in `~/.config/git-jump.json` (or `%USERPROFILE%\.config\git-jump.json` if on Windows). If you wish to configure git-jump, add the following contents: 
```json
{
    "blacklist": [],
    "whitelist": []
}
```
> [!IMPORTANT]
> Even if you do not intend to specify values for a configuration option, it must be included in the file. 

Any repo whose starts with a string in the blacklist will be excluded from results. If the whitelist is populated, the program will only show repos whose paths begin with at least one member of the whitelist. Note that the values of blacklist and whitelist do not support globbing. They are used in a simple case-insensitive string comparison. 

