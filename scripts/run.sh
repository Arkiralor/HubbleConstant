## Usage: sh scrpts/run.sh (release | debug) file/path/to/json

arg="$1"
_file="$2"
BASEDIR="$PWD"

if [[ "$arg" == "release" ]]; then
    if [[ "$_file" == "" || -z "$_file" ]]; then
        ## The excapr sequence are respectively: `red`, `bold` and `reset` codes for the string `NOTICE:`
        echo -e "\033[31m\033[1mNOTICE:\033[0m    File not specified; taking default example."
        _file="$BASEDIR/data/galaxies.json"
    fi
    ./target/release/hubble-constant.exe "$_file"
elif [[ "$arg" == "debug" ]]; then
    if [[ "$_file" == "" || -z "$_file" ]]; then
        ## The excapr sequence are respectively: `red`, `bold` and `reset` codes for the string `NOTICE:`
        echo -e "\033[31m\033[1mNOTICE:\033[0m    File not specified; taking default example."
        _file="$BASEDIR/data/galaxies.json"
    fi
    ./target/debug/hubble-constant.exe "$_file"
else
    if [[ "$arg" == "" ]]; then
        arg="NULL"
    elif [ -z "$arg" ]; then
        arg="NULL"
    fi
    echo "Unknown build type given:    $arg."
fi