import { describe, expect, it } from "vitest";
import { displayParts, render } from "./markdown";

describe("displayParts", () => {
  it("cuts at top-level \\qquad and keeps the comma on the left", () => {
    expect(displayParts(String.raw`\Delta_1 = -229{,}59, \qquad \Delta_2 = -177{,}085`)).toEqual([
      String.raw`\Delta_1 = -229{,}59,`,
      String.raw`\Delta_2 = -177{,}085`,
    ]);
  });

  it("cuts at a \\quad after a comma or semicolon, not elsewhere", () => {
    expect(displayParts(String.raw`I = 31{,}25\ \text{mA}, \quad U_1 = 6{,}875\ \text{V}`)).toEqual([
      String.raw`I = 31{,}25\ \text{mA},`,
      String.raw`U_1 = 6{,}875\ \text{V}`,
    ]);
    expect(displayParts(String.raw`a = b \quad\Rightarrow\quad c = d`)).toEqual([String.raw`a = b \quad\Rightarrow\quad c = d`]);
    expect(displayParts(String.raw`31{,}\quad 2`)).toEqual([String.raw`31{,}\quad 2`]);
  });

  it("never cuts inside braces, environments or \\left…\\right", () => {
    const keep = [
      String.raw`\text{a \qquad b}`,
      String.raw`\begin{cases} a, \quad x > 0 \\ b \qquad x < 0 \end{cases}`,
      String.raw`\left( a \qquad b \right)`,
      String.raw`\left\{ a \qquad b \right.`,
    ];
    for (const tex of keep) expect(displayParts(tex)).toEqual([tex]);
    expect(displayParts(String.raw`\left( a \right) \qquad b`)).toEqual([String.raw`\left( a \right)`, "b"]);
  });

  it("drops empty parts", () => {
    expect(displayParts(String.raw`\qquad a \qquad`)).toEqual(["a"]);
  });
});

describe("display formulas", () => {
  it("render each part in display style, inside one box", () => {
    const html = render(String.raw`$$\frac{a}{b} = 1, \qquad c = 2$$`).html;
    expect(html.match(/class="katex-display math"/g)).toHaveLength(1);
    expect(html.match(/class="katex"/g)).toHaveLength(2);
    expect(html.match(/\\displaystyle/g)).toHaveLength(2);
    expect(html).not.toContain("katex-error");
  });
});
