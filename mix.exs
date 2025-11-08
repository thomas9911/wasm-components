defmodule WasmComponents.MixProject do
  use Mix.Project

  def project do
    [
      app: :wasm_components,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:wasmex, "~> 0.13.0"}
    ]
  end

  defp aliases do
    [
      build_wasm: [
        "cmd wash build --config-path components/javascript",
        "cmd wash build --config-path components/starlark",
        "cmd wash build --config-path components/python",
        "cmd wash build --config-path components/mustache",
        "cmd wash build --config-path components/liquid",
        "cmd wash build --config-path components/handlebars",
        "cmd wash build --config-path components/tinytemplate",
        "cmd wash build --config-path components/tera"
      ],
      copy_wasm: [
        "cmd cp components/javascript/build/javascript_component_s.wasm test/components",
        "cmd cp components/starlark/build/starlark_component_s.wasm test/components",
        "cmd cp components/python/build/python_component_s.wasm test/components",
        "cmd cp components/mustache/build/mustache_component_s.wasm test/components",
        "cmd cp components/liquid/build/liquid_component_s.wasm test/components",
        "cmd cp components/handlebars/build/handlebars_component_s.wasm test/components",
        "cmd cp components/tinytemplate/build/tinytemplate_component_s.wasm test/components",
        "cmd cp components/tera/build/tera_component_s.wasm test/components"
      ],
      wasm: ["build_wasm", "copy_wasm"]
    ]
  end
end
