cd "$(dirname "$PWD")"

spinner() {
  local pid=$1
  local delay=.1
  local chars='-\|/'
  local i=0

  while kill -0 $pid 2>/dev/null
  do
    i=$(( (i+1) %4 ))
    printf "\r${chars:$i:1}"
    sleep $delay
  done

  printf "\r"
}

runBuild() {
  wasm-pack build --release --target web --out-dir $1 $2 &> $output &

  local pid=$!

  spinner $pid

  wait $pid

  if (($?)); then
    echo "$(<$output)"
  fi

  rm -rf "$1"/.gitignore "$1"/README.md "$1"/*.json
}

output=$(mktemp)
rm -rf dist/**/*

runBuild "dist"

rm -rf $output

echo "Done"
