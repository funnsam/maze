set -e

function pre_exit {
	clear
	echo -e "\x1b[?25h"
}

trap pre_exit EXIT
echo -e "\x1b[?25l"
cd generator
cargo build --profile fast -j8
cd ../solver
cargo build --profile fast -j8 2>/dev/null
cd ..

for (( i=1 ; i<=$1 ; i++ )); do
	clear
	seed=$(date +%s)
	echo -e "\x1b[1mSeed:\x1b[0m $seed"
	./generator/target/fast/generator ${@:2} -s $seed >/dev/null
	./solver/target/fast/solver
	sleep 1
done
