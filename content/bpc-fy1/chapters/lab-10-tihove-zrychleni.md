+++
title = "Úloha 10: Tíhové zrychlení (reverzní kyvadlo)"
topics = ["Úloha 10: reverzní kyvadlo"]
+++

**Úkol:** určit místní tíhové zrychlení $g$ reverzním kyvadlem a porovnat ho s tabulkovou hodnotou.

## Pojmy {#pojmy}

- **Tíhové zrychlení** $g$ (jednotka $\mathrm{m\,s^{-2}}$) je zrychlení volného pádu ve vakuu. Skládá se z gravitace a odstředivého účinku rotace Země, takže závisí na místě: na pólech je větší než na rovníku, s nadmořskou výškou klesá, nepatrně ho ovlivňuje i Měsíc (slapy).
- **Těžiště** se určí zavěšováním (protínají se v něm těžnice), **hmotný střed** je dán rozložením hmoty. V homogenním tíhovém poli splývají; v beztížném stavu těžiště ztrácí smysl.
- **Fyzické kyvadlo:** tuhé těleso kývající kolem vodorovné osy, která neprochází těžištěm (bez tření a odporu prostředí).
- **Matematické kyvadlo:** hmotný bod na nehmotném nepružném vlákně délky $\ell$.

Pro malý rozkyv (asi do 5°):

$$T = 2\pi\sqrt{\frac{J}{m g d}} \ \ \text{(fyzické)} \qquad T = 2\pi\sqrt{\frac{\ell}{g}} \ \ \text{(matematické)}$$

$J$ je moment setrvačnosti k ose kyvu, $m$ hmotnost, $d$ vzdálenost osy od těžiště. Doba kmitu matematického kyvadla **nezávisí na hmotnosti** ani na rozkyvu (pro malé úhly).

> [!key] Redukovaná délka
> **Redukovaná délka** $\ell_\mathrm{r}$ fyzického kyvadla je délka matematického kyvadla, které má **stejnou dobu kmitu**. Z ní:
> $$g = \frac{4\pi^2\,\ell_\mathrm{r}}{T^2} \qquad \ell_\mathrm{r} = \frac{g\,T^2}{4\pi^2} = \frac{J}{m d}$$

## Princip reverzního kyvadla {#princip}

![Reverzní kyvadlo](images/lab-10-reverzni-kyvadlo.svg "Tyč se dvěma břity O1, O2 obrácenými ostřím k sobě a posuvným závažím Z")

$J$ a $d$ neznáme, proto se použije vztah pro matematické kyvadlo. Platí, že každé fyzické kyvadlo má dvě různé osy se stejnou dobou kmitu a jejich vzdálenost je právě $\ell_\mathrm{r}$. Reverzní kyvadlo má dva pevné břity $O_1$, $O_2$ a posuvné závaží Z:

1. Pro několik poloh závaží $a$ (3 až 5) se změří doba kmitu kolem obou os (kyvadlo se převrací).
2. Obě závislosti $T(a)$ se vynesou do grafu. **Průsečík** dává polohu, kde $T_{O1} = T_{O2}$.
3. Závaží se nastaví do průsečíku. Vzdálenost břitů je pak redukovanou délkou a stačí ji změřit měřítkem.

![Průsečík závislostí T(a)](images/lab-10-prusecik.svg "Doby kmitu kolem obou os v závislosti na poloze závaží")

## Měření {#mereni}

- $\ell_\mathrm{r}$ (vzdálenost břitů) se měří 10krát ocelovým měřítkem, poloha závaží $a$ posuvným měřítkem.
- Dobu kmitu měří optický snímač (infračervený paprsek se odráží od kyvadla) připojený přes USB k webové aplikaci. Ta ukáže graf $T(a)$ s průsečíkem.
- V nastavené poloze se doba kmitu změří **postupnou metodou** pro obě osy: časy po 10 kmitech až do 90, rozdíly sloupců B − A dají 5 hodnot doby 50 kmitů.

## Zpracování {#zpracovani}

1. $\bar{\ell}_\mathrm{r}$, $s_{\bar{x}}$, $\delta(\ell_\mathrm{r}) = 2{,}320\cdot s_{\bar{x}}$ ($n = 10$) a porovnat s chybou měřítka.
2. Pro každou osu $\overline{50T}$, $\delta(50T) = 2{,}968\cdot s_{\bar{x}}$ ($n = 5$), pak $T = \overline{50T}/50$ a $\delta(T) = \delta(50T)/50$.
3. $g = 4\pi^2\ell_\mathrm{r}/T^2$ zvlášť pro obě osy a relativní chyba
$$\delta_\mathrm{r}(g) = \delta_\mathrm{r}(\ell_\mathrm{r}) + 2\,\delta_\mathrm{r}(T)$$
4. Porovnat s tabulkovou hodnotou: rozdíl větší než $3\delta$ znamená soustavnou chybu.

> [!trap] Na co si dát pozor
> - $\ell_\mathrm{r}$ dosazovat v metrech, $T$ je doba **jednoho** kmitu (ne 50).
> - Doba kmitu je ve vzorci na druhou: její relativní chyba se počítá dvakrát.
> - Větší rozkyv dobu kmitu **prodlouží**, takže $g$ by vyšlo **menší**. Měří se proto s malým rozkyvem.
> - Když závaží není přesně v průsečíku, doby kolem obou os se liší a vzdálenost břitů už není přesně $\ell_\mathrm{r}$.

> [!example] Příklad z měření
> V protokolu vyšlo $g_{O1} = (9{,}862 \pm 0{,}013)\ \mathrm{m\,s^{-2}}$ a $g_{O2} = (9{,}876 \pm 0{,}009)\ \mathrm{m\,s^{-2}}$. Obě hodnoty se shodují, ale od tabulkové hodnoty pro Brno ($9{,}8094\ \mathrm{m\,s^{-2}}$) se liší o víc než $3\delta$, tedy nejspíš soustavná chyba (například závaží ne zcela přesně v průsečíku).

## Na co se ptají ve vstupním testu {#test}

> [!note] Typické otázky (Dodatek návodu)
> - **Doba kmitu fyzického kyvadla je $4\pi$ s. Jaká je redukovaná délka?** $\ell_\mathrm{r} = gT^2/(4\pi^2) = g\cdot16\pi^2/(4\pi^2) = 4g \doteq 39{,}2\ \mathrm{m}$.
> - **Moment setrvačnosti z $T$, $m$, $d$:** z $T = 2\pi\sqrt{J/(mgd)}$ je $J = T^2 m g d/(4\pi^2)$. Pro $m = 0{,}6\ \mathrm{kg}$, $d = 0{,}9\ \mathrm{m}$, $T = 4\ \mathrm{s}$ vyjde $J \doteq 2{,}15\ \mathrm{kg\,m^2}$. Zadávají se i varianty, kdy se počítá $T$, $m$ nebo $d$.
> - **Změní se $\ell_\mathrm{r}$, když se změní $J$, $m$ nebo $d$, ale doba kmitu zůstane stejná?** Nezmění: redukovaná délka je určená jen dobou kmitu.
> - definice fyzického a matematického kyvadla, redukované délky, těžiště a hmotného středu, na čem závisí $g$.
