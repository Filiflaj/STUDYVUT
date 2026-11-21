+++
title = "Gaussův zákon"
topics = ["Tok a Gaussův zákon", "Gaussův zákon"]
+++

Pro **uzavřenou** plochu (koule, krychle, válec, jehlan…) platí jednoduché pravidlo: celkový tok ven je dán **jen nábojem uvnitř**. Tvar ani velikost plochy nehrají roli a náboje venku se nepočítají.

> [!key] Gaussův zákon
> $$\Phi = \oint E \cdot dS = \frac{Q_{\text{uvnitř}}}{\varepsilon_0} \qquad \left(\text{v dielektriku: } \frac{Q_{\text{uvnitř}}}{\varepsilon_0 \varepsilon_r}\right)$$

- **Bez náboje uvnitř je $\Phi = 0$.** Co vejde, to vyjde. Platí pro Gaussovy válce s různými podstavami, krychli v homogenním poli i jehlan.
- Dvě slupky ($R$ a $4R$) kolem stejné částice mají **stejný tok**. Plocha je 16× větší, ale pole 16× slabší.
- Náboj ve středu krychle: tok jednou stěnou je $Q/(6\varepsilon_0)$.
- Znaménko: tok **ven** je kladný, tok **dovnitř** záporný.

![Žádný náboj uvnitř → tok všemi plochami 0](images/ctyri-gaussovy-plochy.svg "Žádný náboj uvnitř → tok všemi plochami 0")

![Počítá se jen +3q − q = 2q, vnější +5q ne](images/uzavrena-plocha-naboje.svg "Počítá se jen +3q − q = 2q, vnější +5q ne")

![Jehlan: pláštěm vstupuje tolik, kolik základnou vystoupí](images/jehlan-v-poli.svg "Jehlan: pláštěm vstupuje tolik, kolik základnou vystoupí")

> [!example]
> Jehlan, $a = 1{,}7\,\mathrm{m}$, $E = 57{,}8\,\mathrm{V/m}$. Základnou vystupuje $E \cdot a^2 = 57{,}8 \cdot 2{,}89 = 167{,}04\,\mathrm{V \cdot m}$. Celkem musí být 0, takže pláštěm $\Phi$ = **−167,04 V·m**.
