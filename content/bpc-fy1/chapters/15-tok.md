+++
title = "Tok intenzity Φ"
topics = ["Tok a Gaussův zákon"]
+++

Tok říká, **kolik pole projde plochou**. Představ si siločáry jako déšť a plochu jako okno. Nejvíc projde, když okno stojí **kolmo** na déšť. Nic neprojde, když je s deštěm **rovnoběžné**.

Plocha se popisuje **vektorem plochy** $S$: šipkou kolmou k ploše (normálou), jejíž délka je velikost plochy.

> [!key] Vzorec
> $$\Phi = E \cdot S \cdot \cos\alpha \qquad \alpha = \text{úhel mezi } E \text{ a normálou plochy} \qquad [\mathrm{N \cdot m^2/C} = \mathrm{V \cdot m}]$$

![Černá šipka je normála plochy](images/tok-pri-ruznych-uhlech.svg "Černá šipka je normála plochy")

## Když je E zadané po složkách

Je-li $E$ = ($E_x$, $E_y$, $E_z$), plochou prochází jen složka **kolmá k ploše**. Vezmi tu, jejíž písmeno **chybí** v názvu roviny:

| Rovina | Normála | Tok |
| --- | --- | --- |
| $xy$ | $z$ | $E_z \cdot S$ |
| $yz$ | $x$ | $E_x \cdot S$ |
| $xz$ | $y$ | $E_y \cdot S$ |

> [!example]
> $E$ = (24, 30, 24) N/C, rovina $xy$, $S = 3\,\mathrm{m^2}$ → $\Phi = E_z \cdot S = 24 \cdot 3$ = **72 N·m²/C**.

> [!trap]
> $\alpha$ je úhel s **normálou**, ne s plochou samotnou. A převáděj jednotky: 100 cm² = 0,01 m² (1 m² = 10 000 cm²).
