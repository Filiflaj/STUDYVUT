+++
title = "Cvičení: metoda uzlových napětí"
topics = ["MUN ve cvičení", "Metoda uzlových napětí"]
+++

Princip MUN, pravidla pro matici vodivostí a celý výpočet 3×3 Cramerovým pravidlem jsou v kapitole **Úloha 4: Metoda uzlových napětí (MUN)**. Tady je, co ve cvičeních (PC6, COZ3) přibývá: obvody se **zdroji napětí**, proudy větví z původního obvodu, ideální zdroj napětí a srovnání s MSP. MUN s maticí 2×2 je prvním příkladem testu v PC11.

## Připomenutí

$$\mathbf{G}\cdot\mathbf{U} = \mathbf{I}_\text{z}$$

- diagonála: součet vodivostí připojených k uzlu (+), mimo diagonálu: vodivost mezi dvěma uzly (−),
- pravá strana: proudy zdrojů **do** uzlu +, **z** uzlu −,
- počet rovnic $n - 1$ (referenční uzel 0 má napětí nula).

Postup ze cvičení: zvolit referenční uzel a očíslovat ostatní, vyznačit uzlová napětí (šipky k uzlu 0), zapsat vodivosti, zapsat proudy zdrojů, vyřešit, dopočítat zbývající napětí a nakonec proudy.

## Zdroje napětí se převedou na zdroje proudu

MUN umí jen zdroje proudu. Reálný zdroj napětí $U$ se sériovým $R$ se nahradí zdrojem proudu $I_\text{z} = U/R$ s paralelní vodivostí $G = 1/R$. Šipka náhradního proudu míří k té svorce, kde má zdroj napětí **+** (u šipky napětí je to její začátek).

![Obvod se dvěma zdroji napětí](images/cv-mun-dva-zdroje.svg "Zdroje U1 a U2 s rezistory R1 a R3, uzly 1 a 2, rezistory R4 a R5 k zemi, R2 mezi uzly")

> [!example] COZ3, př. 3.5 (sbírka 6.4): proudy větví
> $U_1 = 5\ \text{V}$, $U_2 = 10\ \text{V}$, $R_1 = R_2 = 2\ \text{k}\Omega$, $R_3 = 5\ \text{k}\Omega$, $R_4 = 3\ \text{k}\Omega$, $R_5 = 1\ \text{k}\Omega$. Počítám v mS a mA, napětí pak vyjdou ve V (mA/mS = V).
>
> Převod zdrojů: $I_\text{z1} = U_1/R_1 = 5/2 = 2{,}5\ \text{mA}$ do uzlu 1, $I_\text{z2} = U_2/R_3 = 10/5 = 2\ \text{mA}$ do uzlu 2. Vodivosti: $G_1 = G_2 = 0{,}5$, $G_3 = 0{,}2$, $G_4 = 0{,}3333$, $G_5 = 1\ \text{mS}$.
> $$\begin{bmatrix} G_1 + G_2 + G_4 & -G_2 \\ -G_2 & G_2 + G_3 + G_5 \end{bmatrix}\begin{bmatrix} U_{10} \\ U_{20} \end{bmatrix} = \begin{bmatrix} I_\text{z1} \\ I_\text{z2} \end{bmatrix} \quad\Rightarrow\quad \begin{bmatrix} 1{,}3333 & -0{,}5 \\ -0{,}5 & 1{,}7 \end{bmatrix}\begin{bmatrix} U_{10} \\ U_{20} \end{bmatrix} = \begin{bmatrix} 2{,}5 \\ 2 \end{bmatrix}$$
> $$\Delta = 1{,}3333\cdot1{,}7 - (-0{,}5)(-0{,}5) = 2{,}2666 - 0{,}25 = 2{,}0166$$
> $$\Delta_1 = 2{,}5\cdot1{,}7 - (-0{,}5)\cdot2 = 5{,}25, \qquad \Delta_2 = 1{,}3333\cdot2 - (-0{,}5)\cdot2{,}5 = 3{,}9166$$
> $$U_{10} = \frac{5{,}25}{2{,}0166} = 2{,}6034\ \text{V}, \qquad U_{20} = \frac{3{,}9166}{2{,}0166} = 1{,}9422\ \text{V}$$
> Proudy z **původního** obvodu ($I_1$ teče přes $R_1$ do uzlu 1, $I_2$ přes $R_2$ z uzlu 2 do uzlu 1, $I_3$ přes $R_3$ do uzlu 2, $I_4$ a $I_5$ k zemi):
> $$I_1 = \frac{U_1 - U_{10}}{R_1} = \frac{5 - 2{,}6034}{2} = 1{,}198\ \text{mA}, \qquad I_2 = \frac{U_{20} - U_{10}}{R_2} = \frac{1{,}9422 - 2{,}6034}{2} = -0{,}3306\ \text{mA}$$
> $$I_3 = \frac{U_2 - U_{20}}{R_3} = \frac{10 - 1{,}9422}{5} = 1{,}612\ \text{mA}, \quad I_4 = \frac{U_{10}}{R_4} = 0{,}8678\ \text{mA}, \quad I_5 = \frac{U_{20}}{R_5} = 1{,}942\ \text{mA}$$
> Kontrola 1. KZ v uzlu 1: $1{,}198 - 0{,}3306 - 0{,}8678 = -0{,}0004 \approx 0$ (zaokrouhlení).
>
> **Jak to říct, když se zeptají:** „Zdroje napětí s rezistory jsem převedl na zdroje proudu $U/R$, protože MUN pracuje jen se zdroji proudu. Proud větve se zdrojem počítám z původního obvodu: rozdíl napětí zdroje a uzlového napětí lomeno odporem.“

> [!trap] Proud větve s převedeným zdrojem
> Po převodu už rezistor $R_1$ v náhradním schématu „nesedí“ na stejném místě. Proud $G_1 U_{10}$ není proud původního $R_1$. Skutečný proud je $I_1 = I_\text{z1} - G_1 U_{10} = (U_1 - U_{10})/R_1$.

## Ideální zdroj napětí

Ideální zdroj napětí nemá sériový odpor, převést nejde.

- Když je zapojený **mezi uzel a referenční uzel**, je napětí toho uzlu známé ($U_{k0} = U$) a rovnic ubude. Neznámý zůstane jen proud zdroje.
- Obecně se použije **modifikovaná MUN (MMUN)**: proud zdroje se přidá jako další neznámá (vtéká do jednoho uzlu, vytéká z druhého) a soustava se doplní rovnicí z 2. KZ pro napětí zdroje, třeba $U_{20} = U_2$. Matice se rozšíří o řádek a sloupec s jedničkami.

> [!example] Sbírka 7.1: ideální zdroj $U_2$ mezi uzlem 2 a zemí
> Reálný zdroj $U_1 = 5\ \text{V}$ s $R_1 = 5\ \Omega$ napájí uzel 1, $R_2 = 10\ \Omega$ vede z uzlu 1 k zemi, $R_3 = 5\ \Omega$ spojuje uzly 1 a 2, $R_4 = 10\ \Omega$ vede z uzlu 2 k zemi a mezi uzlem 2 a zemí je ideální zdroj $U_2 = 10\ \text{V}$. Převod: $I_\text{z1} = 5/5 = 1\ \text{A}$, $G_1 = G_3 = 0{,}2\ \text{S}$, $G_2 = G_4 = 0{,}1\ \text{S}$. MMUN s neznámým proudem $I$ zdroje $U_2$ do uzlu 2:
> $$\begin{bmatrix} 0{,}5 & -0{,}2 & 0 \\ -0{,}2 & 0{,}3 & -1 \\ 0 & 1 & 0 \end{bmatrix}\begin{bmatrix} U_{10} \\ U_{20} \\ I \end{bmatrix} = \begin{bmatrix} 1 \\ 0 \\ 10 \end{bmatrix}$$
> Poslední řádek říká $U_{20} = 10\ \text{V}$, z prvního $0{,}5\,U_{10} = 1 + 0{,}2\cdot10$, tedy $U_{10} = 6\ \text{V}$, a z druhého $I = 0{,}3\cdot10 - 0{,}2\cdot6 = 1{,}8\ \text{A}$.

## Millmanova věta

Když má obvod jen **jeden nezávislý uzel** (všechny větve mezi dvěma uzly, jako obvody na obrázcích v kapitole o MSP), dá MUN jedinou rovnici:
$$U = \frac{\sum_k U_k G_k}{\sum_k G_k}$$
$U_k$ je napětí zdroje ve větvi $k$ (+, když má zdroj + u horního uzlu) a $G_k$ vodivost té větve. Je to stejný výsledek jako z MUN, jen rovnou ve tvaru vzorce.

> [!example] Sbírka 5.5 jako MUN
> ![Tři větve mezi dvěma uzly](images/cv-msp-55.svg "U1 s R1 + R4, uprostřed R3 a U2, vpravo R2 a U3")
>
> $U_1 = 10\ \text{V}$, $U_2 = 20\ \text{V}$, $U_3 = 30\ \text{V}$, $R_1 = R_4 = 10\ \Omega$, $R_2 = R_3 = 20\ \Omega$. Vlevo je větev s $U_1$ a $R_1 + R_4 = 20\ \Omega$, uprostřed $R_3$ se zdrojem $U_2$ obráceným (+ dole), vpravo $R_2$ a $U_3$:
> $$U = \frac{\frac{10}{20} - \frac{20}{20} + \frac{30}{20}}{\frac{1}{20} + \frac{1}{20} + \frac{1}{20}} = \frac{1}{0{,}15} = 6{,}667\ \text{V}$$
> Proud prostřední větví dolů je $(U + U_2)/R_3 = 26{,}667/20 = 1{,}333\ \text{A}$, stejně jako metodou smyčkových proudů, ale z jedné rovnice místo dvou.

## MUN, nebo MSP?

Volí se metoda s **menším počtem rovnic**: MUN jich má $n - 1$, MSP $v - n + 1$. Obvody s mnoha paralelními větvemi mezi několika uzly jsou pro MUN, „žebříky“ s mnoha uzly a málo oky pro MSP. Zdroje proudu se hodí MUN, zdroje napětí MSP. V COZ3, př. 3.10 se tentýž obvod počítá oběma metodami pro srovnání.
