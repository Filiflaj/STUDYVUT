+++
title = "LC3: Princip superpozice"
topics = ["LC3: Princip superpozice"]
+++

Třetí laboratorní úloha (4. týden). Měřením i výpočtem se ověří, že v lineárním obvodu se účinky dvou zdrojů sčítají, a měřením, že v obvodu s nelineárním prvkem to neplatí. Teorie je v kapitole **04 Princip superpozice**.

![Obvod úlohy LC3](images/lc3-obvod.svg "Zdroje Uz1 a Uz2, R1 a R3 u levého zdroje, R2 (nebo nelineární R2n) k pravému zdroji")

## Cíl a vybavení

- Lineární obvod ($R_2$): změřit proudy $I_1$, $I_2$, $I_3$ s oběma zdroji, jen s $U_\text{z1}$ a jen s $U_\text{z2}$; spočítat totéž teoreticky a porovnat.
- Nelineární obvod ($R_\text{2n}$ místo $R_2$): změřit totéž a ukázat, že součet dílčích proudů nedá proud s oběma zdroji.

**Přístroje:** 2 stabilizované zdroje 0–12 V / 300 mA, 4 číslicové multimetry, panel s $R_1$ = 100 Ω, $R_2$ = 330 Ω, $R_3$ = 220 Ω a nelineárním $R_\text{2n}$.

## Postup

1. K přípravku připojit oba zdroje a do všech tří větví miliampérmetry (polarita podle šipek proudů). Obvod nejdřív propojit přes **lineární** $R_2$.
2. Voltmetrem nastavit $U_\text{z1}$ = 10 V, pak $U_\text{z2}$ = 10 V.
3. Změřit $I_1$, $I_2$, $I_3$ **i se znaménky**.
4. Zdroj $U_\text{z2}$ nahradit zkratem: vypnout ho, odpojit od něj vodiče a ty spojit. Změřit $I'_1$, $I'_2$, $I'_3$.
5. $U_\text{z2}$ znovu připojit (pozor na polaritu) a zkratem nahradit $U_\text{z1}$. Změřit $I''_1$, $I''_2$, $I''_3$.
6. Přepojit obvod přes **nelineární** $R_\text{2n}$ a zopakovat kroky 2 až 5.
7. Regulátory na minimum, vypnout, rozpojit, uklidit.
8. Spočítat teoretické proudy lineárního obvodu (každý zdroj zvlášť), odchylky a pro obě měření součty $I' + I''$.

> [!trap] Zdroj se nahrazuje zkratem, ne vypnutím
> Vypnutý laboratorní zdroj není ideální zkrat. Proto se od zdroje odpojí vodiče a spojí se spolu. Ideální zdroj napětí má nulový vnitřní odpor, a tak se v superpozici nahrazuje zkratem.

## Očekávané hodnoty (lineární obvod)

> [!example] Jen $U_\text{z1}$ ($U_\text{z2}$ zkratován)
> $R_2$ a $R_3$ jsou paralelně a v sérii s $R_1$:
> $$R_2 \parallel R_3 = \frac{330\cdot220}{550} = 132\ \Omega, \qquad I'_1 = \frac{10}{100 + 132} = \frac{10}{232} = 43{,}10\ \text{mA}$$
> $$U'_\text{A} = I'_1\cdot132 = 5{,}690\ \text{V}, \qquad I'_3 = \frac{5{,}690}{220} = 25{,}86\ \text{mA}, \qquad I'_2 = \frac{5{,}690}{330} = 17{,}24\ \text{mA}$$

> [!example] Jen $U_\text{z2}$ ($U_\text{z1}$ zkratován)
> $R_1$ a $R_3$ jsou paralelně a v sérii s $R_2$; proud ze zdroje $U_\text{z2}$ teče rezistorem $R_2$ **doleva**, proti šipce $I_2$:
> $$R_1 \parallel R_3 = \frac{100\cdot220}{320} = 68{,}75\ \Omega, \qquad I''_2 = -\frac{10}{330 + 68{,}75} = -25{,}08\ \text{mA}$$
> $$U''_\text{A} = 0{,}02508\cdot68{,}75 = 1{,}724\ \text{V}, \qquad I''_3 = \frac{1{,}724}{220} = 7{,}84\ \text{mA}, \qquad I''_1 = -\frac{1{,}724}{100} = -17{,}24\ \text{mA}$$

> [!example] Oba zdroje (superpozice)
> $$I_1 = 43{,}10 - 17{,}24 = 25{,}86\ \text{mA}, \qquad I_2 = 17{,}24 - 25{,}08 = -7{,}84\ \text{mA}, \qquad I_3 = 25{,}86 + 7{,}84 = 33{,}70\ \text{mA}$$
> **Kontrola MUN** (uzel A): $\left(\frac{1}{100} + \frac{1}{330} + \frac{1}{220}\right)U_\text{A} = \frac{10}{100} + \frac{10}{330}$, tedy $0{,}017576\,U_\text{A} = 0{,}130303$ a $U_\text{A} = 7{,}414$ V; $I_3 = 7{,}414/220 = 33{,}70$ mA. Sedí, a 1. KZ: $I_1 = I_2 + I_3 = -7{,}84 + 33{,}70 = 25{,}86$ mA.
>
> **Jak to říct, když se zeptají:** „S jedním zdrojem je to zjednodušování: zbylé dva rezistory jsou paralelně. Proud od druhého zdroje teče rezistorem $R_2$ proti šipce, proto je záporný. Sečtené dílčí proudy dávají proud s oběma zdroji, protože obvod je lineární.“

Záporné $I_2$ znamená, že s oběma zdroji teče proud rezistorem $R_2$ ze zdroje $U_\text{z2}$ do uzlu A.

## Nelineární obvod

S $R_\text{2n}$ se teoretické hodnoty nepočítají, jen se porovná naměřený proud s oběma zdroji se součtem $I' + I''$. Rozdíl ukáže, že **superpozice v nelineárním obvodu neplatí**: odpor $R_\text{2n}$ závisí na napětí, které je na něm, a to je s jedním zdrojem jiné než se dvěma.

## Závěr

Rozhodnout, zda princip superpozice platí a) v lineárním obvodu (ano, součty se shodují s měřením i výpočtem v rámci přesnosti), b) v nelineárním obvodu (ne).

## Otázky k zamyšlení

> [!note] Odpovědi
> - **Jaké součástky smí obvod obsahovat, aby šla použít superpozice?** Jen lineární prvky (rezistory s konstantním odporem, lineární zdroje, lineární řízené zdroje); žádné diody, žárovky, varistory.
> - **Superpozice proti MUN nebo MSP?** Superpozice: jednoduché dílčí obvody, ale řeší se tolikrát, kolik je zdrojů. MUN/MSP: jedna soustava rovnic pro všechno, ale je potřeba ji sestavit a vyřešit.
> - **Co způsobilo nelinearitu?** Nelineární rezistor $R_\text{2n}$ (jeho odpor závisí na napětí nebo proudu).
