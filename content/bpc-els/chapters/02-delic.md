+++
title = "Obvodové zákony, zdroje a děliče"
topics = ["Obvodové zákony a zdroje", "Dělič napětí a proudu"]
+++

Základní pojmy a zákony, bez kterých nejde žádná metoda ze semestru: uzel, větev, smyčka, čítací šipky, Ohmův a Kirchhoffovy zákony, ideální a reálné zdroje, spojování prvků a dva nejjednodušší obvody, dělič napětí a dělič proudu. K tomu laboratorní úloha **LC1**.

## Pojmy

- **Uzel** je vodivé spojení prvků, všude v něm je stejný potenciál (ekvipotenciála). Několik teček spojených drátem bez prvku je **jeden** uzel.
- **Větev** je část obvodu mezi dvěma uzly.
- **Smyčka** je uzavřená cesta obvodem.
- **Oko** (minimální smyčka) je smyčka, kterou nerozděluje žádná větev.

## Čítací šipky

Napětí a proud se ve schématu kreslí šipkami, které určují **čítací směr**: jak se veličina počítá v rovnicích, ne nutně kudy skutečně teče. Když vyjde záporná hodnota, skutečný směr je opačný než šipka.

- **Spotřebičová (pasivní) orientace:** šipky $U$ a $I$ míří **stejným** směrem. Používá se u rezistorů, $U = R\,I$ platí s kladným znaménkem.
- **Zdrojová (aktivní) orientace:** šipky $U$ a $I$ míří **proti sobě**.

Šipka napětí míří od + k − (od vyššího potenciálu k nižšímu). U zdroje napětí tedy šipka $U_\text{z}$ dolů znamená, že horní svorka je kladná.

## Ohmův a Kirchhoffovy zákony

**Ohmův zákon:** $U = R\,I$, tedy $I = U/R$ a $R = U/I$. Vodivost $G = 1/R$ (siemens).

**1. Kirchhoffův zákon (proudový):** algebraický součet proudů v uzlu je nulový, $\sum I = 0$. V semináři se proudy **vytékající** z uzlu píšou s **+** a **vtékající** s **−**.

**2. Kirchhoffův zákon (napěťový):** algebraický součet napětí podél uzavřené smyčky je nulový, $\sum U = 0$. Napětí, jehož šipka souhlasí se směrem obíhání smyčky, má +, proti směru −.

> [!example] Rovnice pro jednoduchý obvod
> Zdroj $U$ napájí přes $R_1$ uzel 1, z něj vedou $R_2$ a $R_3$ k zemi (proudy $i_2$, $i_3$), přes $R_1$ teče $i_1$.
> - uzel 1: $-i_1 + i_2 + i_3 = 0$
> - smyčka se zdrojem, $R_1$ a $R_2$: $-u + u_1 + u_2 = 0$
> - smyčka s $R_2$ a $R_3$: $-u_2 + u_3 = 0$

## Ideální a reálné zdroje

**Ideální zdroj napětí** drží na svorkách napětí $U$ bez ohledu na odebíraný proud. Jeho jediný parametr je $U$ a teoreticky by dodal libovolně velký proud.

**Ideální zdroj proudu** dodává proud $I$ bez ohledu na napětí na svorkách. Jediný parametr je $I$.

Skutečné zdroje se modelují jako ideální zdroj plus vnitřní odpor:

![Reálný zdroj napětí](images/realny-zdroj.svg "Model reálného zdroje napětí (vlevo) a jeho zatěžovací charakteristika: přímka od napětí naprázdno U0 po proud nakrátko Ik")

**Reálný zdroj napětí:** ideální zdroj $U_\text{i}$ v sérii s vnitřním odporem $R_\text{i}$.

$$U = U_\text{i} - R_\text{i}\,I, \qquad U_0 = U_\text{i}\ (\text{naprázdno},\ I = 0), \qquad I_\text{k} = \frac{U_\text{i}}{R_\text{i}}\ (\text{nakrátko},\ U = 0)$$

**Reálný zdroj proudu:** ideální zdroj $I_\text{i}$ s paralelní vnitřní vodivostí $G_\text{i}$.

$$I = I_\text{i} - G_\text{i}\,U, \qquad U_0 = \frac{I_\text{i}}{G_\text{i}}, \qquad I_\text{k} = I_\text{i}$$

> [!key] Převod zdrojů (budeš ho potřebovat v MSP a MUN)
> Reálný zdroj napětí a reálný zdroj proudu jsou navenek stejné, když
> $$I_\text{i} = \frac{U_\text{i}}{R_\text{i}}, \qquad G_\text{i} = \frac{1}{R_\text{i}}$$
> Šipka proudu zdroje proudu míří ke svorce, která byla u zdroje napětí kladná.

Nejčastější reálné zdroje: **rotační generátory** (Faradayův indukční zákon; alternátor dává střídavé napětí, dynamo stejnosměrné) a **chemické články** (primární články nejdou nabíjet, sekundární, tedy akumulátory, ano).

> [!example] Monočlánek R14
> Naprázdno má článek $U_0$ = 1,5 V, při odběru 0,5 A klesne napětí na 1,3 V. Jaké jsou prvky náhradního schématu?
>
> Naprázdno neteče proud, takže $U_\text{i} = U_0 = 1{,}5$ V. Při zatížení se na vnitřním odporu ztratí
> $$\Delta U = 1{,}5 - 1{,}3 = 0{,}2\ \text{V}, \qquad R_\text{i} = \frac{\Delta U}{I} = \frac{0{,}2}{0{,}5} = 0{,}4\ \Omega$$
> Proud nakrátko by byl $I_\text{k} = 1{,}5/0{,}4 = 3{,}75$ A.
>
> **Jak to říct, když se zeptají:** „Napětí naprázdno je přímo vnitřní napětí, protože na $R_\text{i}$ bez proudu nic neubude. Pokles napětí při zatížení je úbytek na vnitřním odporu, takže $R_\text{i}$ je ten pokles dělený proudem.“

## Spojování zdrojů a rezistorů

- **Zdroje napětí do série:** napětí se sčítají (se znaménky podle šipek), $U = U_1 + U_2 + \dots$
- **Zdroje proudu paralelně:** proudy se sčítají, $I = I_1 + I_2 + \dots$
- **Rezistory v sérii:** $R = R_1 + R_2 + \dots + R_n$ (teče jimi stejný proud).
- **Rezistory paralelně:** sčítají se vodivosti, $G = G_1 + G_2 + \dots$, a $R = 1/G$. Zkráceně $R = R_1 \parallel R_2$. Pro dva rezistory:

$$R_1 \parallel R_2 = \frac{R_1 R_2}{R_1 + R_2}$$

Paralelní kombinace je vždy **menší** než nejmenší z rezistorů.

## Dělič napětí

![Dělič napětí](images/delic-napeti.svg "Rezistory R1 a R2 v sérii na zdroji U; napětí U1 a U2")

Dva rezistory v sérii na zdroji $U$. Proud $I = U/(R_1 + R_2)$, takže

$$U_1 = U\,\frac{R_1}{R_1 + R_2}, \qquad U_2 = U\,\frac{R_2}{R_1 + R_2}$$

Obecně pro více rezistorů v sérii: napětí na rezistoru je napětí zdroje krát (jeho odpor / celkový odpor), $U_k = U\,R_k/R$.

## Dělič proudu

![Dělič proudu](images/delic-proudu.svg "Rezistory R1 a R2 paralelně na zdroji proudu I; proudy I1 a I2")

Dva rezistory paralelně na zdroji proudu $I$. Napětí $U = I/(G_1 + G_2)$, takže

$$I_1 = I\,\frac{G_1}{G_1 + G_2} = I\,\frac{R_2}{R_1 + R_2}, \qquad I_2 = I\,\frac{G_2}{G_1 + G_2} = I\,\frac{R_1}{R_1 + R_2}$$

Obecně: proud rezistorem je proud zdroje krát (jeho vodivost / celková vodivost), $I_k = I\,G_k/G$. Vztahy pro dělič napětí a proudu jsou si formálně podobné, jen místo odporů jsou vodivosti.

> [!trap] Prohozené odpory v děliči proudu
> V $I_1 = I\,R_2/(R_1 + R_2)$ je v čitateli **druhý** odpor. Větší proud teče menším odporem. Kdo si to plete, ať počítá s vodivostmi: $I_1 = I\,G_1/(G_1+G_2)$, tam je v čitateli „vlastní“ vodivost.

> [!example] Hodnoty úlohy LC1
> **Dělič napětí:** $U$ = 10 V, $R_1$ = 220 Ω, $R_2$ = 100 Ω.
> $$I = \frac{10}{220 + 100} = \frac{10}{320} = 31{,}25\ \text{mA}, \quad U_1 = 10\cdot\frac{220}{320} = 6{,}875\ \text{V}, \quad U_2 = 10\cdot\frac{100}{320} = 3{,}125\ \text{V}$$
> Kontrola 2. KZ: $6{,}875 + 3{,}125 = 10$ V.
>
> **Dělič proudu:** $I$ = 100 mA, stejné rezistory.
> $$I_1 = 100\cdot\frac{100}{220 + 100} = 31{,}25\ \text{mA}, \qquad I_2 = 100\cdot\frac{220}{320} = 68{,}75\ \text{mA}$$
> Kontrola 1. KZ: $31{,}25 + 68{,}75 = 100$ mA.
>
> **Jak to říct, když se zeptají:** „U děliče napětí se napětí dělí v poměru odporů, na větším odporu je větší napětí. U děliče proudu se proud dělí v poměru vodivostí, víc proudu teče menším odporem.“

### Zatížený dělič

Když se k výstupu děliče připojí zátěž $R_\text{z}$, je paralelně k $R_2$ a výstupní napětí klesne:

$$U_2 = U\,\frac{R_2 \parallel R_\text{z}}{R_1 + R_2 \parallel R_\text{z}}$$

> [!example] O kolik klesne napětí děliče 1 : 2
> $R_1 = R_2$ = 200 Ω, zátěž 1,5 kΩ paralelně k $R_2$.
> $$R_2 \parallel R_\text{z} = \frac{200\cdot1500}{1700} = 176{,}47\ \Omega, \qquad \frac{U_2}{U} = \frac{176{,}47}{200 + 176{,}47} = 0{,}46875$$
> Naprázdno je poměr 0,5, takže výstupní napětí klesne o $(0{,}5 - 0{,}46875)/0{,}5 = 6{,}25\ \%$.

## Měření v obvodu

- **Voltmetr** měří rozdíl potenciálů dvou uzlů a připojuje se **paralelně** k prvku. Měl by mít co největší vnitřní odpor.
- **Ampérmetr** měří proud větví a zapojuje se **do série**. Měl by mít co nejmenší vnitřní odpor. Proud jde měřit i bez rozpojení obvodu klešťovým ampérmetrem.
- **Ohmmetr** měří odpor nepřímo (pošle známý proud a měří napětí, $R = U/I$). Měřený rezistor **nesmí být zapojený v obvodu**.
- U stejnosměrných přístrojů záleží na polaritě. Číslicové multimetry ukazují hodnotu přímo i s násobkem a často samy přepínají rozsah.
