import { describe, expect, it } from "vitest";
import { fmtNumber, parseNumber, plainText, texToText } from "./format";

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

  it("reads powers of ten", () => {
    const close = (a: number, b: number) => expect(Math.abs(a - b) / Math.abs(b)).toBeLessThan(1e-12);
    close(parseNumber("2,5e19"), 2.5e19);
    close(parseNumber("2,5·10^19"), 2.5e19);
    close(parseNumber("2,5 * 10^-19"), 2.5e-19);
    close(parseNumber("−1,6×10^(−19)"), -1.6e-19);
    close(parseNumber("5,8x10^28"), 5.8e28);
    close(parseNumber("2,5·10¹⁹"), 2.5e19);
    close(parseNumber("1,6·10⁻¹⁹"), 1.6e-19);
    close(parseNumber("10^19"), 1e19);
    close(parseNumber("-10^3"), -1000);
    expect(parseNumber("2,510^19")).toBeNaN();
    expect(parseNumber("2,5·10")).toBeNaN();
  });
});

describe("fmtNumber", () => {
  it("writes very big and very small numbers in powers of ten", () => {
    expect(fmtNumber(5.8e28)).toBe("5,8·10²⁸");
    expect(fmtNumber(-1.602e-19)).toBe("-1,602·10⁻¹⁹");
    expect(fmtNumber(9.999999e20)).toBe("1·10²¹");
    expect(fmtNumber(1234.5)).toBe("1\u00a0234,5");
    expect(fmtNumber(0.0005)).toBe("0,0005");
    expect(fmtNumber(0)).toBe("0");
  });
});
