+++
title = "Úloha 1: Parametry reálného zdroje"
topics = ["Reálný zdroj"]
+++

Na přípravku jsou dva neznámé zdroje. Měřením naprázdno, při zátěži a nakrátko se zjistí jejich **vnitřní napětí a odpor** (model zdroje napětí) a **vnitřní proud a vodivost** (model zdroje proudu). Přístroje jsou **analogové**, zapisuje se tedy výchylka $\alpha$ a konstanta $k$.

## Model reálného zdroje napětí

Reálný zdroj napětí = ideální zdroj **vnitřního napětí** $U_\text{i}$ v sérii s **vnitřním odporem** $R_\text{i}$. Když zdroj dodává proud $I_\text{z}$, část napětí se ztratí na $R_\text{i}$ a na svorkách zbude **svorkové napětí**:

$$U_\text{z} = U_\text{i} - R_\text{i}\,I_\text{z}$$

![Model reálného zdroje napětí a jeho zatěžovací charakteristika](images/zdroj-napeti.svg "Model zdroje napětí se zátěží a zatěžovací (VA) charakteristika")

Dva krajní stavy dávají oba parametry:

- **naprázdno** ($R_\text{z} \to \infty$, $I_\text{z} = 0$): na svorkách je celé vnitřní napětí, $U_\text{i} = U_0$,
- **nakrátko** ($R_\text{z} = 0$, $U_\text{z} = 0$): teče proud nakrátko $I_\text{k}$, takže $R_\text{i} = U_\text{i} / I_\text{k}$.

**Zatěžovací (VA) charakteristika** $U_\text{z} = f(I_\text{z})$ je přímka z bodu $U_0$ na ose napětí do bodu $I_\text{k}$ na ose proudu. Její sklon je $R_\text{i}$: čím menší vnitřní odpor, tím méně svorkové napětí při zatížení klesá. **Ideální zdroj napětí** má $R_\text{i} = 0$ a vodorovnou charakteristiku.

## Model zdroje proudu a ekvivalence

Stejný zdroj jde popsat i jako ideální zdroj **vnitřního proudu** $I_\text{i}$ s paralelní **vnitřní vodivostí** $G_\text{i}$:

$$I_\text{i} = I_\text{k}, \qquad G_\text{i} = \frac{I_\text{k}}{U_0}, \qquad I_\text{z} = I_\text{i} - G_\text{i}\,U_\text{z}$$

**Ideální zdroj proudu** má $G_\text{i} = 0$ a svislou charakteristiku. Oba modely mají stejnou zatěžovací charakteristiku, takže jsou navenek **ekvivalentní**:

$$I_\text{i} = \frac{U_\text{i}}{R_\text{i}}, \quad G_\text{i} = \frac{1}{R_\text{i}} \qquad\Longleftrightarrow\qquad U_\text{i} = \frac{I_\text{i}}{G_\text{i}}, \quad R_\text{i} = \frac{1}{G_\text{i}}$$

![Ekvivalentní zdroj napětí a zdroj proudu](images/ekvivalence-zdroju.svg "Ekvivalence: zdroj napětí (Ui, Ri) a zdroj proudu (Ii, Gi)")

> [!key] Co si zapamatovat
> $U_\text{i} = U_0$, $\;R_\text{i} = U_0/I_\text{k}$, $\;I_\text{i} = I_\text{k}$, $\;G_\text{i} = I_\text{k}/U_0 = 1/R_\text{i}$. Kontrola: $R_\text{i}\cdot G_\text{i} = 1$ (kΩ · mS = 1).

## Měření

![Tři zapojení: naprázdno, se zátěží, nakrátko](images/zdroj-mereni.svg "a) voltmetr přímo na svorkách, b) miliampérmetr v sérii a voltmetr na zátěži, c) miliampérmetr přímo na svorkách")

1. **Naprázdno:** voltmetr přímo na svorky, odečte se $U_0$.
2. **Se zátěží:** miliampérmetr v sérii s proměnnou zátěží $R_\text{z}$, voltmetr na zátěži. Zátěž se nastavuje tak, aby proud byl 0, 15, 30, … 75 mA (zdroj 1), resp. 0, 10, … 50 mA (zdroj 2), a ke každému proudu se odečte napětí.
3. **Nakrátko:** miliampérmetr přímo na svorky, odečte se $I_\text{k}$ (začít na nejvyšším rozsahu, proud je větší než při zátěži).

Zapojení se mění jen při odpojeném napáječi. Kontrola na místě: napětí má s rostoucím proudem klesat rovnoměrně (stejný krok proudu, stejný pokles napětí).

## Co patří do sešitu

- tab. 1.1 a 1.2: $\alpha$ a $k$ u každého sloupce, šedé buňky $X = \alpha\cdot k$,
- výpočty: jeden převod $\alpha\cdot k$, pak $U_\text{i}$, $R_\text{i}$, $I_\text{i}$, $G_\text{i}$ pro oba zdroje,
- tab. 1.3 a **náhradní schémata s čísly**: pro každý zdroj napěťové ($U_\text{i}$ v sérii s $R_\text{i}$) i proudové ($I_\text{i}$ paralelně s $G_\text{i}$),
- graf: VA charakteristiky obou zdrojů do jednoho grafu, odlišené značkami,
- přístroje včetně třídy přesnosti, závěr.

## Příklad z měření {#priklad}

> [!example] Příklad z měření: napětí naprázdno se nezapsalo
> V jednom měření zůstalo v prvním sloupci (naprázdno) zapsané 0 V, což fyzikálně nejde: naprázdno je na svorkách napětí největší. Vnitřní napětí se pak určí **prodloužením charakteristiky** k ose napětí přes první a poslední bod se zátěží. Zdroj 1 dal 8,0 V při 15 mA a 0,32 V při 72 mA, nakrátko $I_\text{k1}$ = 77 mA.
>
> Sklon charakteristiky:
> $$\frac{\Delta U}{\Delta I} = \frac{8{,}0 - 0{,}32}{72 - 15} = \frac{7{,}68}{57} = 0{,}135\ \text{V/mA}$$
> Prodloužení do $I_\text{z} = 0$ (z $U_\text{z} = U_\text{i} - R_\text{i} I_\text{z}$):
> $$U_\text{i1} = U_\text{z} + \frac{\Delta U}{\Delta I}\,I_\text{z} = 8{,}0 + 0{,}135\cdot 15 = 10{,}0\ \text{V}$$
> Další parametry:
> $$R_\text{i1} = \frac{U_\text{i1}}{I_\text{k1}} = \frac{10{,}0\ \text{V}}{77\ \text{mA}} = 0{,}130\ \text{k}\Omega = 130\ \Omega, \qquad G_\text{i1} = \frac{77\ \text{mA}}{10{,}0\ \text{V}} = 7{,}70\ \text{mS}$$
> Kontrola: $0{,}130\ \text{k}\Omega \cdot 7{,}70\ \text{mS} = 1{,}00$. Zdroj 2 stejně: sklon $9{,}0/40 = 0{,}225$ V/mA, $U_\text{i2} = 12{,}5$ V, $I_\text{k2}$ = 54 mA, $R_\text{i2}$ = 231 Ω, $G_\text{i2}$ = 4,32 mS. Ideálnímu zdroji napětí se víc blíží zdroj 1 (menší $R_\text{i}$).
>
> **Jak to říct, když se zeptají:** „Napětí naprázdno se nezapsalo. Charakteristika je přímka $U_\text{z} = U_\text{i} - R_\text{i} I_\text{z}$, takže ji prodlužuji k nulovému proudu. Sklon počítám z prvního a posledního bodu se zátěží. V grafu je to průsečík prodloužené přímky s osou U.“

![Naměřené VA charakteristiky obou zdrojů](images/va-zdroje-mereni.svg "Příklad z měření: body obou zdrojů, přímky přes první a poslední bod se zátěží a jejich prodloužení k ose U")

> [!trap] Na co si dát pozor
> - Napětí naprázdno nesmí vyjít 0 V. Bod (0 mA; 0 V) se do grafu nekreslí.
> - Když výchylka ampérmetru neodpovídá předtištěnému proudu (např. 72 místo 75 mA), počítá se se skutečnou hodnotou.
> - V/mA = kΩ: $10{,}0/77 = 0{,}130$, tedy 130 Ω, ne 0,13 Ω.
> - Napětí zapsané u zkratu není napětí zdroje, je to úbytek na miliampérmetru (zde asi 0,3 V, tj. odpor ampérmetru kolem 3,8 Ω).
> - Body se neprokládají lomenou čarou, charakteristika je přímka.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Co je vnitřní napětí a odpor?** Parametry modelu zdroje napětí. $U_\text{i}$ je napětí naprázdno, $R_\text{i}$ způsobuje pokles svorkového napětí při zatížení.
> - **Co je svorkové napětí?** Napětí na svorkách při zatížení, $U_\text{z} = U_\text{i} - R_\text{i} I_\text{z}$.
> - **Jak se pozná ideální zdroj napětí a proudu?** Napětí: $R_\text{i} = 0$, vodorovná charakteristika. Proud: $G_\text{i} = 0$, svislá charakteristika.
> - **Jak určit $R_\text{i}$ z měření?** $R_\text{i} = U_0/I_\text{k}$, nebo ze sklonu charakteristiky $\Delta U/\Delta I$ mezi dvěma body.
> - **Kdy jsou zdroj napětí a proudu ekvivalentní?** Když $I_\text{i} = U_\text{i}/R_\text{i}$ a $G_\text{i} = 1/R_\text{i}$.
> - **Proč může být naměřené $U_0$ o něco menší než $U_\text{i}$?** Analogový voltmetr odebírá proud, zdroj tedy není úplně naprázdno.
> - **Proč se $k$ píše jako zlomek a proč měřit v poslední třetině stupnice?** Viz kapitola Jak na laborky.
