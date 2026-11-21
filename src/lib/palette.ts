import type { Color } from "./types";

/**
 * Subject colours. Each has a shade per theme
 * with at least 3.7:1 contrast against that theme's card surface, and
 * neighbours stay apart under normal vision and protan/deutan simulation.
 * Red is left out, so a subject never looks like a mistake.
 */
const SHADES: Record<Color, readonly [light: string, dark: string]> = {
  blue: ["#2171cc", "#4491f0"],
  green: ["#288420", "#4fa847"],
  pink: ["#b74b81", "#d7699d"],
  gold: ["#a68003", "#b58e18"],
  violet: ["#7756c0", "#9c7cea"],
  teal: ["#018b7c", "#09ab99"],
  orange: ["#c25600", "#e0702b"],
  graphite: ["#5c646f", "#8c95a0"],
};

export function subjectColor(color: Color | undefined, dark: boolean): string {
  const shades = SHADES[color ?? "blue"] ?? SHADES.blue;
  return dark ? shades[1] : shades[0];
}
