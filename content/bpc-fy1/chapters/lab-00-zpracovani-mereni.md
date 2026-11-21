+++
title = "Praktikum: zpracování měření a chyby"
topics = ["Zpracování měření"]
+++

Ve fyzikálním praktiku se všechny úlohy zpracovávají stejným postupem podle skripta *Úvod do měření*. Tahle kapitola je kuchařka: průměr, chyba, zaokrouhlení, šíření chyb a porovnání s tabulkovou hodnotou. Podrobný výklad s řešenými příklady je v kapitolách *Úvod do měření* (veličiny a jednotky, měřicí metody, chyby měření, vyhodnocení závislostí, přístroje a zdroje) a pravidla praktika v kapitolách *Praktikum* (jak ho projít, bezpečnost, protokol).

## Opakované měření jedné veličiny {#opakovane}

Veličinu změříme $n$krát (obvykle 5 až 20krát) a z hodnot $x_1, \dots, x_n$ spočítáme:

$$\bar{x} = \frac{1}{n}\sum_{i=1}^{n} x_i \qquad s_{\bar{x}} = \sqrt{\frac{\sum (x_i - \bar{x})^2}{n\,(n-1)}}$$

$s_{\bar{x}}$ je **výběrová směrodatná odchylka aritmetického průměru**. Na kalkulačce: statistický režim dá $s = \sigma_{n-1}$, pak $s_{\bar{x}} = s/\sqrt{n}$.

**Chyba výsledku** (náhodná) se dostane vynásobením koeficientem Studentova rozdělení $t_{n,P}$ a relativní chyba je její podíl s průměrem:

$$\delta(X) = t_{n,P}\cdot s_{\bar{x}} \qquad \delta_\mathrm{r}(X) = \frac{\delta(X)}{\bar{x}}\cdot 100\ \%$$

V praktiku se bere $P = 95\ \%$. Koeficienty z tabulky v Úvodu:

| $n$ | 3 | 5 | 10 | 15 | 20 |
|---|---|---|---|---|---|
| $t_{n;\,0{,}95}$ | 4,526 | 2,968 | 2,320 | 2,145 | 2,093 |
| $t_{n;\,0{,}68}$ | 1,321 | 1,110 | 1,059 | 1,037 | 1,027 |

- Rozumný počet měření je **10 až 20**. S dalšími měřeními chyba klesá už jen pomalu.
- $s$ (na kalkulačce $\sigma_{n-1}$) popisuje rozptyl jednotlivých hodnot, do chyby jde $s_{\bar x} = s/\sqrt n$.
- Excel (`T.INV.2T`) dává pro $n = 3$, $5$ a $10$ jiné koeficienty než tabulka Úvodu (pro $n = 5$ je to $2{,}776$). Počítejte s tabulkou Úvodu a v protokolu uveďte, jaké $t$ jste použili.

> [!key] Zápis výsledku
> $X = (\bar{x} \pm \delta)\ \text{jednotka}$, k tomu počet měření $n$, pravděpodobnost $P$ a relativní chyba. Například $d = (6{,}84 \pm 0{,}02)\ \mathrm{m}$, $n = 10$, $P = 95\ \%$, $\delta_\mathrm{r} = 0{,}3\ \%$.

## Zaokrouhlování {#zaokrouhlovani}

1. **Chyba** se zaokrouhluje **vždy nahoru na 1 platnou číslici**: $0{,}0342 \to 0{,}04$.
2. Začíná-li chyba jedničkou, nechají se **2 číslice** (jinak by se skoro zdvojnásobila): $0{,}0137 \to 0{,}014$.
3. **Výsledek** se zaokrouhlí na **stejný řád jako chyba**: $(6{,}84 \pm 0{,}02)\ \mathrm{m}$ je správně, $(6{,}843 \pm 0{,}02)\ \mathrm{m}$ a $(6{,}8 \pm 0{,}018)\ \mathrm{m}$ špatně.
4. Velká čísla přes mocninu deseti: $U = 1{,}40\cdot10^{4}\ \mathrm{V}$, ne $14000\ \mathrm{V}$, když jsou platné jen 3 číslice.
5. Hodnota zapsaná **bez chyby** má chybu jedné jednotky posledního řádu: $1{,}2\ \mathrm{m}$ je $\pm 0{,}1\ \mathrm{m}$, $1{,}200\ \mathrm{m}$ je $\pm 0{,}001\ \mathrm{m}$. Nuly vpravo tedy něco znamenají.
6. Mezivýsledky bez chyby: při **sčítání** se zaokrouhluje na poslední řád platný u všech sčítanců ($12{,}4 + 3{,}27 = 15{,}67 \to 15{,}7$), při **násobení** má výsledek tolik platných číslic jako nejméně přesný činitel ($3{,}142\cdot2{,}7 \to 8{,}5$).

> [!trap] Rozhoduje první číslice před zaokrouhlením
> $0{,}0904$ se zaokrouhlí na $0{,}1$, ne na $0{,}10$: první platná číslice je 9, takže stačí jedna. Naopak $0{,}01023 \to 0{,}011$.

## Chyba měřidla {#meridlo}

Kromě náhodné chyby má každé měření **chybu měřidla** $u(X)$ (soustavnou):

- **stupnice** (pravítko, ocelové měřítko): nejmenší dílek, dohodou jeho polovina,
- **ručkový přístroj**: z **třídy přesnosti** $T_\mathrm{p}$, $\ u = \dfrac{T_\mathrm{p}}{100}\cdot X_\mathrm{rozsah}$ (stejná na celém rozsahu),
- **digitální displej**: rozlišení (jednotka posledního místa), přesněji údaj výrobce „% z hodnoty + % z rozsahu“ nebo „% z hodnoty + počet digitů“,
- **ruční stopky**: $0{,}3\ \mathrm{s}$ na jeden odečet; měří-li se $n$ kmitů najednou, je chyba doby kmitu $n$krát menší,
- **váhy**: rozdíl nulových poloh před vážením a po něm dělený citlivostí vah.

Relativní chyba měřidla $u_\mathrm{r} = u/x$. **Porovná se s náhodnou chybou a dál se počítá s větší z nich**, menší se zanedbá.

U **jednoho měření** (v praktiku to znamená jedno měření a jedno kontrolní, kvůli hrubé chybě) se chyba určí jen z měřidla.

> [!note] Proč ručka do poslední třetiny stupnice
> Absolutní chyba z třídy přesnosti je stejná na celém rozsahu. Relativní chyba je proto nejmenší, když je výchylka velká. Přístroj třídy 1,5 na rozsahu 60 V má chybu 0,9 V: při 45 V je to 2 %, při 20 V už 4,5 %.

## Šíření chyb (nepřímé měření) {#sireni}

Veličina $V = f(X, Y, \dots)$ se počítá z průměrů. Úplný zákon šíření chyb sčítá příspěvky kvadraticky:

$$\delta(V) = \sqrt{\left(\frac{\partial f}{\partial x}\,\delta(X)\right)^2 + \left(\frac{\partial f}{\partial y}\,\delta(Y)\right)^2 + \dots}$$

Vzorové příklady Úvodu používají zjednodušený tvar (sčítají se absolutní hodnoty příspěvků), který dá o něco větší, bezpečnější chybu:

$$\delta(V) = \left|\frac{\partial f}{\partial x}\right|\delta(X) + \left|\frac{\partial f}{\partial y}\right|\delta(Y) + \dots$$

Pravděpodobnost výsledku je nejmenší z pravděpodobností vstupních veličin.

Nejčastější případy:

| vztah | chyba |
|---|---|
| $V = aX \pm bY$ | $\delta(V) = \lvert a\rvert\,\delta(X) + \lvert b\rvert\,\delta(Y)$ (sčítají se **absolutní** chyby) |
| $V = a X^k Y^m$ nebo $a X^k/Y^m$ | $\delta_\mathrm{r}(V) = \lvert k\rvert\,\delta_\mathrm{r}(X) + \lvert m\rvert\,\delta_\mathrm{r}(Y)$ (sčítají se **relativní** chyby krát exponent) |

> [!example] Tíhové zrychlení z kyvadla
> Pro $g = 4\pi^2\ell/T^2$ je $\delta_\mathrm{r}(g) = \delta_\mathrm{r}(\ell) + 2\,\delta_\mathrm{r}(T)$. Doba kmitu je ve druhé mocnině, proto ji je potřeba měřit dvakrát přesněji než délku.

> [!trap]
> U rozdílu dvou blízkých hodnot se sčítají absolutní chyby, a relativní chyba výsledku proto může být obrovská. Malá relativní chyba vstupů nic nezaručuje.

## Porovnání s tabulkovou hodnotou {#porovnani}

Liší-li se výsledek od tabulkové hodnoty **o víc než $3\delta$**, je měření podle Úvodu nejspíš zatížené **soustavnou chybou** (nevhodná metoda, zanedbaný vliv) nebo hrubou chybou. Do $3\delta$ se výsledek s tabulkovou hodnotou shoduje.

V závěru protokolu se pak uvádí výsledek s chybou a $(n, P)$, porovnání, možná příčina (jako domněnka) a návrh, jak měření zlepšit.

## Postupná metoda {#postupna}

U periodického děje se čas zapisuje průběžně, třeba po každých 10 kmitech až do 90, a čítač se nezastavuje. Časy se rozdělí do dvou sloupců: A (0, 10, 20, 30, 40 kmitů) a B (50, 60, 70, 80, 90 kmitů). Rozdíly $B - A$ dají **5 hodnot doby 50 kmitů**, které se zpracují jako opakované měření ($n = 5$, $t = 2{,}968$).

$$T = \frac{\overline{50T}}{50} \qquad \delta(T) = \frac{\delta(50T)}{50}$$

Výhoda: za 90 kmitů dostaneme přesnost, jako bychom 5krát měřili 50 kmitů. Chyby jednotlivých odečtů se navíc částečně vyruší, protože konec jednoho úseku je začátkem dalšího.

## Lineární závislost a metoda nejmenších čtverců {#mnc}

Když má platit $y = a + bx$, proloží se body přímka **metodou nejmenších čtverců** (tabulkový procesor, kalkulačka). Směrnice:

$$b = \frac{\sum (x_i - \bar{x})(y_i - \bar{y})}{\sum (x_i - \bar{x})^2} \qquad a = \bar{y} - b\,\bar{x}$$

- Nelineární závislost se často převede na přímku: $y = A\,\mathrm{e}^{bx}$ logaritmováním ($\ln y$ proti $x$), mocninná $y = \alpha x^n$ v log–log grafu ($\ln y$ proti $\ln x$, směrnice je $n$).
- Směrnice z grafu se počítá z hodnot na osách $b = (y_2 - y_1)/(x_2 - x_1)$, **ne** jako tangens úhlu na papíře (osy mají různá měřítka).
- Hrubé chyby se vyřadí ještě před fitem, kvadrát odchylky by přímku stáhl k sobě.
- Graf: popsané osy „veličina / jednotka“, body jako značky, proložená hladká křivka (ne lomená čára), legenda a v popisku metoda fitu.

> [!trap] Extrapolace
> Mimo změřený interval může začít platit jiná fyzika. Úvod doporučuje neextrapolovat dál než o zhruba 20 % délky intervalu.
