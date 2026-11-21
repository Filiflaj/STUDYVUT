+++
title = "Úvod do měření: vyhodnocení závislostí"
topics = ["Vyhodnocení závislostí"]
+++

Často se neměří konstanta, ale to, jak veličina $y$ (závisle proměnná, funkce) závisí na veličině $x$ (nezávisle proměnná, argument). Ke zvoleným hodnotám $x_1, \dots, x_n$ se zapíšou naměřené $y_1, \dots, y_n$, dvojice se vynesou do grafu a podle tvaru se rozhodne, jakou funkcí závislost popsat. Kapitola shrnuje kapitolu 4 skripta *Úvod do měření*.

## Model musí mít fyzikální důvod {#model}

- **Závislost je známá** (zákon, teorie) a experiment ji má potvrdit. Hledají se „správné“ hodnoty koeficientů ve vzorci. Tím se zabývá **vyrovnávací počet**.
- **Závislost známá není.** Vysloví se hypotéza o tvaru funkce (modelové funkci) a ověří se jinou metodou.

> [!key] Funkci volte podle fyziky, ne podle toho, co lépe sedí
> Čekáte-li přímku, neprokládejte body parabolou jen proto, že vyjde „lepší shoda“. Když body přímce neodpovídají, hledejte v měření chybu.

Naměřené hodnoty $x$ i $y$ mají stejné chyby jako jakékoli měření: náhodné body rozptylují kolem hledané funkce, soustavné celou závislost posunou. **Hrubé chyby se vyřadí ještě před zpracováním.** Program po zadání tvaru funkce sám spočítá koeficienty a jejich shodu s měřením popíše **regresní koeficient**: při úplné shodě je roven 1 (Excel ukazuje $R^2$, hodnotu spolehlivosti).

V praktiku se nejčastěji ověřují tři tvary:

$$y = a + bx \qquad y = a\,\mathrm{e}^{bx} \qquad y = a\,x^{b}$$

## Lineární závislost {#linearni}

Graf $y = a + bx$ je přímka: $a$ je **úsek** na svislé ose ($x = 0$), $b$ je **směrnice**.

### Metoda nejmenších čtverců {#mnc}

Tabulkové programy a lepší kalkulačky prokládají přímku **metodou nejmenších čtverců**: hledají $a$ a $b$, pro které je součet čtverců svislých odchylek bodů od přímky nejmenší. Výsledek:

$$b = \frac{\sum (x_i - \bar x)(y_i - \bar y)}{\sum (x_i - \bar x)^2} \qquad a = \bar y - b\,\bar x$$

Metoda dává dobré výsledky, když mají chyby normální rozdělení. **Na hrubé chyby je citlivá**: odchylka se počítá ve druhé mocnině, takže jeden špatný bod přímku výrazně stáhne k sobě.

> [!example] Voltampérová charakteristika rezistoru
> | $I$ / mA | 10,0 | 20,0 | 30,0 | 40,0 | 50,0 |
> |---|---|---|---|---|---|
> | $U$ / V | 0,52 | 0,98 | 1,55 | 2,01 | 2,49 |
>
> Průměry: $\bar I = 30{,}0\ \mathrm{mA}$, $\bar U = 7{,}55/5 = 1{,}51\ \mathrm{V}$. Odchylky $I_i - \bar I$: $-20$; $-10$; $0$; $10$; $20$ mA. Odchylky $U_i - \bar U$: $-0{,}99$; $-0{,}53$; $0{,}04$; $0{,}50$; $0{,}98$ V.
> $$\sum (I_i - \bar I)(U_i - \bar U) = 19{,}8 + 5{,}3 + 0 + 5{,}0 + 19{,}6 = 49{,}7\ \mathrm{mA\,V}$$
> $$\sum (I_i - \bar I)^2 = 400 + 100 + 0 + 100 + 400 = 1000\ \mathrm{mA^2}$$
> $$b = \frac{49{,}7\ \mathrm{mA\,V}}{1000\ \mathrm{mA^2}} = 0{,}0497\ \mathrm{V\,mA^{-1}} = 49{,}7\ \Omega \qquad a = 1{,}51 - 0{,}0497\cdot30{,}0 = 0{,}019\ \mathrm{V}$$
> Proložená přímka je $U = 0{,}019\ \mathrm{V} + 49{,}7\ \Omega\cdot I$. Směrnice je odpor rezistoru, úsek je prakticky nulový, jak má u rezistoru být.
>
> **Jak to říct, když se zeptají:** „Přímku jsem proložil metodou nejmenších čtverců, ta minimalizuje součet čtverců odchylek bodů od přímky. Směrnice je podíl součtu součinů odchylek a součtu čtverců odchylek proudu. Proud je v miliampérech, proto vyšlo $0{,}0497\ \mathrm{V/mA}$, což je $49{,}7\ \Omega$.“

### Graficky: skupinová metoda {#skupinova}

Bez programu se přímka dá najít graficky. Zkušený experimentátor ji při nižších nárocích na přesnost proloží „od oka“. Přesnější je **skupinová metoda**:

1. Body s hrubou chybou vyřaďte.
2. Zbylé body rozdělte na dvě skupiny (levou a pravou).
3. Najděte **těžiště** každé skupiny (průměr $x$ a průměr $y$).
4. Přímku veďte oběma těžišti. Bod, který se při opakování měření zopakoval, může dostat dvojnásobnou váhu.

![Skupinová metoda](images/prakt-skupinova-metoda.svg "Přímka vedená těžišti dvou skupin bodů, bod s hrubou chybou je vyřazený")

### Směrnice z grafu {#smernice}

Úsek $a$ se odečte na svislé ose (přímku případně prodloužíte k $x = 0$). Směrnice se počítá **z hodnot na osách**:

$$b = \frac{y_2 - y_1}{x_2 - x_1}$$

> [!trap] Směrnice není tangens úhlu na papíře
> $b = \operatorname{tg}\alpha$ platí jen tehdy, mají-li obě osy stejné měřítko. Tatáž závislost nakreslená s jiným měřítkem svislé osy má jiný sklon, ale směrnice spočtená z hodnot na osách vyjde stejně. Směrnice navíc **má jednotku**, podíl jednotek $y$ a $x$ (tady $\mathrm{V}/\mathrm{A} = \Omega$).

![Směrnice ve dvou měřítkách](images/prakt-smernice.svg "Tatáž přímka ve dvou měřítkách svislé osy: úhly se liší, směrnice z hodnot na osách je stejná")

> [!key] Které dva body
> Do vzorce dosaďte dva **dostatečně vzdálené body ležící na proložené přímce**, ne naměřené hodnoty z tabulky. Nemusí to být krajní body: okraje měřeného intervalu bývají změřené méně přesně.

## Exponenciální a mocninná závislost {#exponencialni}

Exponenciální a mocninnou závislost lze vhodným zobrazením **převést na přímku** (linearizovat). Hodí se to nejen bez počítače:

- v přímce se snadno pozná **hrubá chyba** (v křivce ji přehlédnete),
- podle **zalomení přímky** se pozná, že body je lepší rozdělit do dvou skupin s různými koeficienty,
- z přímky se koeficienty snadno odečtou.

### Exponenciální funkce: semilogaritmický graf {#semilog}

$$y = a\,\mathrm{e}^{bx} \quad\Rightarrow\quad \ln y = \ln a + bx$$

Po dosazení $Y = \ln y$, $A = \ln a$ vyjde přímka $Y = A + bx$. Vynese se tedy **$\ln y$ proti $x$** na lineární osy. Směrnice je $b$ (u klesající závislosti záporná) a protože logaritmus je bezrozměrný, **jednotka $b$ je převrácená jednotka $x$**.

$$b = \frac{\ln y_2 - \ln y_1}{x_2 - x_1}$$

![Linearizace exponenciální závislosti](images/prakt-linearizace.svg "Exponenciální pokles v lineárních osách a tytéž body po zlogaritmování svislé osy")

Na **semilogaritmickém papíře** (nebo s logaritmickou svislou osou v Excelu) se na ose čtou přímo hodnoty $y$, osa je ale dělená v **dekadických** logaritmech. Protože $\ln y = \log y\cdot\ln 10$ a $\ln 10 \doteq 2{,}303$:

$$b = \frac{\log y_2 - \log y_1}{x_2 - x_1}\cdot\ln 10$$

> [!example] Útlum v absorbující vrstvě
> Body $y$ (výchylka úměrná prošlému světlu) v závislosti na tloušťce $x$ leží v grafu $\ln y$ proti $x$ na přímce. Na proložené přímce odečteme dva vzdálené body: $x_1 = 1{,}0\ \mathrm{cm}$, $y_1 = 56{,}4$ a $x_2 = 5{,}0\ \mathrm{cm}$, $y_2 = 13{,}9$.
> $$b = \frac{\ln 13{,}9 - \ln 56{,}4}{(5{,}0 - 1{,}0)\ \mathrm{cm}} = \frac{2{,}632 - 4{,}032}{4{,}0\ \mathrm{cm}} = -0{,}350\ \mathrm{cm^{-1}}$$
> Z dekadických logaritmů vyjde totéž: $\dfrac{1{,}143 - 1{,}751}{4{,}0\ \mathrm{cm}}\cdot2{,}303 = -0{,}350\ \mathrm{cm^{-1}}$. Koeficient $a$: $\ln a = \ln y_1 - b\,x_1 = 4{,}032 + 0{,}350 = 4{,}382$, tedy $a = \mathrm{e}^{4{,}382} \doteq 80$.
>
> **Jak to říct, když se zeptají:** „Exponenciálu jsem zlogaritmoval, $\ln y$ je lineární funkcí $x$ a směrnice té přímky je přímo $b$. Logaritmus je bezrozměrný, proto má $b$ jednotku $\mathrm{cm^{-1}}$. Body jsem bral na proložené přímce, ne z tabulky.“

### Mocninná funkce: logaritmický graf {#loglog}

$$y = a\,x^{b} \quad\Rightarrow\quad \log y = \log a + b\log x$$

Po dosazení $Y = \log y$, $X = \log x$ je to přímka $Y = A + bX$. Logaritmické musí být **obě osy** (logaritmický papír, v Excelu logaritmické měřítko na obou osách). Směrnice je **exponent** $b$:

$$b = \frac{\log y_2 - \log y_1}{\log x_2 - \log x_1}$$

> [!example] Stefanův–Boltzmannův zákon
> Výkon vyzařovaný vláknem žárovky má být úměrný $T^4$. V logaritmickém grafu leží body na přímce, odečteme na ní $(600\ \mathrm{K};\ 0{,}30\ \mathrm{W})$ a $(1500\ \mathrm{K};\ 11{,}7\ \mathrm{W})$:
> $$b = \frac{\log 11{,}7 - \log 0{,}30}{\log 1500 - \log 600} = \frac{1{,}068 - (-0{,}523)}{3{,}176 - 2{,}778} = \frac{1{,}591}{0{,}398} \doteq 4{,}00$$
> Exponent souhlasí se zákonem.
>
> **Jak to říct, když se zeptají:** „Mocninná funkce je v log–log grafu přímka a její směrnice je exponent. Exponent je bezrozměrný, proto na jednotkách nezáleží, jen obě hodnoty musí být ve stejných jednotkách.“

## Zásady pro grafy {#grafy}

V každém oboru jsou zvyklosti trochu jiné. Ve fyzikálním praktiku se držte těchto pravidel, ať graf kreslíte ručně (milimetrový, semilogaritmický nebo logaritmický papír A4) nebo v počítači:

1. **Nezávisle proměnná na vodorovnou osu**, kladné hodnoty rostou doprava a nahoru. V polárních souřadnicích se úhel čte od vodorovné nebo svislé osy proti směru hodinových ručiček.
2. **Osy popište** značkou nebo názvem veličiny a jednotkou za lomítkem nebo v závorce: $U$ / V. Stupnici dejte na vnější stranu os s dílky tak daleko od sebe, aby se dalo pohodlně odečítat. Čísla pište vodorovně, i u svislé osy. Velké a malé hodnoty vyjádřete mocninou deseti nebo předponou jednotky. Souřadnice jednotlivých bodů na osách nevyznačujte, jsou v tabulce.
3. **Měřítka volte tak, aby křivka zaplnila co největší část plochy.** Nulu do průsečíku os dávejte, jen když je důležitá (třeba aby bylo vidět, že přímka prochází počátkem). Jinak začněte stupnici kousek pod nejmenší hodnotou.
4. Chyba odečtu obou souřadnic je stejná jen tam, kde je sklon křivky 1. Kde je křivka skoro rovnoběžná s osou, je odečet jedné souřadnice nepřesný.
5. **Body výrazně označte, nejlépe křížkem.** Tečky po protažení čáry zmizí. Více řad bodů v jednom grafu odlište různými značkami (kroužek, čtverec, trojúhelník). Barvy jen tehdy, když se bude graf tisknout a kopírovat barevně. Ke každé křivce napište hodnotu parametru, kterým se liší.
6. **Body nespojujte lomenou čarou**, spojnice bodů nemá fyzikální význam. Prokládá se **hladká křivka** bez neopodstatněných skoků, zlomů a extrémů, s přibližně stejným počtem bodů nad i pod.
7. Graf má **číslo a výstižný název**, podle potřeby i podmínky měření. Body, které z grafu odečítáte pro další výpočet, vyznačte jinak než naměřené body, na křivce i na ose.

> [!trap] Graf v Excelu
> - Typ grafu je vždy **XY bodový**, ne „spojnicový“. Spojnicový graf dává body do stejných rozestupů bez ohledu na hodnoty $x$.
> - Body nepropojujte čarou. Křivku přidáte jako **spojnici trendu** a typ regrese (lineární, exponenciální, mocninná) volíte **podle fyzikálního zákona**, ne podle vzhledu grafu. Zaškrtněte „zobrazit rovnici v grafu“.
> - V log–log grafu leží mocninná závislost na přímce jen díky logaritmickým osám, trend se proto volí mocninný, ne lineární.
> - Všechna pravidla výše platí i pro graf z počítače: popsané osy s jednotkami, číslo a název grafu.
