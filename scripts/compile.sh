arg="$1"
_file="$2"
_show="$3"

sh scripts/generate_logs.sh

if [[ "$arg" == "" || -z "$arg" ]]; then
    echo -e "ARG: $arg" >> logs/HubbleConstant.log
    arg="release"
fi

if [[ "$_file" == "" || -z "$_file" ]]; then
    echo -e "FILE USED: $_file" >> logs/HubbleConstant.log 2>&1
    _file="data/galaxies.json"
fi


sh scripts/build.sh "$arg" >> logs/HubbleConstant.log
sh scripts/run.sh release "$_file" "$_show" >> logs/output.txt 2>&1
echo -e "\n------------------------------------------------------\n" >> logs/output.txt 2>&1
