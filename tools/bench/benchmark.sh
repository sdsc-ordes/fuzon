#!/usr/bin/env bash

# Benchmark runtime and memory usage of fuzon
# Compares the working directory version against a baseline branch (main by default)

set -euo pipefail

### Final output path
OUTPUT="profiling.md"
PROFILE='release'
BUILD_ARGS=( )
[[ "${PROFILE}" == 'release' ]] && BUILD_ARGS+=( '--release' )
### setup binaries

# baseline binary
BASE_BRANCH='main'

BASE_DIR=$(mktemp -d)
BASE_URL="$(git config --get remote.origin.url)"
(
    GIT_CLONE_PROTECTION_ACTIVE=false \
    git clone \
        --branch "${BASE_BRANCH}" \
        "${BASE_URL}" \
        "${BASE_DIR}" \
    && cd "${BASE_DIR}" \
    && just build "${BUILD_ARGS[@]}"
)
BASE_BIN="${BASE_DIR}/target/${PROFILE}/fuzon"

# current binary
COMP_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
just build "${BUILD_ARGS[@]}"
COMP_BIN="./target/${PROFILE}/fuzon"

# setup data
DATA_URL="https://ftp.uniprot.org/pub/databases/uniprot/current_release/rdf/proteomes.rdf.xz"
INPUT="/tmp/proteomes.ttl"

# Download data if needed
if [ ! -f ${INPUT} ]; then
    curl "${DATA_URL}" \
    | xz -dc -  \
    | rdfpipe-rs -i rdf-xml -o ttl - \
    > "${INPUT}" || rm "${INPUT}"
fi

### Commands to benchmark
QUERY="tein"
BASE_CMD="${BASE_BIN} index -t100 -q ${QUERY} -s ${INPUT}"
COMP_CMD="${COMP_BIN} index -t100 -q ${QUERY} -s ${INPUT}"

### functions for profiling

cpu_prof() {
    local branch1=$1
    local cmd1=$2
    local branch2=$3
    local cmd2=$4
    local out=$5
    hyperfine --export-markdown "${out}" -r 5 \
        -n "${branch1}" "${cmd1}" \
        -n "${branch2}" "${cmd2}"
}

mem_prof() {
    local name=$1
    local cmd=$2
    local heap_out
    heap_out=$(mktemp)
    echo -n "$name: "
    # shellcheck disable=SC2086
    heaptrack -o "${heap_out}" ${cmd} >/dev/null
    heaptrack_print "${heap_out}.zst" \
    | grep '^peak heap memory'
}

make_report() {
    local cpu=$1
    local mem=$2
    local base_branch=$3

    cat <<-MD
	# fuzon profiling

	> date: $(date -u +%Y-%m-%d)

    Comparing $(git branch --show-current) against $base_branch.
	
	## Timings
	
	Run time compared using hyperfine
	
	$(cat "${cpu}")
	
	## Memory
	
	Heap memory usage compared using heaptrack
	
	$(cat "${mem}")
	
	MD
}


###  Run profiling

## Profile cpu time
HYPF_OUT=$(mktemp)

cpu_prof "${BASE_BRANCH}" "${BASE_CMD}" \
         "${COMP_BRANCH}" "${COMP_CMD}" "${HYPF_OUT}"

## Profile memory
HEAP_OUT=$(mktemp)

mem_prof "${BASE_BRANCH}" "${BASE_CMD}" >  "${HEAP_OUT}"
mem_prof "${COMP_BRANCH}" "${COMP_CMD}" >> "${HEAP_OUT}"


### Reporting
make_report "${HYPF_OUT}" "${HEAP_OUT}" "${BASE_BRANCH}" > "${OUTPUT}"
