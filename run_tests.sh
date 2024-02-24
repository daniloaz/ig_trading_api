#!/bin/bash

# This script runs the unit tests and integration tests for this project.
# The integration tests are run individually with a sleep interval between
# each test to avoid rate limiting or any other issue with the API.

# Default values.
sleep_interval=3
verbose=0
run_unit=1
run_integration=1

# Help function.
function print_help {
    echo "Usage: ${0} [options]"
    echo
    echo "Options:"
    echo "-h, --help            Show help"
    echo "-v, --verbose         Enable verbose mode"
    echo "-s, --sleep INTERVAL  Set the sleep interval (default is ${sleep_interval} seconds)"
    echo "-u, --unit            Run only unit tests"
    echo "-i, --integration     Run only integration tests"
    echo "-a, --all             Run all tests (default)"
    exit 1
}

# Parse command line options.
while getopts "hvs:uia" opt; do
    case ${opt} in
    h)
        print_help
        ;;
    v)
        verbose=1
        ;;
    s)
        sleep_interval=${OPTARG}
        ;;
    u)
        run_unit=1
        run_integration=0
        ;;
    i)
        run_unit=0
        run_integration=1
        ;;
    a)
        run_unit=1
        run_integration=1
        ;;
    \?)
        echo "Invalid option: ${OPTARG}" 1>&2
        print_help
        ;;
    :)
        echo "Invalid option: ${OPTARG} requires an argument" 1>&2
        print_help
        ;;
    esac
done
shift $((OPTIND - 1))

# Run the unit tests if required.
if [[ ${run_unit} -eq 1 ]]; then
    if [[ ${verbose} -eq 1 ]]; then
        cargo test --lib -- --nocapture
    else
        cargo test --lib
    fi
fi

# Run the integration tests if required.
# RUN_TESTS_THREADS=1 is used to force the tests to run sequentially
# and avoid rate limiting issues with the API.
if [[ ${run_integration} -eq 1 ]]; then
    if [[ $verbose -eq 1 ]]; then
        RUST_TEST_THREADS=1 RUST_TEST_DELAY=${sleep_interval} cargo test --test integration_tests -- --nocapture
    else
        RUST_TEST_THREADS=1 RUST_TEST_DELAY=${sleep_interval} cargo test --test integration_tests
    fi
fi