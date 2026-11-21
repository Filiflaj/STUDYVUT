+++
title = "Magnetické obvody"
topics = ["Magnetické obvody"]
+++

Magnetický obvod je jádro z feromagnetika (případně se vzduchovou mezerou), ve kterém cívka vytváří magnetický tok. Počítá se velmi podobně jako elektrický obvod, jen „odpor“ jádra není konstantní: závisí na tom, jak je jádro nasycené, a čte se z **magnetizační křivky** $B(H)$. Procvičuje se ve **VC6** (v podkladech zadání označené VC5).

## Veličiny a vztahy

| Veličina | Značka | Jednotka | Vztah |
|---|---|---|---|
| intenzita magnetického pole | $H$ | A/m | zdrojem pole je proud |
| magnetická indukce | $B$ | T | $B = \mu H$ |
| permeabilita | $\mu$ | H/m | $\mu = \mu_0 \mu_\text{r}$, $\mu_0 = 4\pi\cdot10^{-7}$ H/m |
| magnetické napětí | $U_\text{m}$ | A | $U_\text{m} = \int H\,\text{d}l$, pro homogenní pole $U_\text{m} = H\,l$ |
| magnetický indukční tok | $\Phi$ | Wb | $\Phi = \int \mathbf{B}\cdot\text{d}\mathbf{S}$, pro homogenní pole $\Phi = B\,S$ |
| magnetický odpor (reluktance) | $R_\text{m}$ | 1/H | $R_\text{m} = \dfrac{l}{\mu S}$ |
| indukčnost | $L$ | H | $L = \dfrac{\Psi}{I} = \dfrac{N\Phi}{I}$ |

Zjednodušení, se kterými se v semináři počítá: pole je v jádře homogenní, průřez je všude stejný, siločáry jsou kolmé na průřez a rozptyl se zanedbává.

**Indukčnost cívky** s $N$ závity na jádře s magnetickým odporem $R_\text{m}$:

$$L = \frac{N\Phi}{I} = \frac{N^2}{R_\text{m}} = A_L N^2$$

kde $A_L$ je **konstanta indukčnosti** jádra (v katalozích feritů v nH/z²). Indukčnost roste s **druhou mocninou** počtu závitů.

## Analogie s elektrickým obvodem

| Elektrický obvod | Magnetický obvod |
|---|---|
| proud $I$ (A) | magnetický tok $\Phi$ (Wb) |
| napětí $U$ (V) | magnetické napětí $U_\text{m}$ (A) |
| odpor $R = \dfrac{l}{\gamma S}$ (Ω) | magnetický odpor $R_\text{m} = \dfrac{l}{\mu S}$ (1/H) |
| zdroj napětí $U_\text{z}$ | cívka, $U_\text{mn} = N I$ |
| Ohmův zákon $U = R I$ | Hopkinsonův zákon $U_\text{m} = R_\text{m}\Phi$ |
| 2. Kirchhoffův zákon | Ampérův zákon $N I = \sum H_k l_k$ |

![Toroid se vzduchovou mezerou](images/toroid-mezera.svg "Toroid s cívkou N, střední siločára čárkovaně, mezera lv; vpravo náhradní obvod: zdroj Umn = NI, nelineární Rmf jádra a lineární Rmv mezery")

## Postup řešení

Cívka je zdrojem magnetického napětí $U_\text{mn} = N I$, které se rozloží podél **střední siločáry** (integrační dráha Ampérova zákona):

$$N I = U_\text{mn} = U_\text{mf} + U_\text{mv} = H_\text{f}\,l_\text{f} + H_\text{v}\,l_\text{v}$$

- **Vzduchová mezera** je lineární: $H_\text{v} = B_\text{v}/\mu_0$, takže $U_\text{mv} = \dfrac{B_\text{v}}{\mu_0}\,l_\text{v}$.
- **Feromagnetikum** je nelineární: $H_\text{f}$ se pro dané $B_\text{f}$ **odečte z magnetizační křivky** (nebo z tabulky).
- Tok je v celém obvodu stejný, $\Phi = B_\text{f} S_\text{f} = B_\text{v} S_\text{v}$. Při stejném průřezu a bez rozptylu je $B_\text{f} = B_\text{v}$.
- Délka střední siločáry toroidu je $l = \pi D_\text{s}$; délka v železe je $l_\text{f} = l - l_\text{v}$.

Druhá možnost je počítat přes magnetické odpory (Hopkinsonův zákon): $U_\text{mf} = \Phi R_\text{mf}$, $U_\text{mv} = \Phi R_\text{mv}$. Protože $\mu_\text{r}$ železa závisí na $B$, vede to stejně na magnetizační křivku.

> [!key] Dva typy úloh
> - **Známá indukce (tok), hledá se proud:** $B \to H_\text{f}$ z křivky, $H_\text{v} = B/\mu_0$, sečíst magnetická napětí, $I = U_\text{mn}/N$. Přímý výpočet.
> - **Známý proud, hledá se indukce:** u jádra **bez mezery** $H = NI/l$ a $B$ se odečte z křivky. S mezerou to přímo nejde (neznámá je v obou členech), řeší se graficky nebo zkusmo.

![Magnetizační křivky](images/magnetizacni-krivky.svg "Magnetizační křivky B(H) materiálů z VC6, překresleno přibližně; hodnoty čtené z grafu jsou orientační")

> [!trap] Na co si dát pozor
> - Průřez v cm² nebo mm² převést na m² (4 cm² = 4·10⁻⁴ m², 12 mm² = 12·10⁻⁶ m²), délky v mm na m.
> - $\mu_0 = 4\pi\cdot10^{-7}$ H/m, ne $4\pi\cdot10^{7}$.
> - Mezera 1 mm má obvykle **větší** magnetické napětí než celé železné jádro: železo má $\mu_\text{r}$ ve stovkách až tisících.
> - Délka v železe je střední siločára **minus mezery**; u jádra se dvěma mezerami se odečítají obě.
> - Indukčnost cívky s železným jádrem **závisí na proudu** (cívka je nelineární prvek).

## Vzorový příklad: toroid s mezerou (VC6, příklad 1)

Toroid z oceli, cívka $N$ = 200 závitů, střední průměr $D_\text{s}$ = 120 mm, průřez $S$ = 4 cm², mezera $l_\text{v}$ = 1,2 mm. Jaký proud je potřeba pro $B_\text{v}$ = 0,7 T? Magnetizační křivka oceli je dána tabulkou:

| $B_\text{f}$ (T) | 0,3 | 0,5 | 0,7 | 0,9 |
|---|---|---|---|---|
| $H_\text{f}$ (A/m) | 66 | 109 | 167 | 262 |

> [!example] Řešení
> **Náhradní obvod:** zdroj $U_\text{mn} = NI$, v sérii nelineární $R_\text{mf}$ (jádro) a lineární $R_\text{mv}$ (mezera), viz obrázek nahoře.
>
> **Délky:**
> $$l = \pi D_\text{s} = \pi\cdot0{,}12 = 0{,}3770\ \text{m}, \qquad l_\text{f} = l - l_\text{v} = 0{,}3770 - 0{,}0012 = 0{,}3758\ \text{m}$$
> **Mezera:**
> $$H_\text{v} = \frac{B_\text{v}}{\mu_0} = \frac{0{,}7}{4\pi\cdot10^{-7}} = 557\,042\ \text{A/m}, \qquad U_\text{mv} = H_\text{v} l_\text{v} = 557\,042\cdot0{,}0012 = 668{,}5\ \text{A}$$
> **Jádro:** průřez je stejný, takže $B_\text{f} = B_\text{v} = 0{,}7$ T a z tabulky $H_\text{f}$ = 167 A/m:
> $$U_\text{mf} = H_\text{f} l_\text{f} = 167\cdot0{,}3758 = 62{,}76\ \text{A} \approx 62{,}8\ \text{A}$$
> **Proud:**
> $$I = \frac{U_\text{mv} + U_\text{mf}}{N} = \frac{668{,}5 + 62{,}8}{200} = \frac{731{,}3}{200} = 3{,}66\ \text{A}$$
> **Porovnání:** $U_\text{mv}/U_\text{mf} = 668{,}5/62{,}8 \approx 10{,}6$. Mezera 1,2 mm „spotřebuje“ asi desetkrát víc magnetického napětí než 376 mm železa.
>
> **Jak to říct, když se zeptají:** „Ampérův zákon po střední siločáře: $NI$ je součet $H l$ v železe a v mezeře. V mezeře je $H = B/\mu_0$, v železe $H$ čtu z magnetizační křivky pro stejné $B$, protože tok i průřez jsou stejné. Délku železa beru bez mezery.“

## Příklady z přednášky EL1

> [!example] Toroid bez mezery, známý proud
> Toroid z ocelolitiny, $N$ = 200, $I$ = 1 A, $D_\text{s}$ = 120 mm, $S$ = 4 cm². Jaký je tok a indukčnost?
> $$U_\text{mn} = NI = 200\ \text{A}, \qquad l_\text{f} = \pi D_\text{s} = 0{,}377\ \text{m}, \qquad H_\text{f} = \frac{200}{0{,}377} = 530\ \text{A/m}$$
> Z křivky ocelolitiny pro 530 A/m: $B_\text{f} \approx 1{,}12$ T.
> $$\Phi = B_\text{f} S = 1{,}12\cdot4\cdot10^{-4} = 448\ \mu\text{Wb}, \qquad L = \frac{N\Phi}{I} = \frac{200\cdot448\cdot10^{-6}}{1} = 89{,}6\ \text{mH}$$

> [!example] Prstenec s mezerou, hledá se počet závitů
> Prstenec s vnějším průměrem $D$ = 90 mm a průřezem 10 × 10 mm, mezera 1 mm, požadovaná $B_\text{v}$ = 0,5 T při $I$ = 5 A, křivka podle tabulky výše.
>
> Střední průměr $D_\text{s} = D - 10 = 80$ mm, $l_\text{f} = \pi\cdot0{,}08 - 0{,}001 = 0{,}2503$ m. Pro $B_\text{f}$ = 0,5 T je $H_\text{f}$ = 109 A/m.
> $$5N = 109\cdot0{,}2503 + \frac{0{,}5}{4\pi\cdot10^{-7}}\cdot10^{-3} = 27{,}28 + 397{,}9 = 425{,}2\ \text{A}, \qquad N = 85{,}03 \to 85\ \text{závitů}$$
> $$L = \frac{N B_\text{v} S}{I} = \frac{85\cdot0{,}5\cdot10^{-4}}{5} = 850\ \mu\text{H}$$

> [!example] Síla elektromagnetu
> Prstencové jádro z oceli E11 složené ze dvou půlek, $S$ = 4 cm², $D_\text{s}$ = 0,177 m, $N$ = 210, $I$ = 0,8 A. Jakou silou drží půlky u sebe?
> $$l_\text{f} = \pi\cdot0{,}177 = 0{,}556\ \text{m}, \qquad H_\text{f} = \frac{210\cdot0{,}8}{0{,}556} = 302\ \text{A/m} \ \Rightarrow\ B_\text{f} \approx 1{,}1\ \text{T (E11)}$$
> Síla jedné styčné plochy je $B^2 S/(2\mu_0)$, ploch jsou dvě:
> $$F = 2\cdot\frac{B^2 S}{2\mu_0} = 2\cdot\frac{1{,}1^2\cdot4\cdot10^{-4}}{2\cdot4\pi\cdot10^{-7}} = 385\ \text{N}$$

Další příklady (C jádro se dvěma mezerami, nelineární indukčnost toroidu) jsou i s postupem v kvízu **vc6**.
