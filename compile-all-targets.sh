# WARNING: This script is NOT meant for normal installation, it's dedicated
# to the compilation of all supported targets, from a linux machine.
# This is a long process and it involves specialized toolchains.
# For usual compilation do
#     cargo build --release

H1="\n\e[30;104;1m\e[2K\n\e[A" # style first header
H2="\n\e[30;104m\e[1K\n\e[A" # style second header
EH="\e[00m\n\e[2K" # end header
NAME=gem

version=$(./version.sh)
echo -e "${H1}Compilation of all targets for $NAME $version${EH}"

# clean previous build
rm -rf build
mkdir build
echo "   build cleaned"

# build the linux version
target="x86_64-linux"
echo -e "${H2}Compiling the linux version - $target${EH}"
cargo build --release
mkdir "build/$target/"
cp "target/release/$NAME" "build/$target/"

# build versions for other platforms using cargo cross
cross_build() {
    target_name="$1"
    target="$2"
    echo -e "${H2}Compiling the $target_name / $target version${EH}"
    cross build --target "$target" --release
    mkdir "build/$target"
    if [[ $target_name == 'Windows' ]]
    then
        exec="$NAME.exe"
    else
        exec="$NAME"
    fi
    cp "target/$target/release/$exec" "build/$target/"
}
cross_build "Raspberry 32" "armv7-unknown-linux-gnueabihf"
cross_build "Linux GLIBC" "x86_64-unknown-linux-gnu"
cross_build "MUSL" "x86_64-unknown-linux-musl"
cross_build "NetBSD/amd64" "x86_64-unknown-netbsd"
cross_build "MacOS/x86_64" "x86_64-apple-darwin"
cross_build "MacOS/aarch64" "aarch64-apple-darwin"
cross_build "Windows" "x86_64-pc-windows-gnu"

# build, find, and copy the completion scripts
# (they're built as part of the normal compilation)
echo -e "${H2}building and copying completion scripts${EH}"
mkdir build/completion
cp -R target/release/build/*${NAME}*/out/*${NAME}* build/completion
echo "   Done"

# build, find, and copy the man pages
# (they're built as part of the normal compilation)
echo -e "${H2}building and copying man pages${EH}"
mkdir build/man

cp -R target/release/build/*${NAME}*/out/head* build/man
echo "   Done"

echo -e "${H1}Compilations done${EH}"