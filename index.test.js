import { template as mustacheTemplate } from "./transpiled/mustache/mustache_component_s.js";
import { template as liquidTemplate } from "./transpiled/liquid/liquid_component_s.js";
import { template as handlebarsTemplate } from "./transpiled/handlebars/handlebars_component_s.js";
import { template as tinyTemplate } from "./transpiled/tinytemplate/tinytemplate_component_s.js";
import { template as teraTemplate } from "./transpiled/tera/tera_component_s.js";
import { expression as expressionPython } from "./transpiled/python/python_component_s.js";
import { expression as expressionStarlark } from "./transpiled/starlark/starlark_component_s.js";
import { expression as expressionJavascript } from "./transpiled/javascript/javascript_component_s.js";

import { describe, expect, test } from "vitest";

function mustache(template, data) {
  return mustacheTemplate.render(template, JSON.stringify(data));
}

function liquid(template, data) {
  return liquidTemplate.render(template, JSON.stringify(data));
}

function handlebars(template, data) {
  return handlebarsTemplate.render(template, JSON.stringify(data));
}

function tinytemplate(template, data) {
  return tinyTemplate.render(template, JSON.stringify(data));
}

function tera(template, data) {
  return teraTemplate.render(template, JSON.stringify(data));
}

describe("renders mustache", () => {
  test("simple", () => {
    expect(mustache("{{ name }}!!", { name: "hallo" }))
      .toBe("hallo!!");
  });

  test("list", () => {
    expect(mustache(
      `
{{#repo}}
  {{name}}
{{/repo}}
`,
      {
        "repo": [
          { "name": "resque" },
          { "name": "hub" },
          { "name": "rip" },
        ],
      },
    ))
      .toBe(`
  resque
  hub
  rip
`);
  });
});

describe("renders liquid", () => {
  test("simple", () => {
    expect(liquid("{{ name }}!!", { name: "hallo" }))
      .toBe("hallo!!");
  });

  test("list", () => {
    expect(liquid(
      `{% for item in repo %}
  {{ item.name }}{% endfor %}
    `,
      {
        "repo": [
          { "name": "resque" },
          { "name": "hub" },
          { "name": "rip" },
        ],
      },
    ))
      .toBe(`
  resque
  hub
  rip
    `);
  });
});

describe("renders handlebars", () => {
  test("simple", () => {
    expect(handlebars("{{ name }}!!", { name: "hallo" }))
      .toBe("hallo!!");
  });

  test("list", () => {
    expect(handlebars(
      `
{{#each repo}}
  {{name}}
{{/each}}
    `,
      {
        "repo": [
          { "name": "resque" },
          { "name": "hub" },
          { "name": "rip" },
        ],
      },
    ))
      .toBe(`
  resque
  hub
  rip
    `);
  });
});

describe("renders tinytemplate", () => {
  test("simple", () => {
    expect(tinytemplate("{ name }!!", { name: "hallo" }))
      .toBe("hallo!!");
  });

  test("list", () => {
    expect(tinytemplate(
      `{{ for item in repo }}
  { item.name }{{ endfor }}
    `,
      {
        "repo": [
          { "name": "resque" },
          { "name": "hub" },
          { "name": "rip" },
        ],
      },
    ))
      .toBe(`
  resque
  hub
  rip
    `);
  });
});

describe("renders tera", () => {
  test("simple", () => {
    expect(tera("{{ name }}!!", { name: "hallo" }))
      .toBe("hallo!!");
  });

  test("list", () => {
    expect(tera(
      `{% for item in repo %}
  {{ item.name }}{% endfor %}
    `,
      {
        "repo": [
          { "name": "resque" },
          { "name": "hub" },
          { "name": "rip" },
        ],
      },
    ))
      .toBe(`
  resque
  hub
  rip
    `);
  });
});

describe("runs python", () => {
  test("simple function", async () => {
    let script = `
import json
def add(*args):
  return sum(args)

json.dumps(add(1,2,3,4,5))
    `
    expect(expressionPython.run(script)).toBe("15")
  });
})

describe("runs starlark", () => {
  test("simple function", async () => {
    let script = `
def add(*args):
  count = 0
  for item in args:
    count += item
  return count

add(1,2,3,4,5)
    `
    expect(expressionStarlark.run(script)).toBe("15")
  });
})

describe("runs javascript", () => {
  test("simple function", async () => {
    let script = `
function sum(items) {
    return items.reduce((acc, item) => acc + item, 0)
}
  
sum([...Array(5)].map((_, i) => i+1))
    `
    expect(expressionJavascript.run(script)).toBe("15")
  });
})
