# SugarCRM MLP Builder

This is a CLI tool built with rust that generates a module loadable package from a local instance and a text file containing the list of files to include in the copy array

## Usage
```terminal
USAGE:
    sugarcrm_mlp_builder [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f <FILE>            File to use to build the package
    -i <INSTANCE>        Name of SugarCRM folder
    -n <NAME>            Name of package to generate
    -v <VERSION>         Version of the mlp being built
```

```terminal
./sugarcrm_mlp_builder -n "PackageName" -f file_list.txt -i sugar_directory 
```
## Environment Variables
You will need to change the environment variables in the .env file to match the settings you need. Here's a list of the variables currently available

```toml
# Package Author
AUTHOR="Hasan Ghazzawi"

# Webroot Folder
WEB_ROOT="/home/hasan/local/www/"

# Default package version
DEFAULT_VERSION="1.0"
```
## Zip file location
The zip file will be placed in a releases folder that gets created in the same directory as the shell script being run.

## Compiling CLI Tool
After updating the .env file, run the following command to generate the shell script
```terminal
cargo build --release
```