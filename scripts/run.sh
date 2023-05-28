## Usage: sh scrpts/run.sh (release | debug) file/path/to/json

arg="$1"
_file="$2"
_show="$3"

if [[ "$_file" == "" || -z "$_file" ]]; then
    unameOut="$(uname -s)"
    ## The escape sequences are respectively: `bold` and `reset` codes for the string `$unameOut`
    echo -e "\nPlatform Detected:\t\033[1m$unameOut\033[0m\n"
    case "${unameOut}" in
        Linux*)     BASEDIR="$PWD";;
        Darwin*)    BASEDIR="$PWD";;
        CYGWIN*)    BASEDIR="$(cmd //c cd)";;
        MINGW*)     BASEDIR="$(cmd //c cd)";;
        MSYS_NT*)   BASEDIR="$(cmd //c cd)";;
        *)          machine="UNKNOWN:$unameOut"
    esac
fi

if [[ "$arg" == "release" ]]; then
    if [[ "$_file" == "" || -z "$_file" ]]; then
        ## The excape sequences are respectively: `red`, `bold` and `reset` codes for the string `NOTICE:`
        echo -e "\033[31m\033[1mNOTICE:\033[0m    File not specified; taking default example.\n"
        _file="$BASEDIR/data/galaxies.json"
    fi
    ./target/release/hubble-constant.exe "$_file" "$_show"
elif [[ "$arg" == "debug" ]]; then
    if [[ "$_file" == "" || -z "$_file" ]]; then
        ## The excape sequences are respectively: `red`, `bold` and `reset` codes for the string `NOTICE:`
        echo -e "\033[31m\033[1mNOTICE:\033[0m    File not specified; taking default example.\n"
        _file="$BASEDIR/data/galaxies.json"
    fi
    ./target/debug/hubble-constant.exe "$_file" "$_show"
else
    if [[ "$arg" == "" ]]; then
        arg="NULL"
    elif [ -z "$arg" ]; then
        arg="NULL"
    fi
    echo "Unknown build type given:    $arg."
fi