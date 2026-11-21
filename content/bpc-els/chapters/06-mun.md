+++
title = "Metoda uzlových napětí (MUN)"
topics = ["Metoda uzlových napětí"]
+++

Druhá univerzální metoda, „zrcadlová“ k MSP. Neznámé jsou **uzlová napětí** proti zvolenému referenčnímu uzlu, rovnice jsou 1. Kirchhoffovy zákony v uzlech. Matice vodivostí se opět píše rovnou ze schématu. Procvičuje se ve **VC2**; metoda razítek (VC5) na MUN přímo navazuje.

## Princip

- MUN vychází z **1. Kirchhoffova zákona**.
- V obvodu smějí být jen **zdroje proudu**. Reálné zdroje napětí se převedou: $I_\text{z} = U_\text{z}/R_\text{i}$ s paralelní vodivostí $G_\text{i} = 1/R_\text{i}$. Ideální zdroj napětí bez sériového odporu převést nejde (řeší ho modifikovaná MUN, nebo se referenční uzel zvolí na jeho svorce).
- Řeší se maticová rovnice $\mathbf{G}\cdot\mathbf{U} = \mathbf{I}$.
- Počet rovnic = počet uzlů − 1 (referenční uzel má napětí 0).

> [!key] Převod zdroje napětí na zdroj proudu
> $$I_\text{z} = \frac{U_\text{z}}{R_\text{i}}, \qquad G_\text{i} = \frac{1}{R_\text{i}}$$
> Šipka $I_\text{z}$ míří k té svorce, která byla u zdroje napětí **kladná** (šipka napětí míří od + k −, takže šipka proudu míří proti ní).

## Přímé sestavení matice

1. Zvol **referenční uzel 0** (obvykle ten, ke kterému vede nejvíc větví, nebo dolní vodič) a očísluj ostatní uzly 1, 2, …
2. Vyznač uzlová napětí $U_{10}, U_{20}, \dots$, šipky míří od uzlu k referenčnímu uzlu.
3. Přepočti odpory na vodivosti, $G = 1/R$.
4. Zapiš matici vodivostí:
   - **diagonála** $G_{kk}$: součet vodivostí všech prvků připojených k uzlu $k$, znaménko **+**,
   - **mimo diagonálu** $G_{jk}$: vodivost mezi uzly $j$ a $k$ se znaménkem **−**; matice je symetrická.
5. Zapiš vektor proudů zdrojů: proud zdroje, který do uzlu **vtéká**, se píše **+**, který **vytéká**, **−**.
6. Vyřeš soustavu (Cramerovo pravidlo: $U_{k0} = \Delta_k/\Delta$).
7. Dopočítej napětí větví (rozdíly uzlových napětí, $U_{12} = U_{10} - U_{20}$) a proudy ($I = G\,U$).

> [!trap] Vzájemná vodivost
> Mimo diagonálu je jen vodivost prvku, který leží **přímo mezi** dvěma uzly. Když uzly nespojuje žádný prvek, je tam nula. Vnitřní vodivost ideálního zdroje proudu je nulová a do matice nic nepřidá.

> [!note] Proč mS a mA
> Vodivosti v mS a proudy v mA dají napětí přímo ve voltech (mA / mS = V) a čísla v matici jsou kolem jednotek. Nic se tím nemění, jen se nemusí psát mocniny deseti.

## Vzorový příklad z prezentace

Stejný obvod jako u MSP: zdroj proudu $I_\text{Z1}$ = 200 mA s paralelním $R_1$ = 2,4 kΩ, za ním $R_3$ = 1,6 kΩ, příčný $R_4$ = 5 kΩ a větev $R_2$ = 6 kΩ se zdrojem $U_\text{Z2}$ = 300 V (šipka nahoru, kladná svorka dole). Určete proudy rezistorů.

**Převod zdroje napětí:** $I_\text{Z2} = U_\text{Z2}/R_2 = 300/6000 = 50$ mA, paralelně $G_2$. Kladná svorka $U_\text{Z2}$ je dole, šipka $I_\text{Z2}$ tedy míří dolů, ven z uzlu 2.

![Obvod po převodu zdroje](images/mun-priklad.svg "Uzel 1: Iz1, G1, G3; uzel 2: G3, G4, G2, Iz2; referenční uzel 0 dole")

Vodivosti: $G_1 = 1/2400 = 0{,}4167$ mS, $G_2 = 1/6000 = 0{,}1667$ mS, $G_3 = 1/1600 = 0{,}625$ mS, $G_4 = 1/5000 = 0{,}2$ mS.

> [!example] Sestavení matice
> Uzel 1: $G_1$, $G_3$; zdroj $I_\text{Z1}$ míří dolů, ven z uzlu: $-I_\text{Z1}$.
>
> Uzel 2: $G_3$, $G_4$, $G_2$; zdroj $I_\text{Z2}$ ven z uzlu: $-I_\text{Z2}$.
> $$\begin{bmatrix} G_1 + G_3 & -G_3 \\ -G_3 & G_2 + G_3 + G_4 \end{bmatrix}\cdot\begin{bmatrix} U_{10} \\ U_{20} \end{bmatrix} = \begin{bmatrix} -I_\text{Z1} \\ -I_\text{Z2} \end{bmatrix}, \qquad \begin{bmatrix} 1{,}0417 & -0{,}625 \\ -0{,}625 & 0{,}9917 \end{bmatrix}\cdot\begin{bmatrix} U_{10} \\ U_{20} \end{bmatrix} = \begin{bmatrix} -200 \\ -50 \end{bmatrix}$$
> (vodivosti v mS, proudy v mA, napětí vyjdou ve V)

> [!example] Řešení
> $$\Delta = 1{,}0417\cdot0{,}9917 - 0{,}625^2 = 1{,}03305 - 0{,}390625 = 0{,}64243$$
> $$\Delta_1 = \begin{vmatrix} -200 & -0{,}625 \\ -50 & 0{,}9917 \end{vmatrix} = -198{,}34 - 31{,}25 = -229{,}59, \qquad \Delta_2 = \begin{vmatrix} 1{,}0417 & -200 \\ -0{,}625 & -50 \end{vmatrix} = -52{,}085 - 125 = -177{,}085$$
> $$U_{10} = \frac{-229{,}59}{0{,}64243} = -357{,}38\ \text{V}, \qquad U_{20} = \frac{-177{,}085}{0{,}64243} = -275{,}65\ \text{V}$$

> [!example] Proudy rezistorů
> Šipky jako u MSP: $I_{R1}$ a $I_{R4}$ nahoru, $I_{R3}$ a $I_{R2}$ doleva. Napětí ve V děleno odporem v kΩ dá mA.
> $$I_{R1} = \frac{-U_{10}}{R_1} = \frac{357{,}38}{2{,}4} = 148{,}9\ \text{mA}, \quad I_{R3} = \frac{U_{20} - U_{10}}{R_3} = \frac{81{,}73}{1{,}6} = 51{,}08\ \text{mA}, \quad I_{R4} = \frac{-U_{20}}{R_4} = \frac{275{,}65}{5} = 55{,}13\ \text{mA}$$
> **Pozor na $R_2$:** proud rezistorem $R_2$ v náhradním obvodu **není** proud $R_2$ v původním obvodu. Dopočítá se z 1. KZ pro uzel 2 původního obvodu:
> $$I_{R3} - I_{R4} - I_{R2} = 0 \quad\Rightarrow\quad I_{R2} = 51{,}08 - 55{,}13 = -4{,}05\ \text{mA}$$
> nebo z 2. KZ v původním obvodu: $U_{R2} = -U_{20} - U_\text{Z2} = 275{,}65 - 300 = -24{,}35$ V a $I_{R2} = -24{,}35/6 = -4{,}06$ mA. Rozdíl v poslední číslici je zaokrouhlení vodivostí na čtyři číslice.
>
> Výsledky souhlasí s MSP (148,9; 51,08; 55,14; −4,054 mA).
>
> **Jak to říct, když se zeptají:** „Zdroj napětí s $R_2$ jsem převedl na zdroj proudu 50 mA s paralelní $G_2$. Na diagonále je součet vodivostí připojených k uzlu, mimo diagonálu záporná vodivost mezi uzly. Proudy zdrojů, které z uzlu vytékají, píšu se znaménkem minus. Proud $R_2$ v původním obvodu dopočítám z Kirchhoffova zákona, protože převodem se jeho větev změnila.“

## MSP, nebo MUN?

| | MSP | MUN |
|---|---|---|
| zákon | 2. KZ (smyčky) | 1. KZ (uzly) |
| neznámé | smyčkové proudy | uzlová napětí |
| zdroje | jen napětí | jen proudu |
| počet rovnic | počet ok | uzly − 1 |
| matice | odporů, $\mathbf{R}\,\mathbf{I} = \mathbf{U}$ | vodivostí, $\mathbf{G}\,\mathbf{U} = \mathbf{I}$ |

Vybírá se metoda s **menším počtem rovnic**, případně ta, která nevyžaduje převod ideálních zdrojů. Třeba obvod VC2 příkladu 4 (trojúhelník se třemi zdroji napětí) má tři oka, ale jen tři uzly, takže MUN vede na soustavu 2 × 2.

Příklady VC2 jsou i s postupem v kvízu **vc2**.
