+++
title = "Obvodové prvky a postupné zjednodušování"
topics = ["Obvodové prvky", "Postupné zjednodušování"]
+++

Třetí týden: jak vypadají skutečné rezistory, kondenzátory a cívky, jaké metody analýzy obvodů existují a jak se počítá nejjednodušší z nich, **postupné zjednodušování**. K tomu laboratorní úloha **LC2**.

## Ideální a reálné prvky

**Ideální prvek** má jediný parametr: rezistor odpor $R$, kapacitor kapacitu $C$, induktor indukčnost $L$. **Reálné součástky** mají navíc parazitní vlastnosti (rezistor má i malou indukčnost a kapacitu, cívka odpor vinutí…) a jejich parametry závisí na teplotě, kmitočtu, proudu. Modelují se vhodným spojením ideálních prvků R, L, C.

### Rezistory

- Základní vlastnost je **odpor** (Ω). Omezují proud nebo vytvářejí úbytek napětí.
- Vyrábějí se z materiálů s **malým teplotním součinitelem odporu** $\alpha$, aby odpor s teplotou skoro neměnil.
- Proud v rezistoru vytváří Jouleovo teplo, proto má každý rezistor **jmenovité zatížení** (W), tedy výkon, který snese trvale.
- Provedení: **drátové** (vinuté, pro velká zatížení až asi 100 W), **vrstvové** (uhlíková nebo kovová vrstva na keramice, do několika W), **SMD** pro povrchovou montáž.
- Hodnoty jsou v normalizovaných řadách E6, E12, E24, E48, E96, E192. Nejpoužívanější **E12** má v dekádě hodnoty 1; 1,2; 1,5; 1,8; 2,2; 2,7; 3,3; 3,9; 4,7; 5,6; 6,8; 8,2. Značí se nejčastěji **barevným kódem**.
- Proměnné: **potenciometr** (nastavitelný dělič napětí), **trimr** (jednorázové přesné nastavení), **reostat** (výkonový, k regulaci proudu).
- Speciální: **termistor** (odpor závisí na teplotě, měření teploty), **varistor** (odpor závisí na napětí, přepěťová ochrana a stabilizace), **fotorezistor** (odpor klesá s osvětlením, senzor světla).

### Kondenzátory

- Základní vlastnost je **kapacita** (F), dalším důležitým údajem je **jmenovité napětí**.
- Dvě elektrody oddělené dielektrikem (vzduch, papír, plast, slída, keramika).
- **Keramické** (asi 1 pF až 1 µF, 10 V až 10 kV), **fóliové**, **svitkové** (1 nF až 10 µF), **elektrolytické** (1 µF až 1 F, 5 až 500 V): dielektrikum je tenká vrstva Al₂O₃ na naleptané hliníkové fólii, odtud velká kapacita. **Elektrolytický kondenzátor se nesmí přepólovat**, oxidová vrstva by se rozložila. Dále ladicí a dolaďovací kondenzátory.

### Cívky

- Základní vlastnost je **indukčnost** (H). Závisí na počtu závitů, tvaru a magnetických vlastnostech okolí.
- **Vzduchové cívky jsou lineární.** Cívky s feromagnetickým jádrem jsou nelineární: indukčnost závisí na proudu, teplotě i kmitočtu (víc v kapitole o magnetických obvodech).

## Metody analýzy obvodů

| Pro speciální případy | Univerzální |
|---|---|
| postupné zjednodušování | přímé použití Kirchhoffových zákonů |
| metoda úměrných veličin | metoda smyčkových proudů (MSP) |
| transfigurace (hvězda–trojúhelník) | metoda uzlových napětí (MUN) |
| princip superpozice | modifikovaná MUN |
| Théveninova a Nortonova věta | |

## Postupné zjednodušování

**Princip:** obvod s jediným zdrojem se krok za krokem zjednodušuje nahrazováním **sériových** a **paralelních** dvojic rezistorů, až zbude zdroj a jeden rezistor. Pak se spočítá celkový proud a obvod se „rozbaluje“ zpět (zpětný postup), přičemž se dopočítávají napětí a proudy jednotlivých prvků.

- **Výhody:** jednoduchá, stačí základní matematika, dobrá na ruční výpočet.
- **Nevýhody:** pracná, jen pro jednodušší obvody s **jedním zdrojem**, postup je pokaždé jiný (chce zkušenost) a některé obvody takto nejdou (např. můstek, ten potřebuje transfiguraci nebo jinou metodu).

> [!key] Dvě pravidla
> - **Sériově** jsou prvky, kterými teče **stejný proud** (žádná odbočka mezi nimi).
> - **Paralelně** jsou prvky připojené ke **stejné dvojici uzlů** (mají stejné napětí).

### Vzorový příklad

![Obvod pro postupné zjednodušování](images/zjednodusovani-priklad.svg "R4 je přímo na zdroji, R1 je v sérii s paralelní dvojicí R2, R3")

Hodnoty: $U$ = 24 V, $R_1$ = 150 Ω, $R_2$ = 450 Ω, $R_3$ = 370 Ω, $R_4$ = 220 Ω. Určete všechny proudy.

> [!example] Přímý postup (zjednodušování)
> $R_2$ a $R_3$ jsou paralelně:
> $$R_{23} = \frac{R_2 R_3}{R_2 + R_3} = \frac{450\cdot370}{450 + 370} = \frac{166\,500}{820} = 203{,}0\ \Omega$$
> $R_1$ je s nimi v sérii:
> $$R_{123} = R_1 + R_{23} = 150 + 203 = 353\ \Omega$$
> Tato větev a $R_4$ jsou obě přímo na zdroji, každá má napětí $U$.

> [!example] Zpětný postup (dopočítání proudů)
> $$I_4 = \frac{U}{R_4} = \frac{24}{220} = 109{,}1\ \text{mA}, \qquad I_1 = \frac{U}{R_{123}} = \frac{24}{353} = 67{,}99\ \text{mA}$$
> Napětí na paralelní dvojici:
> $$U_2 = I_1 R_{23} = 0{,}06799\cdot203 = 13{,}80\ \text{V}$$
> $$I_2 = \frac{U_2}{R_2} = \frac{13{,}80}{450} = 30{,}67\ \text{mA}, \qquad I_3 = \frac{U_2}{R_3} = \frac{13{,}80}{370} = 37{,}30\ \text{mA}$$
> Celkový proud zdroje: $I = I_1 + I_4 = 67{,}99 + 109{,}1 = 177{,}1$ mA.
>
> Kontrola 1. KZ v uzlu pod $R_1$: $I_2 + I_3 = 30{,}67 + 37{,}30 = 67{,}97 \approx 67{,}99$ mA (rozdíl je zaokrouhlení $U_2$).
>
> **Jak to říct, když se zeptají:** „$R_2$ a $R_3$ jsou mezi stejnými uzly, tak je slučuji paralelně; s $R_1$ mají společný proud, tak je přičtu sériově. $R_4$ je přímo na zdroji, jeho proud je jen $U/R_4$. Zpátky jdu přes napětí na paralelní dvojici.“

V prezentaci vychází $I_1$ = 67,98 mA a $I_3$ = 37,31 mA: tam se počítá s nezaokrouhleným $R_{23}$ = 203,05 Ω. Rozdíl v poslední číslici je jen zaokrouhlení.

> [!trap] Na co si dát pozor
> - $R_4$ se se zbytkem **neslučuje do série**: je přímo na zdroji, paralelně k větvi $R_{123}$.
> - Kdo slučuje celý obvod až na jeden rezistor ($R_{123} \parallel R_4 = 135{,}5$ Ω), dostane jen celkový proud 177,1 mA. Proudy větví stejně musí dopočítat zpětným postupem.
> - Napětí na paralelní dvojici je $I_1 R_{23}$, ne $U - I_4 R_4$ (to je nula).

## Obvod úlohy LC2

![Obvod úlohy LC2](images/lc2-obvod.svg "Zdroj U, větev s R1 a větev se sériovými R2 a R3")

V úloze LC2 je $R_1$ = 220 Ω paralelně k sériové dvojici $R_2$ = 100 Ω a $R_3$ = 150 Ω, napětí $U$ = 9 V:

$$R_{23} = R_2 + R_3 = 250\ \Omega, \qquad R_{123} = \frac{R_1 R_{23}}{R_1 + R_{23}} = \frac{220\cdot250}{470} = 117{,}0\ \Omega$$

Podrobný postup měření je v kapitole **LC2: Postupné zjednodušování**.
