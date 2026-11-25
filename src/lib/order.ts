/**
 * The order in which the options of a choice question are shown: position on
 * the screen → index of the option in the content. Shuffled every time, so
 * that the right option is not always in the same place (the user found it
 * almost always under a), 2026-11-25); `keep` keeps the content's order for
 * options whose order means something (docs/content.md, `keep_order`).
 */
export function optionOrder(count: number, keep: boolean, random: () => number = Math.random): number[] {
  const order = Array.from({ length: count }, (_, i) => i);
  if (keep) return order;
  for (let i = count - 1; i > 0; i--) {
    const j = Math.floor(random() * (i + 1));
    [order[i], order[j]] = [order[j], order[i]];
  }
  return order;
}
