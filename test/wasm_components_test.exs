defmodule WasmComponentsTest do
  use ExUnit.Case, async: true

  defp run_component(wasm, function, args) do
    {:ok, pid} = Wasmex.Components.start_link(%{path: wasm, wasi: %Wasmex.Wasi.WasiP2Options{}})
    {:ok, result} = Wasmex.Components.call_function(pid, function, List.wrap(args))
    result
  end

  test "run javascript" do
    assert {:ok, ~s|"Hello World"|} =
             run_component(
               "test/components/javascript_component_s.wasm",
               {"thomas9911:expression/expression@0.1.0", "run"},
               ["'Hello' + ' ' + 'World'"]
             )
  end

  test "run python" do
    assert {:ok, "Hello World"} =
             run_component(
               "test/components/python_component_s.wasm",
               {"thomas9911:expression/expression@0.1.0", "run"},
               ["'Hello' + ' ' + 'World'"]
             )
  end

  test "run starlark" do
    assert {:ok, ~s|"Hello World"|} =
             run_component(
               "test/components/starlark_component_s.wasm",
               {"thomas9911:expression/expression@0.1.0", "run"},
               ["'Hello' + ' ' + 'World'"]
             )
  end
end
