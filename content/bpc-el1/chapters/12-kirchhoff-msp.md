+++
title = "Cvičení: Kirchhoffovy rovnice a metoda smyčkových proudů"
topics = ["Kirchhoffovy rovnice a MSP"]
+++

Univerzální metody fungují pro jakýkoli lineární obvod, i s více zdroji. Vedou na soustavu lineárních rovnic, která se ve cvičeních řeší **Cramerovým pravidlem** (ručně), kalkulačkou v maticovém režimu nebo programem LinRov. MSP s maticí 2×2 je druhým příkladem testu v PC7.

## Kolik je rovnic

Obvod má $v$ větví a $n$ uzlů. Neznámých proudů větví je $v$ a stejně tolik nezávislých rovnic:

- **1. KZ** pro $n - 1$ uzlů (poslední uzel by dal závislou rovnici),
- **2. KZ** pro $s = v - n + 1$ nezávislých smyček (nejjednodušeji všechna oka).

Třeba obvod se 6 větvemi a 4 uzly: 3 rovnice z 1. KZ a $6 - 4 + 1 = 3$ z 2. KZ. Napětí rezistorů se do 2. KZ dosadí z Ohmova zákona, takže neznámé jsou jen proudy.

## Cramerovo pravidlo

Soustava $\mathbf{A}\,\mathbf{x} = \mathbf{b}$ má jediné řešení, když $\Delta = \det\mathbf{A} \neq 0$. Pak
$$x_k = \frac{\Delta_k}{\Delta},$$
kde $\Delta_k$ je determinant matice, ve které je $k$-tý **sloupec** nahrazený vektorem pravých stran $\mathbf{b}$.

- 2×2: hlavní diagonála minus vedlejší, $\begin{vmatrix} a & b \\ c & d \end{vmatrix} = ad - bc$.
- 3×3: **Sarrusovo pravidlo** (opsat první dva sloupce vedle matice, sečíst součiny tří hlavních diagonál a odečíst součiny tří vedlejších), nebo rozvoj podle řádku.

> [!example] PC3, př. 2: soustava 2×2
> $$\begin{bmatrix} 800 & -500 \\ -500 & 750 \end{bmatrix}\begin{bmatrix} A \\ B \end{bmatrix} = \begin{bmatrix} 10 \\ 20 \end{bmatrix}$$
> $$\Delta = 800\cdot750 - (-500)(-500) = 600\,000 - 250\,000 = 350\,000$$
> $$\Delta_1 = 10\cdot750 - (-500)\cdot20 = 17\,500, \qquad \Delta_2 = 800\cdot20 - (-500)\cdot10 = 21\,000$$
> $$A = \frac{17\,500}{350\,000} = 0{,}05, \qquad B = \frac{21\,000}{350\,000} = 0{,}06$$

## Metoda Kirchhoffových rovnic

![Obvod se dvěma smyčkami](images/cv-msp-zaklad.svg "Zdroje U1 a U2 s rezistory R1 a R2, společný R3; smyčkové proudy Is1 a Is2")

> [!example] PC5, př. 1 (sbírka 4.1): přímo podle Kirchhoffových zákonů
> $U_1 = 6\ \text{V}$, $U_2 = 18\ \text{V}$, $R_1 = 10\ \Omega$, $R_2 = 20\ \Omega$, $R_3 = 15\ \Omega$. Obvod má 2 uzly a 3 větve: 1 rovnice z 1. KZ a 2 z 2. KZ.
> $$\begin{aligned} \text{uzel 1:}&\quad -I_1 - I_2 + I_3 = 0 \\ \text{smyčka } R_1, R_3, U_1:&\quad R_1 I_1 + R_3 I_3 - U_1 = 0 \\ \text{smyčka } R_2, R_3, U_2:&\quad R_2 I_2 + R_3 I_3 - U_2 = 0 \end{aligned}$$
> Maticově:
> $$\begin{bmatrix} -1 & -1 & 1 \\ 10 & 0 & 15 \\ 0 & 20 & 15 \end{bmatrix}\begin{bmatrix} I_1 \\ I_2 \\ I_3 \end{bmatrix} = \begin{bmatrix} 0 \\ 6 \\ 18 \end{bmatrix}$$
> Sarrusovým pravidlem:
> $$\Delta = (-1)\cdot0\cdot15 + (-1)\cdot15\cdot0 + 1\cdot10\cdot20 - \big[1\cdot0\cdot0 + (-1)\cdot15\cdot20 + (-1)\cdot10\cdot15\big] = 200 - (-450) = 650$$
> Stejně (sloupec nahrazený vektorem $(0;\ 6;\ 18)$) vyjde $\Delta_1 = -60$, $\Delta_2 = 360$, $\Delta_3 = 300$:
> $$I_1 = \frac{-60}{650} = -0{,}092\,31\ \text{A}, \quad I_2 = \frac{360}{650} = 0{,}553\,8\ \text{A}, \quad I_3 = \frac{300}{650} = 0{,}461\,5\ \text{A}$$
> Záporné $I_1$ znamená, že proud teče rezistorem $R_1$ opačně, než ukazuje šipka: silnější zdroj $U_2$ nabíjí $U_1$.
>
> **Jak to říct, když se zeptají:** „Dva uzly dávají jednu nezávislou rovnici z 1. KZ, tři větve a dva uzly dávají dvě nezávislé smyčky. Tři rovnice o třech neznámých jsem vyřešil Cramerovým pravidlem, determinanty Sarrusem.“

Rovnic je hodně, proto se v praxi používají metody, které jich mají méně: MSP (jen smyčkové rovnice) a MUN (jen uzlové rovnice, kapitola **Cvičení: metoda uzlových napětí**).

## Metoda smyčkových proudů (MSP)

Každému oku se přiřadí fiktivní **smyčkový proud** $I_\text{s}$, všechny se stejnou orientací (třeba ve směru hodinových ručiček). Proud větve je smyčkový proud nebo rozdíl dvou smyčkových proudů, takže 1. KZ je splněný sám a zbývá jen $s = v - n + 1$ rovnic z 2. KZ.

Matice $\mathbf{R}\cdot\mathbf{I}_\text{s} = \mathbf{U}$ se píše rovnou ze schématu:

> [!key] Pravidla pro sestavení matice MSP
> - **diagonála**: součet odporů celé smyčky, znaménko **+**,
> - **mimo diagonálu**: odpor společný dvěma smyčkám, znaménko **−** (při stejné orientaci smyčkových proudů; matice je symetrická),
> - **pravá strana**: součet napětí zdrojů ve smyčce, **+** když šipka zdroje míří **proti** smyčkovému proudu, **−** když **souhlasí**.

Postup: zvolit nezávislé smyčky, vyznačit smyčkové proudy, zapsat odporovou matici a vektor napětí, dosadit, vyřešit, dopočítat proudy větví a napětí.

> [!example] PC5, př. 2 (sbírka 5.1): tentýž obvod pomocí MSP
> $$\begin{bmatrix} R_1 + R_3 & -R_3 \\ -R_3 & R_2 + R_3 \end{bmatrix}\begin{bmatrix} I_\text{s1} \\ I_\text{s2} \end{bmatrix} = \begin{bmatrix} U_1 \\ -U_2 \end{bmatrix} \quad\Rightarrow\quad \begin{bmatrix} 25 & -15 \\ -15 & 35 \end{bmatrix}\begin{bmatrix} I_\text{s1} \\ I_\text{s2} \end{bmatrix} = \begin{bmatrix} 6 \\ -18 \end{bmatrix}$$
> Šipka $U_1$ míří dolů, smyčkový proud $I_\text{s1}$ jde levou větví nahoru, tedy proti ní: $+U_1$. Šipka $U_2$ míří dolů a $I_\text{s2}$ jde pravou větví také dolů: $-U_2$.
> $$\Delta = 25\cdot35 - (-15)(-15) = 875 - 225 = 650$$
> $$\Delta_1 = 6\cdot35 - (-15)(-18) = 210 - 270 = -60, \qquad \Delta_2 = 25\cdot(-18) - (-15)\cdot6 = -450 + 90 = -360$$
> $$I_\text{s1} = \frac{-60}{650} = -0{,}092\,31\ \text{A}, \qquad I_\text{s2} = \frac{-360}{650} = -0{,}553\,8\ \text{A}$$
> Proudy větví: $I_1 = I_\text{s1} = -0{,}092\,31\ \text{A}$, $I_2 = -I_\text{s2} = 0{,}553\,8\ \text{A}$ (šipka $I_2$ míří proti $I_\text{s2}$), $I_3 = I_\text{s1} - I_\text{s2} = -0{,}092\,31 + 0{,}553\,8 = 0{,}461\,5\ \text{A}$. Stejně jako Kirchhoffovou metodou, ale se dvěma rovnicemi místo tří.
>
> **Jak to říct, když se zeptají:** „Na diagonále je součet odporů smyčky, mimo diagonálu záporně společný odpor $R_3$. Zdroj, jehož šipka jde proti smyčkovému proudu, píšu kladně. Proud $R_3$ je rozdíl obou smyčkových proudů, protože jím tečou oba proti sobě.“

### Zdroje proudu v MSP

MSP pracuje se zdroji napětí. Reálný zdroj proudu $I$ s paralelním rezistorem $R$ se převede na zdroj napětí $U = R I$ se sériovým $R$ (pozor na směr: šipka napětí míří proti šipce proudu zdroje, jako u každého reálného zdroje). Proud původního paralelního rezistoru se pak musí dopočítat z původního obvodu (1. KZ v uzlu, sbírka 5.4). Ideální zdroj proudu bez paralelního rezistoru se obejde tak, že jím vede **jen jedna** smyčka; její smyčkový proud je pak známý a rovnic ubude.

### Kontrola výsledku

- 1. KZ v libovolném uzlu z vypočtených proudů větví.
- **Bilance výkonů** (Tellegenův teorém): výkon dodaný zdroji se rovná výkonu spotřebovanému rezistory, $\sum U_k I_k = \sum R_j I_j^2$. Ve sbírce 5.14 (COZ2, př. 2.14) vyjde 7,746 W proti 7,745 W; rozdíl je jen zaokrouhlení.

> [!trap] Časté chyby v MSP
> - Smyčkové proudy orientované různě: mimo diagonálu pak vychází +, ne −. Držet jednu orientaci pro všechny.
> - Znaménko zdroje podle toho, jak „vypadá“, místo porovnání šipky se smyčkovým proudem.
> - Proud společné větve je **rozdíl** smyčkových proudů, ne součet.
> - V $\Delta_k$ se nahrazuje sloupec, ne řádek.
