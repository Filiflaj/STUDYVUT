+++
title = "Úloha 5: Pohyb po nakloněné rovině"
topics = ["Úloha 5: nakloněná rovina"]
+++

**Úkol:** změřit zrychlení tělesa, které klouže po nakloněné rovině, a z něj spočítat součinitel smykového tření mezi dvěma materiály. Měří se dvě kombinace (jiný úhel nebo jiné materiály).

## Síly a pohybová rovnice {#sily}

![Síly na těleso na nakloněné rovině](images/lab-05-naklonena-rovina.svg "Tíha G a její složky, normálová síla N, třecí síla Ft")

Osa $x$ míří dolů podél roviny, osa $y$ kolmo k ní. Tíhu $G = mg$ rozložíme:

$$G_x = mg\sin\alpha \qquad G_y = mg\cos\alpha$$

- v ose $y$ se těleso nepohybuje: $N = G_y = mg\cos\alpha$,
- třecí síla působí proti pohybu: $F_\mathrm{t} = \mu N = \mu\, mg\cos\alpha$,
- v ose $x$ platí 2. Newtonův zákon: $mg\sin\alpha - \mu\, mg\cos\alpha = ma$.

> [!key] Dva vztahy, které se počítají v testu i v protokolu
> $$a = g\,(\sin\alpha - \mu\cos\alpha) \qquad \mu = \frac{\sin\alpha - \dfrac{a}{g}}{\cos\alpha}$$
> Zrychlení **nezávisí na hmotnosti**. Bez tření $a = g\sin\alpha$. Návod bere $g = 9{,}81\ \mathrm{m\,s^{-2}}$.

**Součinitel smykového tření** $\mu = F_\mathrm{t}/N$ je bezrozměrný. Závisí na materiálech, drsnosti a čistotě povrchů, ne na hmotnosti ani na velikosti styčné plochy. Obvykle $0 < \mu < 1$ (led–led asi 0,01, dřevo–dřevo 0,3–0,5, guma na suchém asfaltu i přes 1).

**Statické a dynamické tření.** Statické tření drží těleso v klidu a přizpůsobí se až do $\mu_\mathrm{s}N$. Při klouzání působí dynamické tření $\mu_\mathrm{d}N$, obvykle $\mu_\mathrm{d} < \mu_\mathrm{s}$: rozjet je těžší než udržet v pohybu.

**Podmínka rozjezdu:** $mg\sin\alpha > \mu\, mg\cos\alpha$, tedy $\operatorname{tg}\alpha > \mu$. Nejmenší úhel, při kterém se těleso rozjede, je $\alpha_\mathrm{min} = \operatorname{arctg}\mu_\mathrm{s}$.

## Pracoviště a měření {#mereni}

- Plošina se sklonem 0–45° (měří se v rozsahu 25–35°), vyměnitelná deska (dřevo, plast, nerez) a závaží z různých materiálů.
- Podél roviny je řada **optických bran**. Osciloskop (spouštěný první branou) a kontrolní počítač dají časy průjezdu jednotlivými branami, úhel ukazuje počítač.
- Vzdálenosti bran se změří měřítkem. Před měřením se ověří, že těleso opravdu klouže rovnoměrně zrychleně (zkušební průjezd).

## Zpracování {#zpracovani}

1. Ke každé bráně čas $t_i$ a dráha $s_i$ od první brány. Graf $s(t)$ je parabola.
2. Z úseků mezi branami rychlosti $v_i = \Delta s_i/\Delta t_i$. Průměrná rychlost v úseku patří do **středu časového intervalu** (u rovnoměrně zrychleného pohybu přesně).
3. Graf $v(t)$ je přímka $v = at + v_0$. **Směrnice přímky** proložené metodou nejmenších čtverců **je zrychlení** $a$.
4. Ze zrychlení a úhlu se spočítá $\mu$. Chyba $\mu$ se dostane šířením chyb (parciální derivace podle $a$ a $\alpha$).

> [!trap] Kvadratický fit $s(t)$
> Proložením paraboly $y = Ax^2 + Bx + C$ dostaneme $A = a/2$, protože $s = \tfrac12 at^2 + v_0t + s_0$. Zrychlení je **dvojnásobek** koeficientu u $t^2$, ne koeficient sám.

> [!trap] Kdy těleso vůbec nepojede
> Pro ocel–ocel uvádí návod $\mu \approx 0{,}6$, ale $\operatorname{tg}25^\circ \doteq 0{,}47$. Kdyby bylo opravdu $\mu = 0{,}6$, těleso by při 25° zůstalo stát. Když klouže, musí být $\mu < \operatorname{tg}\alpha$, a to je dobrá kontrola výsledku.

> [!example] Příklad z měření
> V protokolu vyšlo pro ocel na oceli při $\alpha \doteq 25{,}1^\circ$ zrychlení $a = (2{,}61 \pm 0{,}07)\ \mathrm{m\,s^{-2}}$ a z něj $\mu = 0{,}174 \pm 0{,}009$. Kontrola: $\mu < \operatorname{tg}\alpha \doteq 0{,}47$, takže těleso opravdu klouže.

## Na co se ptají ve vstupním testu {#test}

> [!note] Okruhy k testu
> - rozklad tíhy na nakloněné rovině, $N = mg\cos\alpha$, $F_\mathrm{t} = \mu\, mg\cos\alpha$,
> - pohybová rovnice a $a = g(\sin\alpha - \mu\cos\alpha)$, nezávislost na hmotnosti,
> - statické × dynamické tření, význam a jednotka $\mu$,
> - podmínka rozjezdu $\operatorname{tg}\alpha > \mu$, minimální úhel $\alpha_\mathrm{min} = \operatorname{arctg}\mu$,
> - kinematika rovnoměrně zrychleného pohybu: $s(t)$ parabola, $v(t)$ přímka se směrnicí $a$,
> - výpočet $\mu$ z naměřeného zrychlení a popis pracoviště (optické brány, osciloskop).
