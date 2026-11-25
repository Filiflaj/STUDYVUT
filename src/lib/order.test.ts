import { describe, expect, it } from "vitest";
import { optionOrder } from "./order";

describe("optionOrder", () => {
  it("is a permutation of the options", () => {
    for (let n = 2; n <= 8; n++) {
      expect([...optionOrder(n, false)].sort((a, b) => a - b)).toEqual([...Array(n).keys()]);
    }
  });

  it("keeps the order when asked", () => {
    expect(optionOrder(5, true, () => 0)).toEqual([0, 1, 2, 3, 4]);
  });

  it("puts the first option in every place about equally often", () => {
    const places = [0, 0, 0, 0];
    for (let i = 0; i < 8000; i++) places[optionOrder(4, false).indexOf(0)]++;
    for (const n of places) expect(n).toBeGreaterThan(1700);
  });
});
