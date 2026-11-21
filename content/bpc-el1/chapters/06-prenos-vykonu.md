+++
title = "Úloha 6: Přenos výkonu"
topics = ["Přenos výkonu"]
+++

Ke zdroji s vnitřním odporem $R_\text{i}$ = 220 Ω se připojuje odporová dekáda $R_\text{z}$ = 20 až 460 Ω po 40 Ω. Z naměřeného napětí a proudu zátěže se počítá výkon (nepřímá metoda) a hledá se, kdy je největší. Voltmetr i miliampérmetr jsou multimetry.

## Výkon na zátěži

Zdroj se popíše Théveninovým modelem ($U_\text{i}$ v sérii s $R_\text{i}$, úloha 1 a 3). Zátěží teče proud a je na ní napětí

$$I_\text{z} = \frac{U_\text{i}}{R_\text{i}+R_\text{z}}, \qquad U_\text{z} = U_\text{i}\,\frac{R_\text{z}}{R_\text{i}+R_\text{z}}$$

a výkon na zátěži je

$$P_\text{z} = U_\text{z} I_\text{z} = U_\text{i}^2\,\frac{R_\text{z}}{(R_\text{i}+R_\text{z})^2}$$

Při zkratu ($R_\text{z} = 0$) je napětí nulové, naprázdno ($R_\text{z}\to\infty$) proud nulový, v obou případech $P_\text{z} = 0$. Mezi nimi musí být maximum. Derivace podle $R_\text{z}$ položená rovna nule:

$$\frac{\mathrm{d}P_\text{z}}{\mathrm{d}R_\text{z}} = U_\text{i}^2\,\frac{R_\text{i}-R_\text{z}}{(R_\text{i}+R_\text{z})^3} = 0 \quad\Rightarrow\quad R_\text{z} = R_\text{i}$$

> [!key] Výkonové přizpůsobení
> Největší výkon do zátěže jde při $R_\text{z} = R_\text{i}$:
> $$P_\text{max} = \frac{U_\text{i}^2}{4R_\text{i}}, \qquad U_\text{z} = \frac{U_\text{i}}{2}, \qquad I_\text{z} = \frac{I_\text{k}}{2}, \qquad \eta = 50\ \%$$

## Účinnost

Zdroj dodává $P_\text{i} = U_\text{i} I_\text{z}$; část se ztratí na $R_\text{i}$, zbytek jde do zátěže. Oběma odpory teče stejný proud, takže se $I_\text{z}^2$ vykrátí:

$$\eta = \frac{P_\text{z}}{P_\text{i}}\cdot 100 = \frac{R_\text{z}}{R_\text{i}+R_\text{z}}\cdot 100\ \%$$

Účinnost **nezávisí na $U_\text{i}$** a roste s $R_\text{z}$; sloupec $\eta$ jde proto vyplnit už doma. Maximálního výkonu a vysoké účinnosti nejde dosáhnout zároveň. S $k = R_\text{z}/R_\text{i}$:

$$\frac{P_\text{z}}{P_\text{max}} = \frac{4k}{(1+k)^2}, \qquad \eta = \frac{k}{1+k}$$

Pro $\eta$ = 90 % musí být $R_\text{z} = 9R_\text{i}$ a zátěž dostane jen 36 % $P_\text{max}$. Kolem maxima je křivka plochá: při $R_\text{z} = 2R_\text{i}$ i $R_\text{i}/2$ je to pořád 89 % $P_\text{max}$.

Přizpůsobení se používá ve sdělovací a vf technice (anténa a vstup přijímače), kde je signál slabý a o účinnost nejde. V energetice se volí $R_\text{z} \gg R_\text{i}$, aby byla účinnost blízko 100 %.

![Výkon a účinnost v závislosti na odporu zátěže](images/prenos-vykonu.svg "Teoretické průběhy pro Ui = 12 V a Ri = 220 Ω (Pz plně, η čárkovaně), kroužky jsou naměřené výkony")

## Měření

1. Nejdřív sériová smyčka: + zdroje → $R_\text{i}$ na přípravku → zdířka mA → COM → dekáda → − zdroje. Pak voltmetr **přímo na svorky dekády**.
2. Pro každé $R_\text{z}$ zapsat $U_\text{z}$ a $I_\text{z}$ (všechny číslice z displeje).
3. Rychlá kontrola řádku: $U_\text{z}/I_\text{z} \approx R_\text{z}$ (V/mA = kΩ) a $U_\text{z} + R_\text{i} I_\text{z} \approx U_\text{i}$ ve všech řádcích stejně.
4. Při $R_\text{z}$ = 220 Ω musí vyjít $U_\text{z} \approx U_\text{i}/2$.

Do sešitu: tab. 6.1 ($\eta$ z přípravy, $P_\text{z}$ po měření), příklady výpočtu $\eta$, $P_\text{z}$, $P_\text{max}$ a **jeden graf** $P_\text{z}(R_\text{z})$ a $\eta(R_\text{z})$ se dvěma svislými osami; v grafu vyznačit $P_\text{max}$ a graficky odečíst $\eta$ v tomto bodě.

> [!example] Příklad z měření
> Výkon pro první řádek ($R_\text{z}$ = 20 Ω): $P_\text{z} = U_\text{z} I_\text{z} = 1{,}005\ \text{V}\cdot 50{,}1\ \text{mA} = 50{,}4\ \text{mW}$ (V · mA = mW). Účinnost: $\eta = 20/(220 + 20)\cdot 100 = 8{,}3\ \%$.
>
> Napětí zdroje se nezapsalo, určí se z naměřených hodnot podle $U_\text{z} = U_\text{i} - R_\text{i} I_\text{z}$, např. pro 220 Ω:
> $$U_\text{i} = U_\text{z} + R_\text{i} I_\text{z} = 5{,}98 + 220\cdot 0{,}02738 = 5{,}98 + 6{,}02 = 12{,}0\ \text{V}$$
> (ve všech dvanácti řádcích 11,99 až 12,03 V). Teoretické maximum:
> $$P_\text{max} = \frac{U_\text{i}^2}{4R_\text{i}} = \frac{12{,}0^2}{4\cdot 220} = \frac{144}{880} = 0{,}164\ \text{W} = 164\ \text{mW}$$
> Největší naměřený výkon byl 164 mW právě při 220 Ω, odchylka 0 %. Při 180 a 260 Ω vyšlo 162 a 163 mW: vrchol je plochý.
>
> **Jak to říct, když se zeptají:** „$U_\text{i}$ určuji z naměřeného $U_\text{z}$ a $I_\text{z}$ podle $U_\text{i} = U_\text{z} + R_\text{i} I_\text{z}$, vychází ve všech řádcích 12,0 V, takže zdroj byl nastavený na 12 V.“

> [!trap] Na co si dát pozor
> - Miliampérmetr zapojený paralelně je zkrat přes ampérmetr (pojistka).
> - Voltmetr přes $R_\text{i}$ i dekádu měří $U_\text{i}$, ne $U_\text{z}$; poznáš to tak, že se jeho údaj s $R_\text{z}$ nemění.
> - $\eta$ v příkladu: ve jmenovateli $R_\text{i} + R_\text{z} = 220 + 20$, ne 200 + 20.
> - Body se prokládají plynulou křivkou, ne lomenou čarou.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Kdy je výkon na zátěži maximální a jak se to odvodí?** Při $R_\text{z} = R_\text{i}$, z nulové derivace $P_\text{z}(R_\text{z})$.
> - **Jaká je účinnost při přizpůsobení a proč?** 50 %: stejný proud a stejný odpor, na $R_\text{i}$ se ztratí stejný výkon, jaký dostane zátěž.
> - **Proč účinnost nezávisí na $U_\text{i}$?** Výkon zátěže i zdroje jsou úměrné $U_\text{i}^2$, poměr ne.
> - **Co je nepřímá metoda měření výkonu?** Změří se $U$ a $I$ a výkon se spočítá jako $UI$, bez wattmetru.
> - **Dá se účinnost ověřit z měření?** Ano, $\eta = U_\text{z}/U_\text{i}$; při 220 Ω je to 5,98/12,0 ≈ 50 %.
> - **Proč $U_\text{z}/I_\text{z}$ nevychází přesně nastavené $R_\text{z}$?** Tolerance dekády a přesnost multimetrů (rozdíl pod 1 %).
