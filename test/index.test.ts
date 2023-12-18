import { expect, it } from "vitest";

import { getText } from "..";

it("sum from native", () => {
	expect(getText()).toMatchInlineSnapshot('""');
});
