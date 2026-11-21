+++
title = "Úvod do měření: chyby měření"
topics = ["Chyby měření"]
+++

Žádné měření není přesné. Výsledek, u kterého chybí jeho přesnost, nemá smysl, protože se nedá porovnat s jiným výsledkem ani s tabulkovou hodnotou. Proto je v každém protokolu potřeba rozebrat chyby. Kapitola vychází z kapitol 3 a 6 skripta *Úvod do měření*. Stručný tahák k úlohám je v kapitole **Praktikum: zpracování měření a chyby**.

> [!note] Chyby, nebo nejistoty?
> V jiných předmětech potkáte **standardní nejistoty typu A a B** podle metodiky GUM. Úvod do měření pro praktikum FY1 tuhle terminologii nepoužívá. Počítá **chybu z opakovaného měření** se Studentovým koeficientem a **mezní chybu měřidla** a obě porovná. Odpovídá to zhruba dělení na typ A (statistika z opakování) a typ B (údaje o přístroji), vzorce a způsob kombinace jsou ale jiné. V protokolu z FY1 postupujte podle Úvodu.

## Pravá hodnota a přesnost {#prava-hodnota}

- **Přesnost měření** říká, jak blízko je výsledek skutečné (pravé) hodnotě.
- Pravá hodnota je ideál. Nahrazuje se **konvenčně pravou hodnotou**, která se od skutečné liší zanedbatelně vzhledem k tomu, jak přesně ji zrovna potřebujeme. Planckova konstanta může být podle potřeby $6{,}6\cdot10^{-34}\ \mathrm{J\,s}$ i $6{,}626\cdot10^{-34}\ \mathrm{J\,s}$.
- Při opakovaném měření (po opravě soustavných chyb) se za pravou hodnotu bere **aritmetický průměr**.

## Druhy chyb {#druhy}

| hledisko | druhy |
|---|---|
| původ | osobní (pozorovatel), chyby přístrojů, chyby metody |
| charakter | **náhodné** (při opakování kolísají) a **soustavné** (při stejných podmínkách stejné) |
| vyjádření | **absolutní** (v jednotkách veličiny), **relativní** (podíl s naměřenou hodnotou, bezrozměrná, obvykle v %) |
| velikost | **krajní (mezní)**: největší chyba, která může za daných podmínek nastat; **hrubá (nadměrná)**: větší než mezní, svědčí o omylu nebo poruše |

Relativní chyba umožní porovnat přesnost měření úplně různých veličin (délky s časem). Druhy se překrývají: když se podmínky nepozorovaně mění, soustavná chyba se mění také a snadno se splete s náhodnou.

## Hrubé chyby {#hrube}

Hodnota zatížená hrubou chybou se od ostatních nápadně liší. Vzniká nepozorností nebo únavou (přečtete 13 místo 18), při špatné viditelnosti nebo nevhodnou metodou či přístrojem. Příklad: voltmetr s usměrňovačem je ocejchovaný v efektivních hodnotách jen pro harmonické napětí, na jiném průběhu ukazuje špatně.

> [!key] Hrubé chyby se ze zpracování vyřazují
> Hodnotu s hrubou chybou vyřaďte **dřív**, než začnete počítat průměr nebo prokládat přímku. Jinak výsledek zkreslí. U normálního rozdělení se za hrubou chybu obvykle považuje odchylka větší než $3\sigma$ (pravidlo tří sigma, viz níže).

## Soustavné chyby {#soustavne}

Soustavná (systematická) chyba se při opakování za stejných podmínek nemění. Proto ji opakováním neodhalíte ani nezmenšíte. Pečlivým rozborem se ale dá najít, odhadnout její velikost a znaménko a výsledek opravit. Zdroje:

- **metoda**: voltmetr má konečný vnitřní odpor, proto ukáže o něco menší napětí. Opraví se výpočtem (korekce), nebo se použije kompenzační metoda. Vzorec pro dobu kmitu kyvadla platí pro nulový rozkmit, s ním vychází $g$ vždy o něco menší.
- **přístroj**: nerovnoměrná stupnice (ověří se cejchováním), špatně nastavená nula nebo citlivost, elektronický přístroj měřící hned po zapnutí (musí se ustálit).
- **osoba**: pozdě spuštěné stopky, odečet stupnice zešikma (**paralaxa**), hrubý odhad desetin dílku. Nejlépe je odstraní automatizace měření.
- **měřený objekt**: materiál se dlouhým používáním mění (únava), naměřený modul pak může vyjít menší než tabulkový. Takový rozdíl se v závěru dá zdůvodnit.

Soustavné chyby se částečně potlačí, když se veličina změří **několika různými metodami**. Jednoduchý test, zda opakovaná měření skrývají soustavnou složku: sledujte **znaménka odchylek** od průměru v pořadí měření. Střídají-li se nepravidelně (− − + − + + − +), jsou náhodné. Dlouhé úseky stejných znamének (+ + + + + − − − − −) prozrazují soustavnou složku, která se během měření měnila.

> [!trap] Zjištěnou soustavnou chybu je nutné opravit
> Když soustavnou chybu znáte, nestačí ji zmínit. Do zpracování jdou **opravené** hodnoty.

## Chyba měřidla {#meridlo}

Když soustavné chyby nejdou zjistit a opravit, vymezí se aspoň interval, ve kterém chyba jednoho měření leží. Výsledek jednoho měření se pak zapíše:

$$X = x_\mathrm{N} \pm u(X) \qquad \delta_\mathrm{r}(X) = \frac{u(X)}{x_\mathrm{N}}$$

kde $x_\mathrm{N}$ je naměřená hodnota a $u(X)$ **mezní chyba měřidla**. Jak ji určit:

| měřidlo | mezní chyba $u$ |
|---|---|
| ručkový přístroj | z **třídy přesnosti**: $u = \dfrac{T_\mathrm{p}}{100}\cdot X_\text{rozsah}$, stejná pro celý rozsah |
| číslicový přístroj | součet dvou částí: % z naměřené hodnoty + % z plného rozsahu (nebo počet digitů) |
| stupnice (pravítko, měřítko) | nejmenší dílek, dohodou také jeho polovina |
| posuvné měřítko | podle nonia, např. $0{,}05\ \mathrm{mm}$ |
| stopky (ruční) | odhadem $0{,}3\ \mathrm{s}$ na jeden odečet (přístroj i reakce člověka) |
| váhy | rozdíl nulových poloh před vážením a po něm dělený citlivostí vah |
| součástka (odpor, kondenzátor) | výrobní tolerance |

Podrobnosti o třídě přesnosti a číslicových přístrojích jsou v kapitole **Úvod do měření: přístroje a zdroje**. U většiny měření se uplatní víc chyb najednou, třeba chyba odečtu ze stupnice a chyba z třídy přesnosti. Porovnáním zjistíte, kterou lze zanedbat.

## Náhodné chyby a Gaussovo rozdělení {#nahodne}

I když se měřená veličina nemění, opakovaná měření s dostatečnou rozlišovací schopností dávají různé výsledky. Působí řada drobných nepostižitelných vlivů, které se náhodně skládají. Náhodným chybám se nevyhnete a jsou tím nápadnější, čím přesněji měříte.

Při velmi mnoha měřeních se hodnoty rozloží kolem **střední hodnoty** $\mu$ symetricky a malé odchylky jsou mnohem častější než velké. To popisuje **normální (Gaussovo) rozdělení**:

$$p(x) = \frac{1}{\sigma\sqrt{2\pi}}\,\mathrm{e}^{-\frac{(x-\mu)^2}{2\sigma^2}}$$

- $\sigma^2$ je **rozptyl**, průměrný čtverec odchylek od $\mu$; $\sigma$ je **směrodatná (standardní) odchylka**. Čím menší $\sigma$, tím užší a vyšší je křivka.
- Plocha pod křivkou mezi $x_1$ a $x_2$ je pravděpodobnost, že hodnota padne do tohoto intervalu. Celá plocha je 1 (100 %).

![Gaussova křivka](images/prakt-normalni-rozdeleni.svg "Pravděpodobnost výskytu v intervalech μ ± σ, μ ± 2σ a μ ± 3σ")

| interval | pravděpodobnost |
|---|---|
| $\mu \pm \sigma$ | 68,26 % |
| $\mu \pm 2\sigma$ | 95,44 % |
| $\mu \pm 3\sigma$ | 99,72 % |

**Pravidlo tří sigma:** za největší možnou náhodnou odchylku se bere $3\sigma$. Hodnota dál od střední hodnoty se vyřadí jako hrubá chyba.

## Z několika měření: průměr a směrodatné odchylky {#vyber}

Nekonečně mnoho měření udělat nejde. Z $n$ naměřených hodnot (náhodného výběru) se parametry rozdělení jen **odhadnou**:

$$\bar x = \frac{1}{n}\sum_{i=1}^{n} x_i \qquad s = \sqrt{\frac{\sum (x_i-\bar x)^2}{n-1}} \qquad s_{\bar x} = \frac{s}{\sqrt n} = \sqrt{\frac{\sum (x_i-\bar x)^2}{n\,(n-1)}}$$

- $\bar x$ je **aritmetický průměr** (odhad $\mu$),
- $s$ je **výběrová směrodatná odchylka jednoho měření** (na kalkulačce $\sigma_{n-1}$ nebo $s_x$), ve jmenovateli je $n-1$, ne $n$,
- $s_{\bar x}$ je **výběrová směrodatná odchylka aritmetického průměru**: jak moc by kolísal průměr, kdybychom celou sérii zopakovali.

> [!trap] Odchylka jednoho měření není odchylka průměru
> $s$ popisuje rozptyl jednotlivých hodnot a s počtem měření se skoro nemění. $s_{\bar x}$ klesá s $\sqrt n$. Do chyby výsledku jde $s_{\bar x}$. Když kalkulačka ukáže $\sigma_{n-1}$, musíte ji ještě vydělit $\sqrt n$.

## Chyba výsledku a Studentův koeficient {#student}

Průměr z malého počtu měření je jen odhad. Interval, ve kterém leží pravá hodnota s pravděpodobností $P$ (**interval spolehlivosti**), je

$$\left(\bar x - t_{n,P}\,\frac{s}{\sqrt n}\,;\ \bar x + t_{n,P}\,\frac{s}{\sqrt n}\right)$$

a jeho polovina je **chyba výsledku** z $n$ měření s pravděpodobností $P$:

$$\delta(X) = t_{n,P}\cdot s_{\bar x} \qquad \delta_\mathrm{r}(X) = \frac{\delta(X)}{\bar x}\cdot 100\ \%$$

$t_{n,P}$ je **koeficient Studentova rozdělení**. Tabulka z Úvodu:

| $n$ | $P = 0{,}50$ | $P = 0{,}68$ | $P = 0{,}95$ | $P = 0{,}99$ |
|---|---|---|---|---|
| 3 | 0,817 | 1,321 | 4,526 | 19,210 |
| 5 | 0,741 | 1,110 | 2,968 | 6,620 |
| 10 | 0,703 | 1,059 | 2,320 | 3,250 |
| 15 | 0,692 | 1,037 | 2,145 | 2,997 |
| 20 | 0,688 | 1,027 | 2,093 | 2,861 |

- V technické praxi i v praktiku se požaduje **$P = 95\ \%$**: riziko, že pravá hodnota leží mimo interval, je 5 %. Pro běžná měření se někdy uvádí $P = 68\ \%$.
- K výsledku se vždy píše **$P$ a počet měření $n$**, bez nich chyba nic neříká.
- S rostoucím $n$ se interval zužuje (klesá $t$ i $s_{\bar x}$), ale čím dál pomaleji. Rozumný počet měření je **10 až 20**, víc se nevyplatí a hůř se udrží stálé podmínky.

> [!trap] Koeficient berte z tabulky Úvodu
> Hodnoty pro $n = 3$, $5$ a $10$ se liší od běžných statistických tabulek: Excel (`T.INV.2T(0,05; n−1)`) dá pro $n = 5$ hodnotu $2{,}776$, Úvod $2{,}968$. Úvod říká vzít $t$ z jeho tabulky a tak počítají i úlohy v této aplikaci. Pozor, vzorový příklad v Úvodu sám použil pro $n = 10$ hodnotu $2{,}262$ (Excel), ne $2{,}320$ z tabulky. V protokolu vždy napište, jaké $t$ jste použili. Když návod k úloze nebo vyučující určí jinak, řiďte se jimi.

## Postup s kalkulačkou {#kalkulacka}

1. Přepněte kalkulačku do **statistického režimu** (SD, STAT).
2. Zadejte všechny hodnoty $x_i$ (klávesa DATA nebo M+).
3. Vyvolejte průměr $\bar x$.
4. Vyvolejte výběrovou směrodatnou odchylku $\sigma_{n-1}$ (ne $\sigma_n$!).
5. Vydělte ji $\sqrt n$: $s_{\bar x} = \sigma_{n-1}/\sqrt n$.
6. Z tabulky vezměte $t_{n,P}$ a spočtěte $\delta(X) = t_{n,P}\cdot s_{\bar x}$. Zaokrouhlete ji (viz níže).
7. Zapište $X = (\bar x \pm \delta)$ jednotka, $n$, $P = 95\ \%$, $\delta_\mathrm{r}$. Průměr zaokrouhlete na stejný řád jako chybu.

Kalkulačka počítá $\sigma_{n-1}$ obvykle z upraveného vzorce $\sigma_{n-1} = \sqrt{\dfrac{\sum x_i^2 - (\sum x_i)^2/n}{n-1}}$, který dá totéž co definice.

## Příklad: délka tyčky posuvkou {#priklad-delka}

> [!example] Deset měření posuvkou
> Délka tyčky změřená desetkrát posuvným měřítkem (nonius $0{,}05\ \mathrm{mm}$), v mm: 124,35; 124,40; 124,30; 124,45; 124,35; 124,40; 124,50; 124,35; 124,40; 124,30.
>
> **Průměr:** součet je $1243{,}80\ \mathrm{mm}$, tedy $\bar l = 1243{,}80/10 = 124{,}38\ \mathrm{mm}$.
>
> **Odchylky** $l_i - \bar l$ v mm: $-0{,}03$; $0{,}02$; $-0{,}08$; $0{,}07$; $-0{,}03$; $0{,}02$; $0{,}12$; $-0{,}03$; $0{,}02$; $-0{,}08$ (součet je 0, kontrola). Součet čtverců: $3\cdot0{,}0009 + 3\cdot0{,}0004 + 2\cdot0{,}0064 + 0{,}0049 + 0{,}0144 = 0{,}0360\ \mathrm{mm^2}$.
>
> $$s = \sqrt{\frac{0{,}0360}{9}} = 0{,}0632\ \mathrm{mm} \qquad s_{\bar l} = \frac{0{,}0632}{\sqrt{10}} = 0{,}0200\ \mathrm{mm}$$
> $$\delta(l) = t_{10;\,0{,}95}\cdot s_{\bar l} = 2{,}320\cdot0{,}0200 = 0{,}0464\ \mathrm{mm} \to 0{,}05\ \mathrm{mm}$$
>
> **Chyba měřidla:** posuvka $u(l) = 0{,}05\ \mathrm{mm}$, $u_\mathrm{r} = 0{,}05/124{,}38 \doteq 0{,}04\ \%$. Náhodná chyba a chyba měřidla vyšly stejně velké.
>
> **Výsledek:** $l = (124{,}38 \pm 0{,}05)\ \mathrm{mm}$, $n = 10$, $P = 95\ \%$, $\delta_\mathrm{r} \doteq 0{,}04\ \%$.
>
> **Jak to říct, když se zeptají:** „Průměr je odhad pravé hodnoty. Rozptyl hodnot popisuje směrodatná odchylka jednoho měření $s = 0{,}063\ \mathrm{mm}$, průměr je přesnější, $s_{\bar l} = s/\sqrt{10} = 0{,}020\ \mathrm{mm}$. Pro deset měření a 95 % je v tabulce Úvodu $t = 2{,}320$, chyba tedy vyšla $0{,}046\ \mathrm{mm}$ a zaokrouhlil jsem ji nahoru na jednu platnou číslici. Je stejná jako chyba posuvky, takže víc měření by výsledek nezpřesnilo.“

## Jedno měření {#jedno}

Opakovat každé měření nemá vždy smysl. Když převažují soustavné chyby nebo stačí přesnost jednoho měření, **neměřte deset hodnot jen pro formu** (a už vůbec je uměle nerozptylujte). Přesnost jednoho měření se odhadne z třídy přesnosti, tolerance součástky a podobně. „Jedno měření“ v praktiku znamená **jedno měření a jedno kontrolní**, aby šlo vyloučit hrubou chybu.

> [!example] Napětí odečtené jednou
> Voltmetr třídy 1,5 na rozsahu 30 V ukazuje 21,0 V.
> $$u(U) = \frac{1{,}5}{100}\cdot30\ \mathrm{V} = 0{,}45\ \mathrm{V} \to 0{,}5\ \mathrm{V} \qquad \delta_\mathrm{r} = \frac{0{,}45}{21{,}0} \doteq 2\ \%$$
> Výsledek: $U = (21{,}0 \pm 0{,}5)\ \mathrm{V}$.
>
> **Jak to říct, když se zeptají:** „Měřil jsem jednou, takže chybu beru z třídy přesnosti. Ta je 1,5 % z rozsahu, tedy 0,45 V na celém rozsahu 30 V. Relativně to při 21 V dělá asi 2 %.“

## Náhodná chyba, nebo chyba měřidla? {#porovnani}

U opakovaného měření se vždy uvádí i **relativní chyba měřidla** $u_\mathrm{r}$, aby šlo porovnat náhodné vlivy s přesností přístroje.

- Je-li jedna z chyb **výrazně menší**, zanedbá se a dál se počítá s převažující. V úlohách praktika se tedy pokračuje s **větší** z nich.
- Cílem je, aby náhodná chyba byla menší nebo srovnatelná s chybou měřidla. Je-li náhodná chyba mnohem větší, něco v měření kolísá (podmínky, postup) a stojí za to najít příčinu.

> [!note] Když jsou obě chyby srovnatelné
> Ve vzorovém příkladu Úvodu (rozměry desky) vyšla náhodná chyba $0{,}4\ \mathrm{mm}$ a chyba měřítka $0{,}5\ \mathrm{mm}$. Úvod je označí za srovnatelné a dál počítá s náhodnou chybou. Pokud si nejste jistí, co chce váš vyučující, zeptejte se. V každém případě uveďte v protokolu obě chyby.

## Nepřímé měření: zákon šíření chyb {#sireni}

Většina výsledků se počítá z jiných, přímo měřených veličin: $V = f(X, Y, \dots)$. Nejpravděpodobnější hodnota se získá dosazením průměrů (u jednou měřené veličiny její jediné hodnoty):

$$\bar v = f(\bar x, \bar y, \dots)$$

Chybu výsledku dává **zákon šíření chyb**:

$$\delta(V) = \sqrt{\left(\frac{\partial f}{\partial x}\,\delta(X)\right)^2 + \left(\frac{\partial f}{\partial y}\,\delta(Y)\right)^2 + \dots}$$

Většinou stačí **zjednodušený tvar** (sčítají se absolutní hodnoty příspěvků). Ten používají vzorové příklady Úvodu i úlohy praktika:

$$\delta(V) = \left\lvert\frac{\partial f}{\partial x}\right\rvert\delta(X) + \left\lvert\frac{\partial f}{\partial y}\right\rvert\delta(Y) + \dots$$

Zjednodušený tvar dává o něco **větší** (bezpečnější) chybu než kvadratický. Vstupní chyby nemusí být stejného druhu: jedna veličina může mít chybu z opakovaného měření, jiná chybu měřidla.

| vztah ($a$, $b$, $k$, $m$ konstanty) | chyba výsledku |
|---|---|
| $V = aX$ | $\delta(V) = \lvert a\rvert\,\delta(X)$ |
| $V = aX \pm bY$ | $\delta(V) = \lvert a\rvert\,\delta(X) + \lvert b\rvert\,\delta(Y)$ |
| $V = aX^k$ | $\delta_\mathrm{r}(V) = \lvert k\rvert\,\delta_\mathrm{r}(X)$ |
| $V = aX^kY^m$ | $\delta_\mathrm{r}(V) = \lvert k\rvert\,\delta_\mathrm{r}(X) + \lvert m\rvert\,\delta_\mathrm{r}(Y)$ |
| $V = X^k/Y^m$ | $\delta_\mathrm{r}(V) = \lvert k\rvert\,\delta_\mathrm{r}(X) + \lvert m\rvert\,\delta_\mathrm{r}(Y)$ |

Co z toho plyne:

- **Součet a rozdíl:** sčítají se **absolutní** chyby a o výsledku rozhoduje ta největší. Měřit jednu ze sčítaných veličin mnohem přesněji než ostatní nemá smysl.
- **Součin a podíl:** sčítají se **relativní** chyby vynásobené exponentem. Veličinu ve vyšší mocnině je třeba měřit přesněji.
- **Optimalizace měření:** podle **zásady stejného vlivu** se přístroje a metody volí tak, aby všechny členy součtu byly zhruba stejně velké. Obvyklá přesnost laboratorního měření je kolem 1 %.
- **Pravděpodobnost výsledku** je nejmenší z pravděpodobností, se kterými jsou určeny vstupní veličiny (řetěz je tak pevný jako jeho nejslabší článek).

> [!example] Hustota válečku
> Hustota se počítá ze vztahu $\rho = 4m/(\pi d^2 h)$. Naměřeno: $m = (52{,}4 \pm 0{,}1)\ \mathrm{g}$, průměr $d = (20{,}05 \pm 0{,}03)\ \mathrm{mm}$, výška $h = (21{,}10 \pm 0{,}05)\ \mathrm{mm}$.
> $$\rho = \frac{4\cdot52{,}4\cdot10^{-3}}{\pi\cdot(20{,}05\cdot10^{-3})^2\cdot21{,}10\cdot10^{-3}}\ \mathrm{kg\,m^{-3}} = 7866\ \mathrm{kg\,m^{-3}}$$
> Relativní chyby vstupů: $\delta_\mathrm{r}(m) = 0{,}1/52{,}4 = 0{,}191\ \%$, $\delta_\mathrm{r}(d) = 0{,}03/20{,}05 = 0{,}150\ \%$, $\delta_\mathrm{r}(h) = 0{,}05/21{,}10 = 0{,}237\ \%$. Průměr je ve druhé mocnině:
> $$\delta_\mathrm{r}(\rho) = \delta_\mathrm{r}(m) + 2\,\delta_\mathrm{r}(d) + \delta_\mathrm{r}(h) = 0{,}191 + 2\cdot0{,}150 + 0{,}237 = 0{,}728\ \%$$
> $$\delta(\rho) = 7866\cdot0{,}00728 = 57{,}3\ \mathrm{kg\,m^{-3}} \to 60\ \mathrm{kg\,m^{-3}}$$
> Výsledek: $\rho = (7{,}87 \pm 0{,}06)\cdot10^{3}\ \mathrm{kg\,m^{-3}}$, $\delta_\mathrm{r} \doteq 0{,}8\ \%$.
>
> **Jak to říct, když se zeptají:** „Hustota je součin a podíl mocnin, proto sčítám relativní chyby vynásobené exponenty. Průměr je ve druhé mocnině, jeho příspěvek $0{,}30\ \%$ je největší, i když samotný průměr jsem změřil relativně nejpřesněji. Chybu jsem zaokrouhlil nahoru na jednu platnou číslici a hustotu na stejný řád, tedy na desítky $\mathrm{kg\,m^{-3}}$.“

> [!example] Rozdíl dvou teplot
> $t_1 = (21{,}4 \pm 0{,}3)\ ^\circ\mathrm{C}$, $t_2 = (35{,}2 \pm 0{,}3)\ ^\circ\mathrm{C}$. Pro rozdíl se sčítají absolutní chyby:
> $$\Delta t = 35{,}2 - 21{,}4 = 13{,}8\ ^\circ\mathrm{C} \qquad \delta(\Delta t) = 0{,}3 + 0{,}3 = 0{,}6\ ^\circ\mathrm{C}$$
> Výsledek $\Delta t = (13{,}8 \pm 0{,}6)\ ^\circ\mathrm{C}$ má relativní chybu $0{,}6/13{,}8 \doteq 4{,}3\ \%$, přestože každá z teplot je změřená s relativní chybou do 1,4 %.
>
> **Jak to říct, když se zeptají:** „U rozdílu se sčítají absolutní chyby, kdežto výsledek je malý. Proto vyjde relativní chyba mnohem větší než u vstupních hodnot.“

> [!note] Kvadratický, nebo zjednodušený zákon?
> Pro $R = U/I$ s $\delta_\mathrm{r}(U) = 0{,}5\ \%$ a $\delta_\mathrm{r}(I) = 1{,}2\ \%$ dá zjednodušený tvar $0{,}5 + 1{,}2 = 1{,}7\ \%$, kvadratický $\sqrt{0{,}5^2 + 1{,}2^2} = 1{,}3\ \%$. V protokolu použijte ten, který chce návod k úloze. Bez pokynu je bezpečnější zjednodušený, jako ve vzorových příkladech Úvodu.

## Zaokrouhlování a zápis výsledku {#zapis}

Naměřené hodnoty jsou přibližná čísla. **Platné číslice** jsou všechny od první nenulové zleva po poslední zapsanou. Poslední z nich (obvykle odhadnutá desetina dílku) už je zatížená chybou. Platí i nuly vpravo: $1{,}200\ \mathrm{m}$ je jiný údaj než $1{,}2\ \mathrm{m}$.

- **Hodnota bez chyby** znamená chybu jedné jednotky posledního zapsaného řádu: $d = 1{,}2\ \mathrm{m}$ je $(1{,}2 \pm 0{,}1)\ \mathrm{m}$ (asi 10 %), $d = 1{,}200\ \mathrm{m}$ je $(1{,}200 \pm 0{,}001)\ \mathrm{m}$ (asi 0,1 %). Bez chyby se výsledek smí napsat, jen když stačí řádový odhad přesnosti.
- **Chyba** se zaokrouhluje **vždy nahoru na jednu platnou číslici**: $0{,}0567 \to 0{,}06$. Dvě číslice se nechají, když by jedna chybu neúměrně zvětšila, prakticky když chyba začíná jedničkou: $0{,}0123 \to 0{,}013$ (na $0{,}02$ by se skoro zdvojnásobila).
- **Výsledek** se zaokrouhlí tak, aby jeho poslední číslice byla **stejného řádu jako poslední číslice chyby**: $(9{,}78 \pm 0{,}04)\ \mathrm{m\,s^{-2}}$, ne $(9{,}783 \pm 0{,}04)$ ani $(9{,}8 \pm 0{,}04)$. Správně je i $(6{,}84 \pm 0{,}11)\ \mathrm{m}$.
- **Mocniny deseti:** nuly z činitele $10^n$ nejsou platné číslice. Odpor $25\,300\ \Omega$ platný na tři číslice se píše $25{,}3\ \mathrm{k\Omega}$ nebo $2{,}53\cdot10^{4}\ \Omega$.
- **Relativní chyba** je orientační údaj, stačí ji uvést na jednu až dvě platné číslice ($\doteq 0{,}8\ \%$).

Výpočtem se přesnost zvýšit nedá, ani když kalkulačka ukáže deset míst. Pro mezivýsledky bez spočtené chyby:

- **sčítání a odčítání**: výsledek se zaokrouhlí na poslední řád, který je platný **u všech** sčítanců: $12{,}4 + 3{,}27 + 0{,}058 - 0{,}21 = 15{,}518 \to 15{,}5$,
- **násobení a dělení**: výsledek má nanejvýš tolik platných číslic jako činitel s **nejmenším počtem** platných číslic: $3{,}142\cdot2{,}7 = 8{,}4834 \to 8{,}5$.

> [!key] Zápis výsledku
> $$X = (\bar x \pm \delta)\ \text{jednotka},\quad n = \dots,\quad P = 95\ \%,\quad \delta_\mathrm{r} = \dots\ \%$$
> Chyba na jednu platnou číslici (dvě, začíná-li jedničkou), zaokrouhlená nahoru. Průměr na stejný řád jako chyba. Velká a malá čísla s mocninou deseti.

> [!trap] Rozhoduje první platná číslice před zaokrouhlením
> $0{,}0904$ dá $0{,}1$, ne $0{,}10$ (první číslice je 9, takže stačí jedna). $0{,}152$ dá $0{,}16$ (začíná jedničkou, nechají se dvě a zaokrouhlí se nahoru).
