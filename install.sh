#!/bin/sh


set -u

BINARY_DOWNLOAD_PREFIX="https://github.com/AiAnsar1/inoue"
PACKAGE_VERSION="v1.1.7"

download_binary() {
    downloader --check
    need_cmd mktemp
    need_cmd chmod
    need_cmd mkdir
    need_cmd rm
    need_cmd rmdir
    need_cmd tar
    need_cmd which
    need_cmd dirname
    need_cmd awk
    need_cmd cut


    if [ -z "${VERSION:-}" ]; then
        DOWNLOAD_VERSION=$PACKAGE_VERSION
    else
        DOWNLOAD_VERSION=$VERSION
    fi


    get_architecture || return 1
    _arch="$RETVAL"
    assert_nz "$_arch" "arch"

    _ext=""
    case "$_arch" in
        *windows*)
            _ext=".exe"
            ;;
    esac
    _tardir="inoue-${_arch}"
    echo $_tardir
    _url="$BINARY_DOWNLOAD_PREFIX/$DOWNLOAD_VERSION/${_tardir}.tar.gz"
    _dir="$(mktemp -d 2>/dev/null || ensure mktemp -d -t inoue)"
    _file="$_dir/input.tar.gz"
    _goku="$_dir/inoue$_ext"

    say "Downloading inoue from $_url ..." 1>&2

    ensure mkdir -p "$_dir"
    downloader "$_url" "$_file"
    if [ $? != 0 ]; then
      say "Failed to download $_url"
      say "This may be a standard network error, but it may also indicate"
      say "that inoue release process is not working. When in doubt"
      say "please feel free to open an issue!"
      say "https://github.com/jcaromiq/goku/issues/new/choose"
      exit 1
    fi

    ensure tar xf "$_file"  -C "$_dir"

    outfile="./inoue"

    say "Moving $inoue to $outfile ..."
    mv "$_goku" "$outfile"

    _version="$($outfile --version)"
    _retval=$?

    say ""
    say "You can now run the inoue using '$outfile'"

    ignore rm -rf "$_dir"

    return "$_retval"
}

get_architecture() {
    _ostype="$(uname -s)"
    _cputype="$(uname -m)"

    if [ "$_ostype" = Darwin ] && [ "$_cputype" = i386 ]; then
        if sysctl hw.optional.x86_64 | grep -q ': 1'; then
            _cputype=x86_64
        fi
    fi

    if [ "$_ostype" = Darwin ] && [ "$_cputype" = arm64 ]; then

        _cputype=x86_64
    fi


    case "$_ostype" in
        Linux)
            _ostype=Linux-musl
            ;;

        Darwin)
            _ostype=macOS
            ;;

        MINGW* | MSYS* | CYGWIN*)
            _ostype=Windows-msvc
            ;;

        *)
            err "no precompiled binaries available for OS: $_ostype"
            ;;
    esac

    case "$_cputype" in
        x86_64 | x86-64 | x64 | amd64 | aarch64)
            ;;
        *)
            err "no precompiled binaries available for CPU architecture: $_cputype"

    esac

    _arch="$_ostype-$_cputype"

    RETVAL="$_arch"
}

say() {
    green=$(tput setaf 2 2>/dev/null || echo '')
    reset=$(tput sgr0 2>/dev/null || echo '')
    echo "$1"
}

err() {
    red=$(tput setaf 1 2>/dev/null || echo '')
    reset=$(tput sgr0 2>/dev/null || echo '')
    say "${red}ERROR${reset}: $1" >&2
    exit 1
}

need_cmd() {
    if ! check_cmd "$1"
    then err "Installation halted. Reason: [command not found '$1' - please install this command]"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
    return $?
}

need_ok() {
    if [ $? != 0 ]; then err "$1"; fi
}

assert_nz() {
    if [ -z "$1" ]; then err "assert_nz $2"; fi
}


ensure() {
    "$@"
    need_ok "command failed: $*"
}


ignore() {
    "$@"
}


downloader() {
    if check_cmd wget
    then _dld=wget
    elif check_cmd curl
    then _dld=curl
    else _dld='curl or wget'
    fi

    if [ "$1" = --check ]
    then need_cmd "$_dld"
    elif [ "$_dld" = curl ]
    then curl -sSfL "$1" -o "$2"
    elif [ "$_dld" = wget ]
    then wget "$1" -O "$2"
    else err "Unknown downloader"
    fi
}

download_binary "$@" || exit 1