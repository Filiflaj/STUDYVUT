import { describe, expect, it } from "vitest";
import { parseNumber, plainText, texToText } from "./format";

describe("plain text", () => {
  it("turns math into readable text", () => {
    expect(texToText("-2\\,\\mathrm{nC}")).toBe("-2 nC");
    expect(texToText("\\frac{T}{3}")).toBe("T/3");
    expect(texToText("F = \\frac{1}{4\\pi\\varepsilon_0}")).toBe("F = 1/4πε0");
    expect(texToText("r^2 \\cdot 10^{-12}")).toBe("r² · 10⁻¹²");
  });

  it("strips Markdown", () => {
    expect(plainText("Náboj $Q_1$ je **kladný** ![](images/x.svg)")).toBe("Náboj Q1 je kladný");
    expect(plainText("Tok $-2\\,\\mathrm{nC}$")).toBe("Tok -2 nC");
  });
});

describe("parseNumber", () => {
  it("reads comma, point, minus signs and spaces", () => {
    expect(parseNumber("3,14")).toBe(3.14);
    expect(parseNumber(" −226 ")).toBe(-226);
    expect(parseNumber("1 500,5")).toBe(1500.5);
    expect(parseNumber("2e-3")).toBe(0.002);
    expect(parseNumber("abc")).toBeNaN();
    expect(parseNumber("")).toBeNaN();
    expect(parseNumber("1,2,3")).toBeNaN();
  });
});
