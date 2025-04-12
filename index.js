import { template } from "./transpiled/mustache/mustache_component_s.js";

console.log(template.render("{{ name }}!!", JSON.stringify({ name: "hallo" })));
