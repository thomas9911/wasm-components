#! /bin/env bash

function escape_via_elixir() {
    elixir -e "IO.inspect(\"$1\")"
}

function escape_via_python() {
    python -c "import json; print(json.dumps('''$1'''))"
}

function escape_via_javascript() {
    node -e "console.log(JSON.stringify(\`$1\`))"
}

escaper=escape_via_javascript

function run() {
    script="$1"
    output=$(wasmtime run --invoke "run($($escaper "$script"))" build/python_component_s.wasm)
    if [ $? -ne 0 ]; then
        echo "Failed to run script"
        echo "$output"
        exit 1
    fi
    # shellcheck disable=SC2001
    output=$(echo "$output" | sed 's/ok("\([^"]*\)")/\1/g')
    echo "$output"
}

ran=0
failed=0

function assert() {
    ran=$((ran + 1))
    test_name="$1"
    expected="$2"
    output="$3"
    if [ "$output" != "$expected" ]; then
        failed=$((failed + 1))
        echo "$test_name: Assertion failed! ❌"
        echo "Expected: $expected"
        echo "Got: $output"
        exit 1
    else
        echo "$test_name: Assertion passed ✅"
    fi
}

function success() {
    ran=$((ran + 1))
    test_name="$1"
    output="$2"
    if [[ $output == err\(*\) ]]; then
        failed=$((failed + 1))
        echo "$test_name: Failed ❌ Output: $output"
    else
        echo "$test_name: Success ✅ Output: $output"
    fi
}

function urllib_test() {
    # Http requests cannot be made in wasi, sockets is not supported.
    script="
import urllib.request
req = urllib.request.Request('http://www.example.com/')
with urllib.request.urlopen(req) as f:
    f.read().decode('utf-8')
"
    run "$script"
}

function regex_test() {
    script="
import re

re.match('hello', \"hello world\")
"
    run "$script"
}

function uuid_test() {
    script="
from uuid import uuid4
uuid4().hex
"
    run "$script"
}

function secrets_test() {
    script="
from secrets import token_hex
token_hex(16)
"
    run "$script"
}

function math_test() {
    script="
import math
math.gamma(0.12345)
"
    run "$script"
}

assert regex_test "<re.Match object; span=(0, 5), match=\'hello\'>" "$(regex_test)"
assert math_test "7.633143184677179" "$(math_test)"
success uuid_test "$(uuid_test)"
success secrets_test "$(secrets_test)"

echo "Ran $ran tests, $failed failed."
if [ $failed -ne 0 ]; then
    exit 1
fi
