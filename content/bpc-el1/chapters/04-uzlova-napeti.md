+++
title = "Úloha 4: Metoda uzlových napětí (MUN)"
topics = ["Metoda uzlových napětí"]
+++

Obvod s jedním zdrojem proudu a pěti rezistory se vyřeší metodou uzlových napětí a výsledek se ověří měřením. Hodnoty: $I_\text{z}$ = 80 mA (ideální zdroj proudu), $R_1$ = 100 Ω, $R_2$ = 150 Ω, $R_3$ = 220 Ω, $R_4$ = 330 Ω, $R_5$ = 100 Ω. Celá domácí příprava je výpočet.

![Obvod pro MUN](images/mun-obvod.svg "Uzly 0 až 3, uzlová napětí U10, U20, U30 míří k referenčnímu uzlu 0")

## Princip

MUN vychází z **1. Kirchhoffova zákona** pro nezávislé uzly. Neznámé jsou **uzlová napětí** proti zvolenému **referenčnímu uzlu 0**. Proud každého rezistoru se vyjádří přes uzlová napětí a vodivost ($I = G\,U$). Pro $n$ uzlů vyjde $n - 1$ rovnic, tady 3.

Postup:

1. V obvodu smí být jen **zdroje proudu**; reálné zdroje napětí se převedou ($I_\text{i} = U_\text{i}/R_\text{i}$, $G_\text{i} = 1/R_\text{i}$).
2. Zvolit referenční uzel 0, očíslovat uzly 1, 2, 3; šipky uzlových napětí míří k uzlu 0.
3. Pro každý uzel napsat 1. KZ, proudy zdrojů na pravou stranu.
4. Vyřešit soustavu. Větvová napětí jsou rozdíly uzlových ($U_{12} = U_{10} - U_{20}$).

Maticově $\mathbf{G}\cdot\mathbf{U} = \mathbf{I}_\text{z}$ a matici jde psát rovnou ze schématu:

> [!key] Pravidla pro sestavení matice
> - **diagonála:** součet vodivostí všech prvků připojených k uzlu, znaménko **+**,
> - **mimo diagonálu:** vodivost mezi dvěma uzly, znaménko **−** (matice je symetrická),
> - **vektor proudů:** proudy zdrojů do uzlu **+**, z uzlu **−**.

Pro tento obvod (uzly 1–2 spojuje $R_1$, 1–3 $R_3$, 2–3 $R_2$, k zemi vedou $R_4$ z uzlu 2 a $R_5$ z uzlu 3; zdroj $I_\text{z}$ teče do uzlu 1):

$$\begin{bmatrix} G_1+G_3 & -G_1 & -G_3 \\ -G_1 & G_1+G_2+G_4 & -G_2 \\ -G_3 & -G_2 & G_2+G_3+G_5 \end{bmatrix} \cdot \begin{bmatrix} U_{10} \\ U_{20} \\ U_{30} \end{bmatrix} = \begin{bmatrix} I_\text{z} \\ 0 \\ 0 \end{bmatrix}$$

Řádek 1 je třeba 1. KZ pro uzel 1: $G_1(U_{10}-U_{20}) + G_3(U_{10}-U_{30}) = I_\text{z}$, po roznásobení $(G_1+G_3)U_{10} - G_1 U_{20} - G_3 U_{30} = I_\text{z}$.

## Domácí příprava: řešení soustavy {#reseni}

Vodivosti v mS (1/Ω = S, takže 1/(100 Ω) = 10 mS), aspoň na 4 platné číslice:

$$G_1 = 10{,}000,\quad G_2 = 6{,}667,\quad G_3 = 4{,}545,\quad G_4 = 3{,}030,\quad G_5 = 10{,}000\ \text{mS}$$

Po dosazení (mS · V = mA, takže proud se píše v mA a napětí vyjde ve V):

$$\begin{bmatrix} 14{,}545 & -10{,}000 & -4{,}545 \\ -10{,}000 & 19{,}697 & -6{,}667 \\ -4{,}545 & -6{,}667 & 21{,}212 \end{bmatrix} \cdot \begin{bmatrix} U_{10} \\ U_{20} \\ U_{30} \end{bmatrix} = \begin{bmatrix} 80 \\ 0 \\ 0 \end{bmatrix}$$

Soustava se řeší **Cramerovým pravidlem**: $U_{k0} = D_k / D$, kde $D$ je determinant matice a $D_k$ determinant matice, v níž je $k$-tý sloupec nahrazený vektorem pravé strany. Determinant 3×3 se počítá **rozvojem podle 1. řádku** přes subdeterminanty 2×2 (škrtnout řádek a sloupec prvku, znaménka + − +). Mezivýsledky na 3 desetinná místa, přímo s čísly z matice.

> [!example] Výpočet krok za krokem
> **Subdeterminanty** (vždy z 2. a 3. řádku):
> $$M_{11} = \begin{vmatrix} 19{,}697 & -6{,}667 \\ -6{,}667 & 21{,}212 \end{vmatrix} = 19{,}697\cdot21{,}212 - (-6{,}667)(-6{,}667) = 417{,}813 - 44{,}449 = 373{,}364$$
> $$M_{12} = \begin{vmatrix} -10{,}000 & -6{,}667 \\ -4{,}545 & 21{,}212 \end{vmatrix} = -212{,}120 - 30{,}302 = -242{,}422$$
> $$M_{13} = \begin{vmatrix} -10{,}000 & 19{,}697 \\ -4{,}545 & -6{,}667 \end{vmatrix} = 66{,}670 + 89{,}523 = 156{,}193$$
> **Determinant** (rozvoj podle 1. řádku):
> $$D = 14{,}545\cdot373{,}364 - (-10{,}000)(-242{,}422) + (-4{,}545)\cdot156{,}193 = 5430{,}579 - 2424{,}220 - 709{,}897 = 2296{,}462$$
> **$D_1$, $D_2$, $D_3$:** nahrazený sloupec je (80; 0; 0), nuly nic nepřidají a zbude vždy 80 krát subdeterminant téhož prvku 1. řádku (se znaménkem jeho pozice):
> $$D_1 = 80\cdot M_{11} = 29\,869{,}12, \qquad D_2 = -80\cdot M_{12} = 19\,393{,}76, \qquad D_3 = 80\cdot M_{13} = 12\,495{,}44$$
> **Uzlová napětí:**
> $$U_{10} = \frac{29\,869{,}12}{2296{,}462} = 13{,}007\ \text{V}, \quad U_{20} = \frac{19\,393{,}76}{2296{,}462} = 8{,}445\ \text{V}, \quad U_{30} = \frac{12\,495{,}44}{2296{,}462} = 5{,}441\ \text{V}$$
> **Větvová napětí:** $U_{12} = 13{,}007 - 8{,}445 = 4{,}562$ V, $U_{23} = 8{,}445 - 5{,}441 = 3{,}004$ V.
>
> **Zkouška** (1. rovnice musí dát 80 mA): $14{,}545\cdot13{,}007 - 10{,}000\cdot8{,}445 - 4{,}545\cdot5{,}441 = 189{,}187 - 84{,}450 - 24{,}729 = 80{,}008 \approx 80$ mA.
>
> **Jak to říct, když se zeptají:** „Soustavu $\mathbf{G}\mathbf{U} = \mathbf{I}$ řeším Cramerovým pravidlem. $U_{10}$ je podíl determinantu matice s 1. sloupcem nahrazeným vektorem proudů a determinantu matice $\mathbf{G}$. Determinanty počítám rozvojem podle 1. řádku. Vektor proudů má jediné nenulové číslo 80 mA, takže každý $D_k$ je jen 80 krát jeden subdeterminant 2×2.“

S nezaokrouhlenými vodivostmi by vyšlo $U_{10}$ = 13,006 V; rozdíl 0,001 V je jen zaokrouhlení vodivostí a proti přesnosti měření je zanedbatelný. Do tabulky patří hodnota z vlastní matice. Stejně dobře jde soustava vyřešit programem LinRov nebo kalkulačkou (režim EQN), Sarrusovým pravidlem nebo Gaussovou eliminací.

Fyzikální kontrola: $U_{10} > U_{20} > U_{30} > 0$, proud teče ze zdroje do uzlu 1 a přes rezistory k zemi. Vstupní odpor ze svorek zdroje je $U_{10}/I_\text{z}$ ≈ 162,6 Ω.

## Měření

1. Zdroj musí pracovat jako **zdroj proudu** (režim CC). Napětí na něm bude asi 13 V, napěťový limit tedy nastavit výš (15 až 20 V); když svítí CV, je limit moc nízko.
2. Podle miliampérmetru nastavit $I_\text{z}$ = 80 mA.
3. Přepojovat voltmetr a měřit $U_{10}$, $U_{20}$, $U_{30}$, $U_{12}$, $U_{23}$ (+ na prvním indexu).
4. Kontrola: $U_{10} - U_{20} \approx U_{12}$, $U_{20} - U_{30} \approx U_{23}$. Rezistory topí (až asi 0,3 W), obvod nenechávat zbytečně dlouho pod proudem.

Kdyby na pracovišti zadali jiný proud, nic se nepřepočítává: obvod je lineární, všechna napětí se vynásobí poměrem $I_\text{z}/80$ mA.

> [!trap] Na co si dát pozor
> - Diagonála +, mimo diagonálu −; vzájemná vodivost je jen vodivost **mezi** dvěma uzly.
> - $G_{11} = G_1 + G_3$: ideální zdroj proudu má $G_\text{i} = 0$, do matice nic nepřidá.
> - Vodivosti zaokrouhlené na 2 platné číslice dají znatelně jiný výsledek, determinant malé chyby zesiluje.
> - Znaménko $D_2$: prvek na pozici (1, 2) má v rozvoji znaménko −.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Na čem je MUN založena?** Na 1. KZ v nezávislých uzlech; neznámé jsou uzlová napětí proti referenčnímu uzlu.
> - **Kolik rovnic?** Počet uzlů − 1, tady 4 − 1 = 3.
> - **Co se zdrojem napětí?** Reálný se převede na zdroj proudu ($U_\text{i}/R_\text{i}$, $1/R_\text{i}$); ideální zdroj napětí převést nejde, na to je modifikovaná MUN.
> - **Proč ampérmetr neovlivní měřená napětí?** Je v sérii se zdrojem proudu, který si proud vnutí bez ohledu na odpor ve větvi.
> - **Jak zkontrolovat výsledek?** Dosazením do rovnic, 1. KZ z vypočtených proudů, nebo bilancí výkonů $I_\text{z}U_{10} = \sum U^2/R$.
