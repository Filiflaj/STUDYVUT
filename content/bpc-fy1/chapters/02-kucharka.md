+++
title = "Kuchařka: úloha se silami krok za krokem"
topics = ["Tření a rozklad sil", "Pohybová rovnice"]
+++

Skoro každá úloha s bednou, lanem a třením se řeší stejně:

1. **Nakresli síly:** tíha $G$ (vždy svisle dolů), normálová síla $N$ (kolmo od podložky), síla lana nebo ruky $F$, tření $F_t$ (proti pohybu).
2. **Zvol osy:** $x$ ve směru pohybu, $y$ kolmo na něj.
3. **Rozlož šikmou sílu:** složka u úhlu dostane cos, druhá sin: $F_x = F \cos \alpha$, $F_y = F \sin \alpha$.
4. **Osa y** (zrychlení 0) → z ní dostaneš $N$.
5. **Osa x** → $m \cdot a$ = (síly dopředu) − (síly dozadu).

![Bedna tažená lanem pod úhlem α](images/bedna-rozklad-sil.svg "Bedna tažená lanem pod úhlem α")

> [!key] Výsledek pro tuto bednu
> $$\text{osa } y\text{:} \quad N + F\sin\alpha - G = 0 \quad\Rightarrow\quad N = G - F\sin\alpha$$
>
> $$\text{tření:} \quad F_t = f \cdot (G - F\sin\alpha)$$
>
> $$\text{osa } x\text{:} \quad (G/g) \cdot a = F\cos\alpha - f \cdot (G - F\sin\alpha)$$

> [!trap]
> $G$ je tíha v newtonech, ne hmotnost. Hmotnost je $m = G/g$.
