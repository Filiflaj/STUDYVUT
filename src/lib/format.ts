import { i18n } from "./i18n/index.svelte";

const DAY = 86_400_000;
const cache = new Map<string, Intl.NumberFormat | Intl.DateTimeFormat>();

function cached<T extends Intl.NumberFormat | Intl.DateTimeFormat>(key: string, make: () => T): T {
  const k = `${i18n.locale}|${key}`;
  let f = cache.get(k) as T | undefined;
  if (!f) {
    f = make();
    cache.set(k, f);
  }
  return f;
}

/** A number in the UI language (decimal comma in Czech). */
export function fmtNumber(n: number, maxDigits = 6): string {
  return cached(`n${maxDigits}`, () => new Intl.NumberFormat(i18n.locale, { maximumFractionDigits: maxDigits })).format(n);
}

/** `part` of `total` as a whole percentage, or null without a total. */
export function percent(part: number, total: number): number | null {
  return total > 0 ? Math.round((part / total) * 100) : null;
}

/** Date and time of a Unix ms timestamp, short ("21. 11. 14:05"). */
export function fmtDateTime(ts: number): string {
  return cached(
    "dt",
    () => new Intl.DateTimeFormat(i18n.locale, { day: "numeric", month: "numeric", hour: "numeric", minute: "2-digit" }),
  ).format(ts);
}

/** A local day number (days since 1970-01-01 in the user's zone) as a date. */
export function fmtDay(day: number, opts: Intl.DateTimeFormatOptions = { day: "numeric", month: "numeric" }): string {
  const key = `d${JSON.stringify(opts)}`;
  // The day number is already local, so format it as UTC.
  return cached(key, () => new Intl.DateTimeFormat(i18n.locale, { ...opts, timeZone: "UTC" })).format(day * DAY);
}

/** Today's date in words ("sobota 21. listopadu"). */
export function fmtToday(): string {
  return cached("today", () => new Intl.DateTimeFormat(i18n.locale, { weekday: "long", day: "numeric", month: "long" })).format(
    Date.now(),
  );
}

/**
 * A typed number: decimal comma or point, a Unicode minus, spaces as
 * thousands separators. NaN when it is no number.
 */
export function parseNumber(input: string): number {
  const s = input
    .trim()
    .replace(/[\s  ]/g, "")
    .replace(/[−–]/g, "-")
    .replace(",", ".");
  if (!/^[+-]?(\d+\.?\d*|\.\d+)(e[+-]?\d+)?$/i.test(s)) return NaN;
  return Number(s);
}

/** The option letter: 0 → "a". */
export const letter = (i: number): string => "abcdefgh"[i] ?? String(i + 1);

const GREEK: Record<string, string> = {
  alpha: "α", beta: "β", gamma: "γ", delta: "δ", Delta: "Δ", varepsilon: "ε", epsilon: "ε", eta: "η",
  theta: "θ", lambda: "λ", mu: "μ", pi: "π", rho: "ρ", sigma: "σ", Sigma: "Σ", tau: "τ", varphi: "φ",
  phi: "φ", Phi: "Φ", omega: "ω", Omega: "Ω", cdot: "·", times: "×", pm: "±", approx: "≈", doteq: "≐",
  le: "≤", ge: "≥", ne: "≠", Rightarrow: "⇒", Leftrightarrow: "⇔", to: "→", infty: "∞", propto: "∝",
};
const SUPERSCRIPT: Record<string, string> = {
  "0": "⁰", "1": "¹", "2": "²", "3": "³", "4": "⁴", "5": "⁵", "6": "⁶", "7": "⁷", "8": "⁸", "9": "⁹", "-": "⁻",
};
const superscript = (s: string) => [...s].map((c) => SUPERSCRIPT[c] ?? c).join("");

/** LaTeX math as readable plain text: \frac{a}{b} → a/b, \mathrm{nC} → nC, \pi → π. */
export function texToText(tex: string): string {
  return tex
    .replace(/\\[,;:! ]/g, " ")
    .replace(/\\(?:mathrm|text|mathbf|mathit|operatorname|vec|bar)\{([^{}]*)\}/g, "$1")
    .replace(/\\(?:d|t)?frac\{([^{}]*)\}\{([^{}]*)\}/g, "$1/$2")
    .replace(/\\sqrt\{([^{}]*)\}/g, "√($1)")
    .replace(/\^\{(-?\d+)\}|\^(-?\d)/g, (_m, braced: string | undefined, bare: string | undefined) =>
      superscript(braced ?? bare ?? ""),
    )
    .replace(/\\([a-zA-Z]+)/g, (_m, name: string) => GREEK[name] ?? "")
    .replace(/[{}_]/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

/** Markdown reduced to plain text, for previews and search. */
export function plainText(md: string): string {
  return md
    .replace(/!\[[^\]]*\]\([^)]*\)/g, "")
    .replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
    .replace(/\$\$([^$]*)\$\$|\$([^$]*)\$/g, (_m, display: string | undefined, inline: string | undefined) =>
      texToText(display ?? inline ?? ""),
    )
    .replace(/<[^>]+>/g, "")
    .replace(/[*_`#>]/g, "")
    .replace(/\s+/g, " ")
    .trim();
}
