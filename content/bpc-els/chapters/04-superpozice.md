+++
title = "Princip superpozice"
topics = ["Princip superpozice"]
+++

Obvod s více zdroji jde rozložit na několik jednodušších obvodů, v každém z nich působí jen jeden zdroj, a výsledky sečíst. Funguje to ale **jen v lineárních obvodech**. K tomu laboratorní úloha **LC3**.

## Princip

> [!key] Princip superpozice
> Odezva na součet podnětů je rovna součtu odezev na jednotlivé podněty působící samostatně.

V obvodu jsou **podněty** napětí a proudy nezávislých zdrojů a **odezvy** napětí a proudy prvků. Nejjednodušší ilustrace: dva zdroje napětí v sérii s rezistorem $R$,

$$I = \frac{U_1 + U_2}{R} = \frac{U_1}{R} + \frac{U_2}{R} = I_1 + I_2$$

Postup:

1. Obvod se řeší **pro každý zdroj zvlášť**. Ostatní zdroje se nahradí svým **vnitřním odporem**:
   - ideální **zdroj napětí** ($R_\text{i} = 0$) se nahradí **zkratem**,
   - ideální **zdroj proudu** ($R_\text{i} \to \infty$) se **rozpojí**,
   - reálný zdroj nechá v obvodu svůj vnitřní odpor nebo vodivost.
2. Dílčí obvody (každý s jedním zdrojem) se vyřeší třeba zjednodušováním nebo děličem.
3. Dílčí výsledky se **sečtou se znaménky** podle čítacích šipek: $U = U' + U''$, $I = I' + I''$.

> [!trap] Superpozice neplatí pro výkon
> Výkon je kvadratický ($P = R I^2$), takže se dílčí výkony **nesčítají**. Nejdřív se sečtou proudy (nebo napětí) a teprve z výsledku se spočítá výkon.

## Vzorový příklad

![Obvod pro superpozici](images/superpozice-priklad.svg "Zdroj Uz1 s Ri1, rezistor R1, zdroj proudu Iz2 s vnitřní vodivostí Gi2; hledá se U1 a I1")

$U_\text{z1}$ = 12 V, $R_\text{i1}$ = 2 Ω; $I_\text{z2}$ = 4 A (šipka dolů, ven z horního uzlu), $G_\text{i2}$ = 0,5 S; $R_1$ = 4 Ω. Určete napětí a proud rezistoru $R_1$.

> [!example] Příspěvek prvního zdroje ($I_\text{z2}$ rozpojen)
> Zbude zdroj $U_\text{z1}$ s $R_\text{i1}$ a k němu paralelně $R_1$ a $R_\text{i2} = 1/G_\text{i2} = 2\ \Omega$.
> $$R_1 \parallel R_\text{i2} = \frac{4\cdot2}{4 + 2} = 1{,}333\ \Omega, \qquad R' = R_\text{i1} + 1{,}333 = 3{,}333\ \Omega$$
> $$I'_\text{z1} = \frac{12}{3{,}333} = 3{,}6\ \text{A}, \qquad U'_1 = U_\text{z1} - R_\text{i1} I'_\text{z1} = 12 - 2\cdot3{,}6 = 4{,}8\ \text{V}, \qquad I'_1 = \frac{4{,}8}{4} = 1{,}2\ \text{A}$$
> Stejně rychle to dá dělič napětí: $U'_1 = 12\cdot\dfrac{1{,}333}{2 + 1{,}333} = 4{,}8$ V.

> [!example] Příspěvek druhého zdroje ($U_\text{z1}$ nahrazen zkratem)
> Zdroj proudu napájí tři paralelní vodivosti: $G_\text{i1} = 1/2$, $G_1 = 1/4$ a $G_\text{i2}$.
> $$G'' = 0{,}5 + 0{,}25 + 0{,}5 = 1{,}25\ \text{S}, \qquad R'' = \frac{1}{1{,}25} = 0{,}8\ \Omega$$
> Šipka $I_\text{z2}$ míří **ven** z horního uzlu, zdroj tedy proud z uzlu odebírá a horní uzel vyjde záporně:
> $$U''_1 = -I_\text{z2} R'' = -4\cdot0{,}8 = -3{,}2\ \text{V}, \qquad I''_1 = \frac{-3{,}2}{4} = -0{,}8\ \text{A}$$

> [!example] Výsledek
> $$U_1 = U'_1 + U''_1 = 4{,}8 + (-3{,}2) = 1{,}6\ \text{V}, \qquad I_1 = I'_1 + I''_1 = 1{,}2 + (-0{,}8) = 0{,}4\ \text{A}$$
> **Kontrola** jedinou rovnicí MUN pro horní uzel: $(0{,}5 + 0{,}25 + 0{,}5)\,U_1 = 12/2 - 4$, tedy $1{,}25\,U_1 = 2$ a $U_1 = 1{,}6$ V.
>
> **Výkon** na $R_1$ je $P = U_1 I_1 = 1{,}6\cdot0{,}4 = 0{,}64$ W. Součet „dílčích výkonů“ $4{,}8\cdot1{,}2 + 3{,}2\cdot0{,}8 = 8{,}32$ W je nesmysl.
>
> **Jak to říct, když se zeptají:** „Každý zdroj počítám zvlášť, zdroj napětí nahrazuji zkratem a zdroj proudu rozpojím, ale vnitřní odpory reálných zdrojů nechávám. Dílčí výsledky sčítám se znaménkem: druhý zdroj táhne proud z uzlu ven, proto je jeho příspěvek záporný.“

## Kdy superpozice neplatí

Princip stojí na linearitě: když se zdvojnásobí napětí zdroje, zdvojnásobí se všechny proudy. V obvodu s **nelineárním prvkem** (dioda, žárovka, varistor, termistor) to neplatí a dílčí výsledky se sečíst nedají. Přesně to ukazuje druhá část úlohy LC3: s nelineárním rezistorem $R_\text{2n}$ místo $R_2$ se součet proudů naměřených s jedním zdrojem nerovná proudu se dvěma zdroji.

## Superpozice a ostatní metody

- **Výhoda:** každý dílčí obvod je jednoduchý (jeden zdroj), často stačí dělič. Dobře je vidět, kolik který zdroj přispívá.
- **Nevýhoda:** pro $n$ zdrojů se obvod řeší $n$krát. U obvodů s více zdroji a uzly je rychlejší MSP nebo MUN, které dají všechno jednou soustavou.

V úloze LC3 ($U_\text{z1} = U_\text{z2}$ = 10 V, $R_1$ = 100 Ω, $R_2$ = 330 Ω, $R_3$ = 220 Ω) vychází s oběma zdroji $I_1$ = 25,86 mA, $I_2$ = −7,84 mA a $I_3$ = 33,70 mA; postup je v kapitole **LC3: Princip superpozice**.
