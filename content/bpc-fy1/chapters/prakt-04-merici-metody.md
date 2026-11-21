+++
title = "Úvod do měření: měřicí metody"
topics = ["Měřicí metody"]
+++

**Měřicí metoda** je postup, kterým se získá číselná hodnota veličiny. Záleží na povaze veličiny, na vztazích, ze kterých se vychází, a na dostupných přístrojích. Většinu veličin jde měřit několika metodami a o volbě obvykle rozhoduje požadovaná přesnost. Kapitola shrnuje kapitolu 2 skripta *Úvod do měření*.

## Dvojice metod {#dvojice}

| metody | princip | příklad |
|---|---|---|
| **přímá** | veličina se porovná přímo s jednotkou nebo se přímo odečte na přístroji | délka pravítkem, čas stopkami, napětí voltmetrem |
| **nepřímá** | hodnota se vypočte z jiných, přímo měřených veličin | hustota z hmotnosti a rozměrů, odpor z $U$ a $I$ |
| **absolutní** | výsledek vyjde přímo v jednotce veličiny | čas v sekundách, proud v ampérech |
| **relativní** | porovnání s veličinou stejného druhu o známé hodnotě | hustota oleje porovnáním s vodou; každé srovnání s etalonem |
| **statická** | měřená ani ostatní veličiny se v čase nemění | odečet výchylky ustáleného přístroje |
| **dynamická** | měřená veličina nebo veličiny, na kterých závisí, se mění, obvykle periodicky | doba kmitu kyvadla |
| **substituční** | neznámá hodnota se postupně nahrazuje řadou známých hodnot (normálů) | výběr závaží, dekáda odporů |
| **kompenzační** | měřená veličina se vyrovná stejně velkou hodnotou téže veličiny | vážení na rovnoramenných vahách, můstek |

- I dynamická měření se dělají v **ustáleném stavu**, kdy se podmínky dost dlouho nemění: amplituda kyvadla nemá klesat, napětí baterie se nemá vybíjením měnit.
- Velmi pomalé změny se dají snadno přehlédnout (měření **kvazistatické**). Je to jeden z nejčastějších zdrojů chyb.
- **Kompenzační metoda je obvykle přesnější než substituční**: vyrovnání proběhne v jednom okamžiku, kdežto při substituci se normál hledá postupně a podmínky se mezitím mohou změnit.
- Kompenzační metoda bývá **nulová**: v okamžiku vyrovnání ukazuje indikační přístroj nulu. U nulových metod je potřeba opakovaně kontrolovat nulovou polohu přístroje.

## Interpolace a extrapolace {#interpolace}

Závislost $y = f(x)$ změříme jen v konečném počtu bodů. Hodnotu $y_0$ pro $x_0$ **uvnitř** intervalu mezi dvěma změřenými body $x_1$, $x_2$ najde **interpolace**. Pro lineární závislost nebo krátký interval:

$$\frac{y_0 - y_1}{x_0 - x_1} = \frac{y_2 - y_1}{x_2 - x_1} \quad\Rightarrow\quad y_0 = y_1 + (x_0 - x_1)\,\frac{y_2 - y_1}{x_2 - x_1}$$

U nelineární závislosti je jednodušší interpolovat **graficky**: body proložíte křivkou, která odpovídá teorii, a $y_0$ odečtete na ose.

**Extrapolace** odhaduje hodnotu **mimo** změřený interval. Pro lineární závislost se počítá stejným vztahem, je ale potřeba mnohem větší opatrnost: mimo změřený interval se mohou projevit jevy, které se uvnitř neprojevily. Vodič změřený mezi 10 °C a 40 °C se může při 60 °C roztavit, a extrapolace jeho odporu do 100 °C by pak neměla smysl.

![Interpolace a extrapolace](images/prakt-interpolace.svg "Interpolace mezi dvěma body a extrapolace za okraj měřeného intervalu")

> [!key] Extrapolovat nejvýš o 20 % délky intervalu
> Pro interval 10 °C až 40 °C (délka 30 °C) je to nejvýš 6 °C za každý okraj, tedy od 4 °C do 46 °C.

> [!example] Interpolace v tabulce termočlánku
> Napětí termočlánku je při 40 °C $1{,}61\ \mathrm{mV}$ a při 50 °C $2{,}02\ \mathrm{mV}$. Jaké je při 47 °C?
> $$U_0 = 1{,}61\ \mathrm{mV} + (47 - 40)\ {}^\circ\mathrm{C}\cdot\frac{(2{,}02 - 1{,}61)\ \mathrm{mV}}{(50 - 40)\ {}^\circ\mathrm{C}} = 1{,}61 + 7\cdot0{,}041 = 1{,}897\ \mathrm{mV} \doteq 1{,}90\ \mathrm{mV}$$
>
> **Jak to říct, když se zeptají:** „Mezi dvěma sousedními body tabulky beru závislost jako přímku. Na jeden stupeň připadá $0{,}041\ \mathrm{mV}$ a od 40 °C jsem o 7 stupňů dál. Výsledek nechávám na setiny milivoltu jako hodnoty v tabulce.“

## Postupná metoda {#postupna}

Obvykle se opakovaná měření dělají nezávisle na sobě. U **opakujících se dějů** je výhodnější a rychlejší, když konec jednoho měření je zároveň začátkem dalšího. Zvětší-li chybný odečet jeden úsek, zmenší tím sousední, takže se chyby částečně vyruší.

Postup pro dobu kmitu: čítač (nebo stopky) se spustí a **nezastavuje**, čas se zapisuje po každých 10 kmitech až do 90. Deset časů se rozdělí do dvou sloupců:

- sloupec **A**: 0, 10, 20, 30, 40 kmitů,
- sloupec **B**: 50, 60, 70, 80, 90 kmitů,
- rozdíly **B − A** jsou **5 hodnot doby 50 kmitů**, které se zpracují jako opakované měření s $n = 5$.

Měřili jste jen 90 kmitů, ale dostanete stejně přesný výsledek, jako kdybyste pětkrát měřili 50 kmitů, tedy 250 kmitů. Vyplatí se to hlavně u tlumených dějů, které by se zastavily dřív, než by se stihlo naměřit dost hodnot nezávisle.

> [!example] Doba kmitu kyvadla postupnou metodou
> | kmity | A / s | kmity | B / s | $50T = B - A$ / s |
> |---|---|---|---|---|
> | 0 | 0,00 | 50 | 80,21 | 80,21 |
> | 10 | 16,04 | 60 | 96,30 | 80,26 |
> | 20 | 32,11 | 70 | 112,29 | 80,18 |
> | 30 | 48,12 | 80 | 128,37 | 80,25 |
> | 40 | 64,20 | 90 | 144,40 | 80,20 |
>
> Průměr: $\overline{50T} = 401{,}10/5 = 80{,}22\ \mathrm{s}$. Odchylky $-0{,}01$; $0{,}04$; $-0{,}04$; $0{,}03$; $-0{,}02$ s, součet čtverců $0{,}0046\ \mathrm{s^2}$.
> $$s_{\bar x} = \sqrt{\frac{0{,}0046}{5\cdot4}} = 0{,}0152\ \mathrm{s} \qquad \delta(50T) = 2{,}968\cdot0{,}0152 = 0{,}045\ \mathrm{s} \to 0{,}05\ \mathrm{s}$$
> $50T = (80{,}22 \pm 0{,}05)\ \mathrm{s}$. Pro jeden kmit se hodnota i chyba vydělí padesáti:
> $$T = \frac{80{,}22}{50}\ \mathrm{s} = 1{,}6044\ \mathrm{s} \qquad \delta(T) = \frac{0{,}05}{50}\ \mathrm{s} = 0{,}001\ \mathrm{s}$$
> Výsledek: $T = (1{,}604 \pm 0{,}001)\ \mathrm{s}$, $n = 5$, $P = 95\ \%$.
>
> **Jak to říct, když se zeptají:** „Čítač jsem nezastavoval a časy jsem rozdělil do dvou sloupců. Rozdíly B − A jsou pět nezávislých hodnot doby padesáti kmitů, takže v tabulce beru $t$ pro $n = 5$, tedy $2{,}968$. Doba kmitu je padesátina, a proto je padesátkrát menší i její chyba.“

## Měření násobku {#nasobek}

Nejsou-li nároky na přesnost velké, stačí změřit **$n$-násobek** opakující se veličiny jednou a chybu odhadnout z přístroje:

$$X = \frac{X_n}{n} \qquad \delta(X) = \frac{\delta(X_n)}{n}$$

> [!example] Doba kmitu ručními stopkami
> Dvacet kmitů trvalo $31{,}4\ \mathrm{s}$, chyba ručních stopek je $0{,}3\ \mathrm{s}$ na odečet.
> $$T = \frac{31{,}4}{20}\ \mathrm{s} = 1{,}570\ \mathrm{s} \qquad \delta(T) = \frac{0{,}3}{20}\ \mathrm{s} = 0{,}015\ \mathrm{s}$$
> $T = (1{,}570 \pm 0{,}015)\ \mathrm{s}$. Chyba začíná jedničkou, proto má dvě platné číslice.
>
> **Jak to říct, když se zeptají:** „Chyba stopek se týká celého změřeného času, ne jednoho kmitu. Když změřím dvacet kmitů najednou, rozdělí se na dvacet dílů a doba jednoho kmitu je dvacetkrát přesnější.“
