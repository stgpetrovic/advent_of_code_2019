 #!/bin/bash

# ./solve.sh -a=1
solve1() {
  awk '{sum+=int($1/3)-2} END {print sum}' input
}

# ./solve.sh -b=1
solve2() {
  awk '{do {$1=int($1/3)-2; if ($1 > 0) {sum+=$1}} while ($1>0) } END {print sum}' input
}

while getopts "a:b:" p; do
  case "$p" in
    a)  solve1;;
    b)  solve2;;
    [?])  print >&2 "Usage: $0 [-p=a|b]"
      exit 1;;
  esac
done
