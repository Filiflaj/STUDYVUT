+++
title = "Cvičení: magnetické obvody"
topics = ["Výpočet magnetických obvodů", "Magnetický obvod"]
+++

Magnetický obvod (jádro s cívkou, často se vzduchovou mezerou) se počítá podobně jako elektrický: magnetické napětí cívky „pohání“ magnetický tok přes magnetické odpory. Rozdíl je v tom, že feromagnetikum je **nelineární** a jeho vlastnosti se berou z magnetizační (B–H) křivky. Měření pole ve vzduchové mezeře je v kapitole **Úloha 7: Magnetické pole ve vzduchové mezeře**; tady jsou výpočty z PC13 a COZ6.

## Veličiny

| veličina | značka | jednotka | vztah |
|---|---|---|---|
| intenzita magnetického pole | $H$ | A/m | zdrojem je proud |
| magnetická indukce | $B$ | T | $B = \mu H = \mu_0\mu_\text{r}H$ |
| permeabilita | $\mu$ | H/m | $\mu_0 = 4\pi\cdot10^{-7}\ \text{H/m}$ |
| magnetický indukční tok | $\Phi$ | Wb | $\Phi = B S$ (homogenní pole kolmo k ploše) |
| magnetické napětí | $U_\text{m}$ | A | $U_\text{m} = H l$ |
| magnetomotorické napětí cívky | $U_\text{mn}$ | A | $U_\text{mn} = N I$ |
| magnetický odpor (reluktance) | $R_\text{m}$ | H⁻¹ | $R_\text{m} = \dfrac{l}{\mu S}$ |
| indukčnost | $L$ | H | $L = \dfrac{N\Phi}{I} = \dfrac{N^2}{R_\text{m}}$ |

**Ampérův zákon celkového proudu:** $\oint H\,\mathrm{d}l = \sum I$. Pro cívku s $N$ závity na jádře se střední siločárou délky $l$ je $H l = N I$, pro dlouhý přímý vodič $H\cdot2\pi r = I$, tedy $H = I/(2\pi r)$. Dva rovnoběžné vodiče ve vzdálenosti $r$ na sebe působí silou $F = \mu_0 I_1 I_2 l/(2\pi r)$; opačné proudy se odpuzují.

## Analogie s elektrickým obvodem

| elektrický obvod | magnetický obvod |
|---|---|
| proud $I$ (A) | tok $\Phi$ (Wb) |
| napětí $U$ (V) | magnetické napětí $U_\text{m}$ (A) |
| odpor $R = l/(\gamma S)$ | magnetický odpor $R_\text{m} = l/(\mu S)$ |
| Ohmův zákon $U = R I$ | Hopkinsonův zákon $U_\text{m} = R_\text{m}\Phi$ |
| 1. KZ $\sum I = 0$ | $\sum \Phi = 0$ v uzlu větveného obvodu |
| 2. KZ | $N I = \sum H_k l_k$ (Ampérův zákon) |

![Magnetický obvod a jeho náhradní schéma](images/magneticky-obvod.svg "Jádro s cívkou a mezerou; náhradní schéma: zdroj NI, magnetický odpor jádra a mezery v sérii")

Jádro a mezera jsou v sérii: teče jimi stejný tok. Při stejném průřezu mají i stejnou indukci $B_\text{f} = B_\text{v}$. V mezeře je $H_\text{v} = B/\mu_0$, a protože $\mu_\text{r}$ feromagnetika bývá stovky až tisíce, **mezera milimetrové délky potřebuje víc magnetického napětí než celé jádro**.

> [!example] PC13, př. 1 (COZ6, př. 6.7): magnetický odpor s mezerami
> Dvě jádra tvaru U, střední siločára v železe $l_\text{s} = 80\ \text{mm}$, průřez $S = 25\ \text{mm}^2$, $\mu_\text{r} = 200$, dvě mezery po $\delta = 0{,}4\ \text{mm}$:
> $$R_\text{m} = \frac{l_\text{s}}{\mu_0\mu_\text{r}S} + 2\,\frac{\delta}{\mu_0 S} = \frac{0{,}08}{4\pi\cdot10^{-7}\cdot200\cdot25\cdot10^{-6}} + \frac{2\cdot0{,}4\cdot10^{-3}}{4\pi\cdot10^{-7}\cdot25\cdot10^{-6}}$$
> $$R_\text{m} = 1{,}273\cdot10^{7} + 2{,}546\cdot10^{7} = 3{,}819\cdot10^{7}\ \text{H}^{-1}$$
> Dvě mezery celkem 0,8 mm mají dvakrát větší odpor než 80 mm železa.

## Nelineární jádro: postup

Permeabilita feromagnetika není konstantní, proto se nepočítá přes $R_\text{m}$, ale přes magnetická napětí a B–H křivku (tabulka nebo graf).

**Přímá úloha** (zadaný tok nebo indukce, hledá se proud): $B = \Phi/S$ → z křivky $H_\text{f}$ → $N I = H_\text{f} l_\text{f} + \dfrac{B_\text{v}}{\mu_0} l_\text{v}$ → $I$.

**Obrácená úloha** (zadaný proud, hledá se tok) jde přímo jen bez mezery: $H = NI/l$ → z křivky $B$ → $\Phi = BS$. S mezerou se řeší graficky nebo zkoušením.

> [!example] Sbírka 11.15: prstenec s mezerou a bez ní
> Prstenec s vnějším průměrem $D = 250\ \text{mm}$ z kruhového profilu průměru $d = 50\ \text{mm}$, proud $I = 5\ \text{A}$, požadovaná indukce $B_\text{f} = 0{,}7\ \text{T}$. Materiál: $B$ = 0,3; 0,5; 0,7; 0,9 T odpovídá $H$ = 66; 109; 167; 262 A/m.
>
> Střední průměr je $D - d = 200\ \text{mm}$, střední siločára $l_\text{f} = \pi\cdot0{,}2 = 0{,}628\,3\ \text{m}$. Z tabulky $H_\text{f}(0{,}7\ \text{T}) = 167\ \text{A/m}$.
>
> a) Bez mezery:
> $$N_1 = \frac{H_\text{f} l_\text{f}}{I} = \frac{167\cdot0{,}628\,3}{5} = \frac{104{,}9}{5} = 20{,}99 \doteq 21\ \text{závitů}$$
> b) S mezerou $l_\text{v} = 1\ \text{mm}$ (stejný průřez, tedy stejná indukce 0,7 T i v mezeře):
> $$U_\text{mv} = \frac{B}{\mu_0}l_\text{v} = \frac{0{,}7}{4\pi\cdot10^{-7}}\cdot10^{-3} = 557{,}0\ \text{A}, \qquad U_\text{mf} = 167\cdot(0{,}628\,3 - 0{,}001) = 104{,}8\ \text{A}$$
> $$N_2 = \frac{U_\text{mf} + U_\text{mv}}{I} = \frac{104{,}8 + 557{,}0}{5} = 132{,}4 \doteq 132\ \text{závitů}$$
> Milimetrová mezera zvýšila potřebný počet závitů víc než šestkrát.
>
> **Jak to říct, když se zeptají:** „Jádro a mezera jsou v sérii, mají stejný tok a při stejném průřezu i stejnou indukci. Intenzitu v železe beru z tabulky, v mezeře je $B/\mu_0$. Ampérův zákon: $NI$ je součet $H l$ po celé siločáře.“

> [!trap] Na co si dát pozor
> - **Střední** průměr: je-li zadaný vnější průměr prstence $D$ a tloušťka jádra $d$, je střední průměr $D - d$.
> - Délka železa je střední siločára **minus** mezera.
> - Je-li jádro složené z plechů s činitelem plnění $k_\text{p}$, je indukce v železe $B_\text{f} = B_\text{v}/k_\text{p}$ (železa je v průřezu méně).
> - Hodnoty z grafu B–H jsou odečtené přibližně; výsledek se zaokrouhluje na 2 až 3 platné číslice.

## Indukčnost a síla

Indukčnost cívky $L = N\Phi/I$. U nelineárního jádra závisí na proudu, proto se uvádí „pro tento proud“. Pro prstenec z PC13, př. 5 ($N = 85$, $B = 0{,}5\ \text{T}$, $S = 1\ \text{cm}^2$, $I = 5\ \text{A}$) je $L = 85\cdot0{,}5\cdot10^{-4}/5 = 850\ \mu\text{H}$.

Přítažná síla elektromagnetu na jednu pólovou plochu $S$ v mezeře s indukcí $B$:
$$F = \frac{B^2 S}{2\mu_0}$$
Kotva přitahovaná přes **dvě** mezery (jádro tvaru U) má plochu započtenou dvakrát: $F = B^2\cdot2S/(2\mu_0)$. Rozšíření pole v mezeře se zohlední větší plochou $S_\text{v}$.

## Permanentní magnet

Magnet je zdrojem magnetického napětí i jeho odporem. Bez rozptylu a s nástavci zanedbatelného odporu platí $B_\text{p}S_\text{p} = B_\text{v}S_\text{v}$ (stejný tok) a $H_\text{p}l_\text{p} = B_\text{v}l_\text{v}/\mu_0$ (magnetické napětí magnetu pokryje mezeru). Pracovní bod leží na demagnetizační křivce, ideálně tam, kde je součin $BH$ největší. Pro $B_\text{v} = 0{,}5\ \text{T}$, $S_\text{v} = 5\ \text{cm}^2$, $l_\text{v} = 0{,}6\ \text{cm}$ a pracovní bod $B_\text{p} = 0{,}23\ \text{T}$, $H_\text{p} = 90\ \text{kA/m}$ (sbírka 11.11) vyjde $S_\text{p} = 0{,}5\cdot5/0{,}23 = 10{,}87\ \text{cm}^2$ a $l_\text{p} = 0{,}5\cdot0{,}6\cdot10^{-2}/(4\pi\cdot10^{-7}\cdot90\cdot10^3) = 2{,}653\ \text{cm}$.
