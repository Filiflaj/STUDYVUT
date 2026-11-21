+++
title = "LC4: Metoda náhradního zdroje"
topics = ["LC4: Metoda náhradního zdroje"]
+++

Čtvrtá laboratorní úloha (7. týden, souběžně s VC3). Část obvodu se nahradí náhradním zdrojem napětí $U_\text{i}$, $R_\text{i}$ a ověří se, že zátěž má v původním i náhradním obvodu stejné napětí a proud. Teorie je v kapitole **07 Metoda náhradního zdroje**.

![Obvod úlohy LC4](images/lc4-obvod.svg "a) zdroj U, sériový R1, příčný R2 a zátěž Rz na svorkách A–B, b) náhradní zdroj Ui, Ri se stejnou zátěží")

## Cíl

- Vypočítat napětí a proud zátěží $R_\text{z1}$, $R_\text{z2}$ v původním obvodu a ověřit je měřením.
- Vypočítat parametry náhradního zdroje napětí ($U_\text{i}$, $R_\text{i}$) i proudu ($I_\text{i}$, $R_\text{i}$).
- Sestavit náhradní obvod s vypočtenými $U_\text{i}$, $R_\text{i}$ a změřit na něm napětí a proud obou zátěží.
- Parametry náhradního zdroje určit i měřením.

**Přístroje:** stabilizovaný zdroj 0–12 V / 300 mA, 4 číslicové multimetry, panel s $R_1$ = 2,2 kΩ a $R_2$ = 4,7 kΩ, zátěže $R_\text{z1}$ = 330 Ω a $R_\text{z2}$ = 680 Ω, proměnný rezistor pro $R_\text{i}$.

## Domácí příprava: výpočet

> [!example] Parametry náhradního zdroje pro $U$ = 10 V
> Naprázdno je to dělič $R_1$, $R_2$; se zkratovaným zdrojem jsou $R_1$ a $R_2$ paralelně:
> $$U_\text{i} = U\,\frac{R_2}{R_1 + R_2} = 10\cdot\frac{4{,}7}{6{,}9} = 6{,}812\ \text{V}, \qquad R_\text{i} = \frac{R_1 R_2}{R_1 + R_2} = \frac{2{,}2\cdot4{,}7}{6{,}9} = 1{,}499\ \text{k}\Omega$$
> Nortonův zdroj: proud nakrátko (svorky A–B spojené, $R_2$ je zkratovaný) a vodivost
> $$I_\text{i} = \frac{U}{R_1} = \frac{10}{2{,}2} = 4{,}545\ \text{mA} \quad\left(= \frac{U_\text{i}}{R_\text{i}} = \frac{6{,}812}{1{,}499}\right), \qquad G_\text{i} = \frac{1}{R_\text{i}} = 0{,}6671\ \text{mS}$$

> [!example] Zátěže
> $$R_\text{z1} = 330\ \Omega:\quad I_\text{z1} = \frac{6{,}812}{1499 + 330} = \frac{6{,}812}{1829} = 3{,}724\ \text{mA}, \qquad U_\text{z1} = 330\cdot0{,}003724 = 1{,}229\ \text{V}$$
> $$R_\text{z2} = 680\ \Omega:\quad I_\text{z2} = \frac{6{,}812}{1499 + 680} = \frac{6{,}812}{2179} = 3{,}126\ \text{mA}, \qquad U_\text{z2} = 680\cdot0{,}003126 = 2{,}126\ \text{V}$$
>
> **Jak to říct, když se zeptají:** „$U_\text{i}$ je napětí naprázdno, tady prostě dělič. $R_\text{i}$ je odpor do svorek se zdrojem nahrazeným zkratem, tedy $R_1 \parallel R_2$. Zátěž je pak v sérii s $R_\text{i}$ a počítá se jako dělič.“

## Postup měření

1. Zapojit původní obvod a) se zátěží $R_\text{z1}$; voltmetr na zdroj, miliampérmetr a voltmetr na zátěž. Nastavit $U$ = 10 V.
2. Změřit $U_\text{zm}$, $I_\text{zm}$ pro $R_\text{z1}$, pak pro $R_\text{z2}$ (zátěž přepojovat při vypnutém zdroji).
3. Odpojit zátěž a voltmetrem změřit **napětí naprázdno** $U_\text{im}$ na svorkách A–B.
4. Vypnout a **odpojit zdroj**, místo něj dát propojku (zkrat), místo zátěže ohmmetr a změřit **vnitřní odpor** $R_\text{im}$.
5. Sestavit náhradní obvod b): proměnný rezistor nastavit ohmmetrem na vypočtené $R_\text{i}$ **ještě před zapojením**, zdroj nastavit na vypočtené $U_\text{i}$. Připojit $R_\text{z1}$, pak $R_\text{z2}$ a změřit napětí a proud.
6. Regulátor na minimum, vypnout, rozpojit, uklidit.
7. Spočítat odchylky naměřených hodnot od vypočtených.

Orientační hodnoty: $U_\text{im}$ ≈ 6,81 V, $R_\text{im}$ ≈ 1,50 kΩ; $R_\text{z1}$: 1,23 V, 3,72 mA; $R_\text{z2}$: 2,13 V, 3,13 mA.

> [!trap] Na co si dát pozor
> - Při měření $R_\text{im}$ musí být zdroj **odpojený a nahrazený zkratem**. Ohmmetr nikdy na obvod pod napětím.
> - Proměnný rezistor se nastavuje ohmmetrem, když **není** zapojený v obvodu.
> - Proudy jsou jednotky mA, rozsah miliampérmetru tomu přizpůsobit.

## Závěr

Na základě shody napětí a proudu zátěže v původním a náhradním obvodu a shody vypočtených a měřených parametrů $U_\text{i}$, $R_\text{i}$ rozhodnout o platnosti věty o náhradním zdroji.

## Otázky k zamyšlení

> [!note] Odpovědi
> - **Jde věta o náhradním zdroji použít v nelineárních obvodech?** Nahrazovaná část musí být **lineární**; nelineární smí být jen zátěž (tak se počítá VC4).
> - **Co způsobuje malé rozdíly mezi vypočtenými a měřenými $U_\text{i}$, $R_\text{i}$?** Tolerance rezistorů, přesnost přístrojů, odpor propojek a vnitřní odpor voltmetru.
> - **Autobaterie má naprázdno 12,6 V, při proudu 50 A napětí klesne na 11,6 V. Jaký má vnitřní odpor a jaký proud by dodala do zkratu?** $R_\text{i} = (12{,}6 - 11{,}6)/50 = 0{,}02\ \Omega$ a $I_\text{k} = 12{,}6/0{,}02 = 630$ A.
