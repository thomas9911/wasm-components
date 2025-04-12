import { template as mustacheTemplate } from "./transpiled/mustache/mustache_component_s.js";
import { template as liquidTemplate } from "./transpiled/liquid/liquid_component_s.js";
import { template as handlebarsTemplate } from "./transpiled/handlebars/handlebars_component_s.js";

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
