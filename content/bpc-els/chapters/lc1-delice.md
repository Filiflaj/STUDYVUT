+++
title = "LC1: Děliče napětí a proudu"
topics = ["LC1: Děliče napětí a proudu"]
+++

První laboratorní úloha (2. týden). Měřením se ověří vztahy pro dělič napětí a dělič proudu a oba Kirchhoffovy zákony. Teorie je v kapitole **02 Obvodové zákony, zdroje a děliče**.

## Cíl

- Na děliči napětí změřit napětí obou rezistorů a ověřit $U_k = U\,R_k/(R_1 + R_2)$.
- Na děliči proudu změřit proudy obou rezistorů a ověřit $I_k = I\,G_k/(G_1 + G_2)$.
- U obou vyhodnotit relativní odchylku měřených a vypočtených hodnot.

**Přístroje:** stabilizovaný zdroj 0–12 V / 300 mA, 4 číslicové multimetry, panely s rezistory $R_1$ = 220 Ω a $R_2$ = 100 Ω.

## Co připravit do sešitu

- název, krátkou teorii (vzorce děliče napětí a proudu, 1. a 2. KZ),
- schémata obou zapojení s přístroji,
- dvě tabulky:

| Dělič napětí | $U$ (V) | $I$ (mA) | $U_1$ (V) | $U_2$ (V) |
|---|---|---|---|---|
| měřeno | | | | |
| vypočteno | | | | |
| $\delta$ (%) | | | | |

| Dělič proudu | $I$ (mA) | $I_1$ (mA) | $I_2$ (mA) |
|---|---|---|---|
| měřeno | | | |
| vypočteno | | | |
| $\delta$ (%) | | | |

## Postup

**Dělič napětí** (rezistory v sérii):

1. Zapojit $R_1$ a $R_2$ do série se zdrojem a miliampérmetrem; voltmetr nejdřív na svorky zdroje. Hlídat polaritu přístrojů.
2. Zapnout zdroj a podle voltmetru nastavit $U$ = 10 V.
3. Změřit proud $I$ a napětí $U_1$, $U_2$. K měření napětí se používá **jeden voltmetr**, který se za provozu přepojuje.
4. Vypočítat teoretické $I$, $U_1$, $U_2$, přitom **dosadit skutečně naměřené napětí zdroje**, ne přesně 10 V.

**Dělič proudu** (rezistory paralelně):

5. Zapojit $R_1$ a $R_2$ paralelně, do každé větve miliampérmetr a jeden do přívodu.
6. Regulátorem napětí zdroje nastavit podle ampérmetru celkový proud $I$ = 100 mA.
7. Změřit $I_1$ a $I_2$.
8. Regulátor na minimum, vypnout zdroj, vypnout přístroje, rozpojit, uklidit.
9. Vypočítat teoretické $I_1$, $I_2$ ze skutečně naměřeného $I$ a odchylky.

## Očekávané hodnoty

> [!example] Výpočet pro $U$ = 10 V a $I$ = 100 mA
> **Dělič napětí:**
> $$I = \frac{U}{R_1 + R_2} = \frac{10}{320} = 31{,}25\ \text{mA}, \qquad U_1 = U\,\frac{R_1}{R_1 + R_2} = 10\cdot\frac{220}{320} = 6{,}875\ \text{V}, \qquad U_2 = 10\cdot\frac{100}{320} = 3{,}125\ \text{V}$$
> **Dělič proudu:**
> $$I_1 = I\,\frac{R_2}{R_1 + R_2} = 100\cdot\frac{100}{320} = 31{,}25\ \text{mA}, \qquad I_2 = 100\cdot\frac{220}{320} = 68{,}75\ \text{mA}$$
> **Odchylka** (podle prezentace semináře, vztaženo k naměřené hodnotě): například naměřeno $U_1$ = 6,82 V, pak
> $$\delta_{U1} = \frac{6{,}82 - 6{,}875}{6{,}82}\cdot100\ \% = -0{,}81\ \%$$
>
> **Jak to říct, když se zeptají:** „Teoretické hodnoty počítám ze skutečně nastaveného napětí zdroje, protože tím je odchylka jen chybou děliče a přístrojů, ne nastavení zdroje.“

Rezistory mají toleranci (typicky 5 %), takže odchylky v jednotkách procent jsou v pořádku.

## Závěr

Srovnat teoretické a naměřené hodnoty a ověřit vztahy pro dělič napětí a proudu. Dále ověřit **1. KZ** v uzlu děliče proudu ($I_1 + I_2 = I$) a **2. KZ** ve smyčce děliče napětí ($U_1 + U_2 = U$).

## Otázky k zamyšlení

> [!note] Odpovědi
> - **Jak se rozdělí napětí na $N$ stejných rezistorech v sérii?** Rovnoměrně, na každém je $U/N$.
> - **Jak se rozdělí proud mezi $N$ stejných rezistorů paralelně?** Rovnoměrně, každým teče $I/N$.
> - **Dělič 1 : 2 s $R_1 = R_2$ = 200 Ω se zatíží rezistorem 1,5 kΩ paralelně k $R_2$. O kolik klesne výstupní napětí?** $R_2 \parallel R_\text{z} = 176{,}47$ Ω, výstupní poměr $176{,}47/376{,}47 = 0{,}46875$ místo 0,5, tedy pokles o **6,25 %**.
> - **Proč se voltmetr přepojuje a nepoužijí se dva?** Stačí jeden přístroj; každý voltmetr navíc odebírá malý proud a o kus mění obvod.
