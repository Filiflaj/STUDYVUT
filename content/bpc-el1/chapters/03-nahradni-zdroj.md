+++
title = "Úloha 3: Metoda náhradního zdroje (Théveninova věta)"
topics = ["Náhradní zdroj"]
+++

Lineární část obvodu se nahradí jednoduchým zdrojem napětí a měřením se ověří, že se zátěž chová v původním i náhradním obvodu stejně. Hodnoty: $U$ = 12 V, $R_1$ = 680 Ω, $R_2$ = 390 Ω, $R_3$ = 470 Ω, zátěže $R_\text{z1}$ = 390 Ω a $R_\text{z2}$ = 1 kΩ.

## Théveninova a Nortonova věta

**Théveninova věta:** libovolný **lineární** obvod (nebo jeho část) lze vzhledem ke dvěma svorkám A–B nahradit **reálným zdrojem napětí**, tedy ideálním zdrojem $U_\text{i}$ v sérii s odporem $R_\text{i}$:

- $U_\text{i}$ = napětí na svorkách A–B **naprázdno** (bez zátěže),
- $R_\text{i}$ = odpor mezi svorkami A–B, když se v nahrazované části **zdroje napětí zkratují a zdroje proudu rozpojí**.

**Nortonova věta** je obdoba se zdrojem proudu: $I_\text{i} = I_\text{k}$ (proud nakrátko) a paralelní vodivost $G_\text{i} = 1/R_\text{i}$. Oba náhradní zdroje jsou ekvivalentní, $I_\text{i} = U_\text{i}/R_\text{i}$ (úloha 1).

Nahrazovaná část musí být lineární, zátěž může být jakákoli. Metoda se hodí, když hledám napětí a proud **jen v jedné větvi**, hlavně pro více zátěží: obvod se řeší jen jednou.

![Původní obvod a náhradní zdroj](images/thevenin.svg "a) nahrazovaná část s děličem R1–R3 a rezistorem R2, b) náhradní zdroj Ui, Ri se stejnou zátěží")

## Výpočet pro měřený obvod

Naprázdno neteče $R_2$ proud, takže na něm není úbytek a $U_\text{i}$ je napětí děliče $R_1$–$R_3$:

$$U_\text{i} = U\,\frac{R_3}{R_1 + R_3}$$

Se zkratovaným zdrojem $U$ jsou $R_1$ a $R_3$ paralelně a k nim v sérii $R_2$:

$$R_\text{i} = \frac{R_1 R_3}{R_1 + R_3} + R_2$$

Proud a napětí zátěže z náhradního obvodu:

$$I_\text{z} = \frac{U_\text{i}}{R_\text{i} + R_\text{z}}, \qquad U_\text{z} = U_\text{i}\,\frac{R_\text{z}}{R_\text{i} + R_\text{z}}$$

> [!example] Domácí příprava (výpočet)
> $$U_\text{i} = 12\cdot\frac{470}{680 + 470} = 12\cdot\frac{470}{1150} = 4{,}904\ \text{V}$$
> $$R_\text{i} = \frac{680\cdot 470}{1150} + 390 = 277{,}9 + 390 = 667{,}9\ \Omega$$
> Zátěž $R_\text{z1}$ = 390 Ω:
> $$I_\text{z1} = \frac{4{,}904}{667{,}9 + 390} = \frac{4{,}904}{1057{,}9} = 4{,}636\ \text{mA}, \qquad U_\text{z1} = 4{,}904\cdot\frac{390}{1057{,}9} = 1{,}808\ \text{V}$$
> Zátěž $R_\text{z2}$ = 1 kΩ:
> $$I_\text{z2} = \frac{4{,}904}{1667{,}9} = 2{,}940\ \text{mA}, \qquad U_\text{z2} = 4{,}904\cdot\frac{1000}{1667{,}9} = 2{,}940\ \text{V}$$
> (Stejné číslo u $R_\text{z2}$ není chyba: $U_\text{z} = 1000\ \Omega\cdot I_\text{z}$.)
>
> **Jak to říct, když se zeptají:** „$U_\text{i}$ je napětí naprázdno. Přes $R_2$ pak neteče proud, takže je to jen dělič $R_1$, $R_3$. $R_\text{i}$ je odpor do svorek se zdrojem nahrazeným zkratem: $R_1 \parallel R_3$ a k tomu $R_2$.“ Kontrolně dá celý obvod řešený bez Théveninovy věty pro $R_\text{z1}$ stejný proud 4,636 mA.

## Měření

1. **$U_\text{im}$:** původní obvod bez zátěže, $U$ = 12 V, voltmetrem na svorkách A–B.
2. **$R_\text{im}$:** zdroj **odpojit a nahradit propojkou**, ohmmetrem změřit odpor do svorek. Ohmmetr nikdy na obvod pod napětím.
3. **Původní obvod se zátěží** $R_\text{z1}$, pak $R_\text{z2}$: změřit $U_\text{zm}$ a $I_\text{zm}$.
4. **Náhradní obvod:** na rozpojeném obvodu nastavit ohmmetrem proměnný rezistor na $R_\text{i}$ ≈ 668 Ω, pak zdroj na $U_\text{i}$ ≈ 4,90 V, připojit zátěže a změřit $U'_\text{zm}$, $I'_\text{zm}$.
5. Odchylky se počítají **vždy proti vypočteným** $U_\text{z}$, $I_\text{z}$, i pro náhradní obvod.

Orientační hodnoty: $U_\text{im}$ ≈ 4,90 V, $R_\text{im}$ ≈ 668 Ω, u $R_\text{z1}$ ≈ 1,81 V a 4,64 mA, u $R_\text{z2}$ ≈ 2,94 V a 2,94 mA.

> [!trap] Na co si dát pozor
> - $R_2$ ve vzorci pro $U_\text{i}$ není (naprázdno jím neteče proud).
> - Při měření $R_\text{im}$ musí být zdroj odpojený a nahrazený zkratem, ne jen vypnutý a ponechaný v obvodu.
> - Rozsah a jednotky na displeji (V vs. mV, Ω vs. kΩ).
> - Soustavně záporné odchylky proudu mívají příčinu ve vnitřním odporu miliampérmetru.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Co říká Théveninova / Nortonova věta?** Lineární dvojpól nahradím zdrojem napětí $U_\text{i}$ se sériovým $R_\text{i}$, nebo zdrojem proudu $I_\text{i}$ s paralelní $G_\text{i}$.
> - **Jak se určí $U_\text{i}$ a $R_\text{i}$ experimentálně?** $U_\text{i}$ voltmetrem naprázdno; $R_\text{i}$ ohmmetrem při zkratovaném zdroji, nebo jako $U_0/I_\text{k}$.
> - **Proč musí být zdroj při měření $R_\text{im}$ nahrazený zkratem?** Ideální zdroj napětí má nulový vnitřní odpor. Ohmmetr navíc měří vlastním proudem a cizí napětí by měření zkreslilo nebo přístroj poškodilo.
> - **Proč se měří dvě zátěže?** Náhrada platí pro libovolnou zátěž, ne jen pro jednu.
> - **Kdy věta neplatí?** Když nahrazovaná část obsahuje nelineární prvek (dioda).
> - **Norton pro tento obvod:** $I_\text{i} = 4{,}904/667{,}9 = 7{,}343$ mA, $G_\text{i} = 1{,}497$ mS.
> - **Maximální výkon do zátěže:** při $R_\text{z} = R_\text{i}$ ≈ 668 Ω, $P_\text{max} = U_\text{i}^2/(4R_\text{i})$ ≈ 9,0 mW (úloha 6).
