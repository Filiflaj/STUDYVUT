+++
title = "Volný pád a vrhy"
topics = ["Vrhy", "Volný pád a vrhy"]
+++

Všechno, co letí bez odporu vzduchu, má zrychlení **$g$ svisle dolů**, a to po celou dobu, i v nejvyšším bodě. Upuštěný, hozený dolů i hozený nahoru předmět mají v každém okamžiku stejné zrychlení.

Trik: pohyb rozděl na dva **nezávislé** pohyby.

- **vodorovně** nic nepůsobí → rovnoměrný pohyb, $v_x = v_0 \cos \alpha$ = konst.
- **svisle** působí $g$ → $v_y = v_0 \sin \alpha - g \cdot t$

![Vodorovná složka (černá) se nemění, svislá (modrá) klesá](images/sikmy-vrh.svg "Vodorovná složka (černá) se nemění, svislá (modrá) klesá")

> [!key] Vzorce šikmého vrhu
> $$x = v_0 \cos\alpha \cdot t \qquad y = v_0 \sin\alpha \cdot t - \tfrac{1}{2} g t^2$$

## Grafy

![Grafy složek rychlosti a výšky](images/graf-krivek.svg "AB = $v_x(t)$, AP = $v_y(t)$, OC = $y(t)$")

- $v_x(t)$ je vodorovná přímka (AB).
- $v_y(t)$ je klesající přímka, která protne osu v nejvyšším bodě (AP).
- Výška $y(t)$ je parabola (OC).

> [!example]
> Vodorovně vystřelená kulka dopadne **současně** s upuštěnou, protože svisle se pohybují úplně stejně.
>
> Sněhová koule na cíl ve vzdálenosti $d$ a výšce $y$: z $x$ vyjádři $t = d/(v_0 \cos \alpha)$ a dosaď do $y$: $y = d \operatorname{tg} \alpha - g d^2/(2v_0^2 \cos^2 \alpha)$.

> [!trap]
> V nejvyšším bodě je nulová jen **svislá** rychlost. Celková rychlost je $v_0 \cos \alpha$ a zrychlení je pořád $g$.
