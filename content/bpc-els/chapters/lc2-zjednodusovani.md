+++
title = "LC2: Postupné zjednodušování"
topics = ["LC2: Postupné zjednodušování"]
+++

Druhá laboratorní úloha (3. týden). Dvojice rezistorů se postupně nahrazují jedním proměnným rezistorem nastaveným tak, aby se **celkový proud nezměnil**, a jeho odpor se změří. Tak se experimentálně ověří, že náhradní odpory sériové a paralelní kombinace jsou správně. Teorie je v kapitole **03 Obvodové prvky a postupné zjednodušování**.

![Obvod úlohy LC2](images/lc2-obvod.svg "Zdroj U, větev s R1 a větev se sériovými R2 a R3")

## Cíl

- V obvodu a) změřit napětí a proudy všech prvků.
- Nahradit sériovou dvojici $R_2$, $R_3$ jedním rezistorem $R_{23}$ (zapojení b), pak paralelní dvojici $R_1 \parallel R_{23}$ jedním rezistorem $R_{123}$ (zapojení c), vždy se stejným celkovým proudem $I$. Náhradní rezistory změřit ohmmetrem.
- Spočítat teoretické $R_{23}$, $R_{123}$ a porovnat s naměřenými.

**Přístroje:** stabilizovaný zdroj 0–12 V / 300 mA, 4 číslicové multimetry, panely s rezistory $R_1$ = 220 Ω, $R_2$ = 100 Ω, $R_3$ = 150 Ω, proměnný rezistor 0–250 Ω.

## Postup

1. Zapojit obvod a) s miliampérmetry pro $I$, $I_1$, $I_2$; voltmetr na zdroj.
2. Nastavit $U$ = 9 V podle voltmetru.
3. Změřit $U_2$, $U_3$, $I$, $I_1$, $I_2$ (jeden voltmetr se přepojuje).
4. **Každou změnu zapojení dělat s vypnutým zdrojem**; napětí zdroje neměnit, jen vypnout vypínačem.
5. Sériové $R_2$, $R_3$ nahradit proměnným rezistorem P, zapnout, zkontrolovat napětí a nastavit P tak, aby byl **celkový proud $I$ stejný** jako předtím. Změřit $I_1$, $I_2$.
6. Vypnout, P **odpojit z obvodu** (ohmmetr nesmí měřit rezistor zapojený v obvodu) a změřit jeho odpor $R_{23}$. Pozor, aby se nastavení P nezměnilo.
7. Paralelní kombinaci $R_1 \parallel R_{23}$ nahradit proměnným rezistorem, opět nastavit stejné $I$, odpojit a změřit $R_{123}$.
8. Regulátor na minimum, vypnout přístroje, rozpojit, uklidit.
9. Spočítat teoretické $R_{23}$, $R_{123}$ a odchylky.

Tabulka měřených veličin má sloupce $U_2$, $U_3$, $U_2 + U_3$, $I$, $I_1$, $I_2$, $I_1 + I_2$ pro zapojení a), b), c); druhá tabulka náhradní odpory měřené, vypočtené a odchylku.

## Očekávané hodnoty

> [!example] Výpočet pro $U$ = 9 V
> $$R_{23} = R_2 + R_3 = 100 + 150 = 250\ \Omega, \qquad R_{123} = \frac{R_1 R_{23}}{R_1 + R_{23}} = \frac{220\cdot250}{470} = 117{,}0\ \Omega$$
> $$I = \frac{U}{R_{123}} = \frac{9}{117{,}0} = 76{,}9\ \text{mA}, \qquad I_1 = \frac{U}{R_1} = \frac{9}{220} = 40{,}9\ \text{mA}, \qquad I_2 = \frac{U}{R_{23}} = \frac{9}{250} = 36{,}0\ \text{mA}$$
> $$U_2 = R_2 I_2 = 100\cdot0{,}036 = 3{,}60\ \text{V}, \qquad U_3 = R_3 I_2 = 150\cdot0{,}036 = 5{,}40\ \text{V}$$
> Kontroly: $I_1 + I_2 = 76{,}9$ mA $= I$ a $U_2 + U_3 = 9$ V $= U$.
>
> **Jak to říct, když se zeptají:** „Náhradní rezistor je takový, který při stejném napětí odebírá stejný proud. Proto ho nastavuji podle ampérmetru na stejný celkový proud a teprve pak měřím jeho odpor, odpojený z obvodu.“

> [!trap] Na co si dát pozor
> - $R_{23}$ = 250 Ω je přesně horní mez proměnného rezistoru 0–250 Ω; bude vytočený skoro naplno.
> - Ohmmetrem se měří jen rezistor odpojený od obvodu a od zdroje.
> - Po každé změně zapojení zkontrolovat, že se nezměnilo napětí zdroje.

## Závěr

Porovnat teoretické a naměřené náhradní odpory a ověřit, že postupné zjednodušování dává správné náhradní hodnoty; zmínit největší odchylku a její příčiny (tolerance rezistorů, přesnost nastavení proudu a ohmmetru).

## Otázky k zamyšlení

> [!note] Odpovědi
> - **Jde postupné zjednodušování použít v obvodu s více zdroji?** Přímo ne, metoda je pro obvody s jedním zdrojem. S více zdroji se kombinuje se superpozicí (každý zdroj zvlášť), nebo se použije MSP či MUN.
> - **Máme jen rezistory 1 kΩ. Jak sestavit 2500 Ω a kolik jich bude potřeba?** Dva v sérii (2 kΩ) a k nim do série dva paralelně (500 Ω): celkem **4 rezistory**.
