
endpath=$(basename "$(pwd)")

if [[ $endpath = "blackchain" ]]
then
    cd blackbody
fi

if [[ $1 = "-b" ]] || [[ $1 = "--build" ]] # build
then

    cargo build "${@:2}"

else

    if [[ $1 = "-r" ]] || [[ $1 = "--release" ]]
    then
        cargo run --release -- "${@:2}"
    else
        cargo run -- "$@"
    fi

fi