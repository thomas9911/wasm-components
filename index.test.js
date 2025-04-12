import { template } from "./transpiled/mustache/mustache_component_s.js";

import { describe, expect, test } from "vitest";

function mustache(template_string, data) {
  return template.render(template_string, JSON.stringify(data));
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
