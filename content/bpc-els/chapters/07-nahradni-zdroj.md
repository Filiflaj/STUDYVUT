+++
title = "Metoda náhradního zdroje (Thévenin a Norton)"
topics = ["Metoda náhradního zdroje"]
+++

Když nás zajímá **jediná větev** obvodu (zátěž), je zbytečné řešit celý obvod. Všechno kromě zátěže se nahradí jedním reálným zdrojem a zátěž se počítá jako u děliče. Procvičuje se ve **VC3**, měří v úloze **LC4** a používá se i u nelineárních obvodů (VC4).

## Théveninova a Nortonova věta

Lineární část obvodu jde vzhledem ke dvěma svorkám A–B nahradit:

- **zdrojem napětí** $U_\text{i}$ se sériovým odporem $R_\text{i}$ (**Théveninova věta**), nebo
- **zdrojem proudu** $I_\text{i}$ s paralelní vodivostí $G_\text{i}$ (**Nortonova věta**).

Náhradní zdroj je vzhledem ke svorkám A–B **ekvivalentní** s původním obvodem: pro stejnou zátěž $R_\text{z}$ dá stejné $U_\text{z}$ a $I_\text{z}$.

> [!key] Jak se určí parametry
> - $U_\text{i} = U_0$: napětí na svorkách A–B **naprázdno** (zátěž odpojená).
> - $I_\text{i} = I_\text{k}$: proud svorkami A–B **nakrátko**.
> - $R_\text{i}$: odpor do svorek A–B, když se v nahrazované části **zdroje napětí nahradí zkratem a zdroje proudu rozpojí**. Kontrolně $R_\text{i} = U_0/I_\text{k}$.
> - Thévenin a Norton spolu souvisí jako převod zdrojů: $I_\text{i} = U_\text{i}/R_\text{i}$, $G_\text{i} = 1/R_\text{i}$.

Pak už je to dělič:

$$I_\text{z} = \frac{U_\text{i}}{R_\text{i} + R_\text{z}}, \qquad U_\text{z} = R_\text{z} I_\text{z} = U_\text{i}\,\frac{R_\text{z}}{R_\text{i} + R_\text{z}}$$

Kdy se metoda hodí:

- počítá se proud nebo napětí **jedné větve**, hlavně pro **více hodnot zátěže** (obvod se řeší jen jednou),
- hledá se **vnitřní odpor** obvodu, třeba kvůli výkonovému přizpůsobení,
- přechodné děje s jedním akumulačním prvkem,
- **nelineární zátěž**: nahrazovaná část musí být lineární, zátěž může být jakákoli (kapitola 08).

### Maximální výkon do zátěže

Výkon v zátěži $P_\text{z} = R_\text{z} I_\text{z}^2$ je největší, když $R_\text{z} = R_\text{i}$ (výkonové přizpůsobení). Pak

$$P_\text{z,max} = \frac{U_\text{i}^2}{4R_\text{i}}$$

## Příklad 1: zátěž s proměnným odporem

![Obvod příkladu 1](images/nz-priklad.svg "Zátěž R2 mezi A a B; za ní R1, zdroj proudu Iz2, R3 a větev R4 se zdrojem Uz1")

$U_\text{Z1}$ = 20 V, $I_\text{Z2}$ = 15 mA, $R_1$ = 30 Ω, $R_3$ = 10 Ω, $R_4$ = 20 Ω. Spočítejte $U_2$ pro $R_2$ = 10, 20, 30, 40 a 50 Ω.

> [!example] Vnitřní odpor
> Zdroj $U_\text{Z1}$ zkratovat, zdroj $I_\text{Z2}$ rozpojit. Od svorek A–B je pak $R_1$ v sérii s paralelní dvojicí $R_3$, $R_4$:
> $$R_\text{i} = R_1 + \frac{R_3 R_4}{R_3 + R_4} = 30 + \frac{10\cdot20}{10 + 20} = 30 + 6{,}667 = 36{,}67\ \Omega$$

> [!example] Napětí naprázdno pomocí MUN
> Bez zátěže teče $R_1$ nulový proud, takže $U_\text{i}$ je napětí uzlu 1 (nad $I_\text{Z2}$ a $R_3$) proti dolnímu vodiči. Zdroj $U_\text{Z1}$ s $R_4$ se převede na zdroj proudu $I_\text{z1} = U_\text{Z1}/R_4 = 20/20 = 1$ A do uzlu 1. Jediná rovnice MUN:
> $$(G_3 + G_4)\,U_\text{i} = I_\text{z1} + I_\text{Z2}, \qquad (0{,}1 + 0{,}05)\,U_\text{i} = 1 + 0{,}015, \qquad U_\text{i} = \frac{1{,}015}{0{,}15} = 6{,}767\ \text{V}$$
> **Kontrola pomocí MSP:** $I_\text{Z2}$ s $R_3$ se převede na zdroj napětí $I_\text{Z2} R_3 = 0{,}015\cdot10 = 0{,}15$ V. Jediná smyčka: $(R_3 + R_4)\,I_\text{S} = -U_\text{Z1} + 0{,}15$, tedy $I_\text{S} = -19{,}85/30 = -0{,}6617$ A. Napětí $U_{R4} = -0{,}6617\cdot20 = -13{,}23$ V a $U_\text{i} = U_{R4} + U_\text{Z1} = -13{,}23 + 20 = 6{,}77$ V.

> [!example] Výsledky pro všechny zátěže
> $$I_2 = \frac{U_\text{i}}{R_\text{i} + R_2}, \qquad U_2 = R_2 I_2; \qquad \text{např. } R_2 = 10\ \Omega:\ I_2 = \frac{6{,}767}{46{,}67} = 0{,}1450\ \text{A},\ U_2 = 1{,}450\ \text{V}$$
>
> | $R_2$ (Ω) | 10 | 20 | 30 | 40 | 50 |
> |---|---|---|---|---|---|
> | $U_2$ (V) | 1,450 | 2,388 | 3,045 | 3,530 | 3,904 |
>
> **Jak to říct, když se zeptají:** „Obvod mimo zátěž je lineární, takže ho nahradím zdrojem $U_\text{i}$ s $R_\text{i}$. $R_\text{i}$ je odpor do svorek se zdrojem napětí nahrazeným zkratem a zdrojem proudu rozpojeným. $U_\text{i}$ je napětí naprázdno; $R_1$ pak neteče proud, takže je to napětí uzlu nad $R_3$. Pak už jen dělič pro každou zátěž.“

## Příklad 2: můstek

![Můstek](images/most.svg "R1 až R4 v můstku, mezi uzly A a B rezistor RG s proudem IG, zdroj U mezi levým a pravým uzlem")

$U$ = 12 V (šipka doprava, levý uzel je kladný), $R_1$ = 10 Ω, $R_2$ = 50 Ω, $R_3$ = 20 Ω, $R_4$ = 40 Ω, $R_\text{G}$ = 2 Ω. Určete proud $I_\text{G}$.

> [!example] Řešení
> **$R_\text{i}$:** zdroj se zkratuje, tím se spojí levý a pravý uzel. Od A k tomuto spojenému uzlu jsou paralelně $R_1$, $R_2$, od něj k B paralelně $R_3$, $R_4$:
> $$R_\text{i} = \frac{R_1 R_2}{R_1 + R_2} + \frac{R_3 R_4}{R_3 + R_4} = \frac{10\cdot50}{60} + \frac{20\cdot40}{60} = 8{,}333 + 13{,}333 = 21{,}667\ \Omega$$
> **$U_\text{i}$:** bez $R_\text{G}$ jsou to dva nezávislé děliče. Napětí uzlu A proti pravému uzlu je napětí na $R_2$, uzlu B napětí na $R_4$; podle 2. KZ
> $$U_\text{i} = U_2 - U_4 = U\left(\frac{R_2}{R_1 + R_2} - \frac{R_4}{R_3 + R_4}\right) = 12\left(\frac{50}{60} - \frac{40}{60}\right) = 10 - 8 = 2\ \text{V}$$
> $$I_\text{G} = \frac{U_\text{i}}{R_\text{i} + R_\text{G}} = \frac{2}{21{,}667 + 2} = 84{,}51\ \text{mA}$$
> Stejné $U_\text{i}$ dá MUN (uzlová napětí 10 V a 8 V) i MSP (smyčkové proudy 0,4 A a 0,2 A).
>
> **Jak to říct, když se zeptají:** „Když vyjmu $R_\text{G}$, rozpadne se můstek na dva děliče a $U_\text{i}$ je rozdíl jejich výstupních napětí. Při zkratovaném zdroji jsou levý a pravý uzel jeden uzel, proto $R_1 \parallel R_2$ plus $R_3 \parallel R_4$.“

Můstek je **vyvážený**, když $R_1/R_2 = R_3/R_4$: pak $U_\text{i} = 0$ a $I_\text{G} = 0$ pro jakékoli $R_\text{G}$.

> [!trap] Na co si dát pozor
> - Při určování $R_\text{i}$ se zdroj proudu **rozpojí**, ne zkratuje; zdroj napětí se **zkratuje**.
> - $U_\text{i}$ se počítá **bez zátěže**. Rezistor v sérii se svorkou, kterým naprázdno neteče proud (tady $R_1$ v příkladu 1), na $U_\text{i}$ nemá vliv, ale do $R_\text{i}$ patří.
> - Znaménko $U_\text{i}$ musí odpovídat šipce hledaného proudu; když vyjde záporné, teče proud opačně.

## Úloha LC4 v číslech

$U$ = 10 V, sériový $R_1$ = 2,2 kΩ, příčný $R_2$ = 4,7 kΩ, zátěže $R_\text{z1}$ = 330 Ω a $R_\text{z2}$ = 680 Ω:

$$U_\text{i} = 10\cdot\frac{4{,}7}{2{,}2 + 4{,}7} = 6{,}812\ \text{V}, \qquad R_\text{i} = \frac{2{,}2\cdot4{,}7}{6{,}9} = 1{,}499\ \text{k}\Omega, \qquad I_\text{i} = \frac{U}{R_1} = 4{,}545\ \text{mA}$$

Postup měření a výsledky pro obě zátěže jsou v kapitole **LC4: Metoda náhradního zdroje**. Příklady VC3 (včetně výkonového přizpůsobení) jsou v kvízu **vc3**.
