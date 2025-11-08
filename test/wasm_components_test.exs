defmodule WasmComponentsTest do
  use ExUnit.Case, async: true

  defp run_component(wasm, function, args) do
    {:ok, pid} = Wasmex.Components.start_link(%{path: wasm, wasi: %Wasmex.Wasi.WasiP2Options{}})
    {:ok, result} = Wasmex.Components.call_function(pid, function, List.wrap(args))
    result
  end

  defp run_js(code) do
    run_component(
      "test/components/javascript_component_s.wasm",
      {"thomastimmer:expression/expression@0.1.0", "run"},
      [code]
    )
  end

  defp run_python(code) do
    run_component(
      "test/components/python_component_s.wasm",
      {"thomastimmer:expression/expression@0.1.0", "run"},
      [code]
    )
  end

  describe "run javascript" do
    wasm_file = "test/components/javascript_component_s.wasm"

    if File.exists?(wasm_file) == false do
      IO.puts("Skipping WasmComponentsTest: #{Path.basename(wasm_file)} not found")
      @describetag :skip
    end

    test "concat text" do
      assert {:ok, ~s|"Hello World"|} = run_js("'Hello' + ' ' + 'World'")
    end

    test "fibonacci" do
      assert {:ok, "55"} =
               run_js("""
               function fib(n) {
                 if (n <= 1) {
                   return n;
                 } else {
                   return fib(n - 1) + fib(n - 2);
                 }
               }
               fib(10);
               """)
    end
  end

  describe "run python" do
    wasm_file = "test/components/python_component_s.wasm"

    if File.exists?(wasm_file) == false do
      IO.puts("Skipping WasmComponentsTest: #{Path.basename(wasm_file)} not found")
      @describetag :skip
    end

    test "concat text" do
      # also use implicit string concat (that works in python)
      assert {:ok, "Hello World"} = run_python("'Hello' + ' ' 'World'")
    end

    test "base64 encode" do
      assert {:ok, "SGVsbG8gV29ybGQ="} =
               run_python("import base64; base64.b64encode(b'Hello World').decode('utf-8')")
    end

    test "fibonacci" do
      assert {:ok, "55"} =
               run_python("""
               def fib(n):
                   if n <= 1:
                       return n
                   else:
                       return fib(n-1) + fib(n-2)
               fib(10)
               """)
    end

    test "error handling" do
      assert {:error,
              """
              Traceback (most recent call last):
                File "<embedded>", line 1, in <module>
              Exception: This is an error
              """} =
               run_python("raise Exception('This is an error')")
    end

    test "regex match" do
      assert {:ok, res} =
               run_python("""
               import re

               re.match('hello', "hello world")
               """)

      assert String.contains?(res, "hello")
    end

    test "uuid4 hex" do
      assert {:ok, res} =
               run_python("""
               from uuid import uuid4
               uuid4().hex
               """)

      assert byte_size(res) == 32
      assert {:ok, _} = Base.decode16(res, case: :lower)
    end

    test "secrets token_hex" do
      assert {:ok, res} =
               run_python("""
               from secrets import token_hex
               token_hex(16)
               """)

      assert byte_size(res) == 32
      assert {:ok, _} = Base.decode16(res, case: :lower)
    end

    test "math gamma" do
      assert {:ok, "7.633143184677179"} =
               run_component(
                 "test/components/python_component_s.wasm",
                 {"thomastimmer:expression/expression@0.1.0", "run"},
                 [
                   """
                   import math
                   math.gamma(0.12345)
                   """
                 ]
               )
    end
  end

  describe "run starlark" do
    wasm_file = "test/components/starlark_component_s.wasm"

    if File.exists?(wasm_file) == false do
      IO.puts("Skipping WasmComponentsTest: #{Path.basename(wasm_file)} not found")
      @describetag :skip
    end

    test "concat text" do
      assert {:ok, ~s|"Hello World"|} =
               run_component(
                 "test/components/starlark_component_s.wasm",
                 {"thomastimmer:expression/expression@0.1.0", "run"},
                 ["'Hello' + ' ' + 'World'"]
               )
    end
  end
end
