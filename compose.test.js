import { out } from "./transpiled/compose/composed.js";

import { describe, expect, test } from "vitest";

describe("compose", () => {
  test('simple', () => {
    expect(out.run("1 + {{ index }}", `{"index": " 3 "}`)).toBe("4")
  })

  test('liquid', () => {
    expect(out.run("1 + {{ index | strip | plus: 7 }}", `{"index": " 3 "}`)).toBe("11")
  })

  test('liquid loop', () => {
    expect(out.run(`
function calc(data) {
  return data.reduce((i, acc) => acc + i, 0)
}

calc([{% for i in (1..count) %} {{ i }}, {% endfor %}])      
`, `{"count": 9}`)).toBe("45")
  })
})
