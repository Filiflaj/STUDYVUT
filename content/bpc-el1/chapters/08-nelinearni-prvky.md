+++
title = "Úloha 8: Lineární a nelineární prvky"
topics = ["Nelineární prvky"]
+++

Na pracovišti RC 2000 se snímají AV charakteristiky rezistoru, nelineárního prvku a jejich sériového a paralelního spojení. Kurzory se odečtou souřadnice pracovního bodu $P$ (asi 5 mA) a dvou bodů v okolí ($P_1$ ≈ 4,5 mA, $P_2$ ≈ 5,5 mA), z nich se počítá statický a dynamický odpor.

## Statický a dynamický odpor

Nelineární prvek má parametr závislý na napětí nebo proudu (dioda). Neplatí pro něj superpozice (úloha 5), a proto se obvod řeší jen **v okolí pracovního bodu** $P$.

**Statický odpor** je poměr napětí a proudu v bodě $P$, sklon spojnice počátku s $P$. Popisuje chování pro stejnosměrný proud:

$$R_\text{s} = \frac{u(P)}{i(P)}$$

**Dynamický odpor** odpovídá tečně v $P$ a popisuje chování pro malé změny kolem $P$. Počítá se ze dvou blízkých bodů:

$$R_\text{d} = \frac{\mathrm{d}u}{\mathrm{d}i}\bigg|_P \cong \frac{\Delta u}{\Delta i} = \frac{u_{P2} - u_{P1}}{i_{P2} - i_{P1}}$$

U lineárního rezistoru je $R_\text{s} = R_\text{d} = R$. U diody je $R_\text{d}$ mnohem menší než $R_\text{s}$.

![Statický a dynamický odpor, linearizovaný model](images/rs-rd.svg "Spojnice 0–P dává Rs, tečna v P dává Rd a její průsečík s osou u napětí US; vpravo linearizovaný model")

## Linearizace

V okolí $P$ se charakteristika nahradí tečnou. Modelem je zdroj napětí $U_\text{S}$ (průsečík tečny s osou $u$) v sérii s $R_\text{d}$:

$$i = \frac{u - U_\text{S}}{R_\text{d}}, \qquad U_\text{S} = u_P - R_\text{d}\,i_P$$

Model platí jen pro malé signály kolem $P$. Daleko od $P$ dává nesmysly (při $u = 0$ by vyšel velký záporný proud).

## Spojování prvků

> [!key] Grafický součet charakteristik
> **Sériově** teče stejný proud a sčítají se napětí (charakteristiky se sčítají vodorovně): $u = u_R + u_n$.
> **Paralelně** je stejné napětí a sčítají se proudy (sčítají se svisle): $i = i_R + i_n$.

Vzorec pro paralelní odpory platí jen pro lineární prvky. Nelineární prvek je v paralelním zapojení při jiném napětí, tedy v jiném pracovním bodě s jiným odporem. Správná kontrola je **součet proudů při stejném napětí**.

## Měření na RC 2000

![Princip měření na RC 2000](images/rc2000.svg "Generátor pilového napětí, snímací rezistor 100 Ω (kanál B, proud) a měřený dvojpól (kanál A, napětí)")

Generátor dává pilové napětí −5 až +5 V přes snímací rezistor $R_\text{sn}$ = 100 Ω na měřený dvojpól. Kanál A měří napětí na dvojpólu, kanál B napětí na snímacím rezistoru, z něj program počítá proud $i = u_{R\text{sn}}/100\ \Omega$. Charakteristika se tak nakreslí najednou, ne bod po bodu. Nastavení: Sense R = 100 Ω, Gain A = 2 V, Gain B = 20 mA. Propojkami se volí rezistor, nelineární prvek, sériové a paralelní spojení; grafy se tisknou a vlepují do sešitu.

> [!trap] Jednotky v sešitu
> Pracovní sešit má napětí v **mV** (skripta ve V). Výhoda: mV/mA = Ω, odpor vyjde rovnou v ohmech. Zapsané „2,050“ ve sloupci mV znamená 2050 mV.

## Příklad z měření {#priklad}

> [!example] Příklad z měření
> Nelineární prvek: $P$ = (660 mV; 5,331 mA), $P_1$ = (650 mV; 4,319 mA), $P_2$ = (670 mV; 6,590 mA).
> $$R_\text{s} = \frac{u_P}{i_P} = \frac{660\ \text{mV}}{5{,}331\ \text{mA}} = 124\ \Omega$$
> $$R_\text{d} \cong \frac{670 - 650}{6{,}590 - 4{,}319} = \frac{20\ \text{mV}}{2{,}271\ \text{mA}} = 8{,}8\ \Omega$$
> $$U_\text{S} = u_P - R_\text{d}\,i_P = 660 - 8{,}8\cdot 5{,}331 = 660 - 46{,}9 = 613\ \text{mV}$$
> Model: zdroj 613 mV v sérii s 8,8 Ω (Ω · mA = mV). Rezistor dal $R_\text{s}$ = 1380/5 = 276 Ω a $R_\text{d}$ = 280/1,0 = 280 Ω, tedy prakticky stejně (lineární prvek).
>
> Sériové spojení: při 4,5 mA naměřeno 1890 mV = 1230 mV (rezistor) + asi 660 mV (prvek). Paralelní spojení: při 650 mV teče rezistorem 650/276 = 2,36 mA a prvkem 4,319 mA, součet 6,67 mA; naměřeno 6,62 mA.
>
> **Jak to říct, když se zeptají:** „$R_\text{s}$ je sklon spojnice s počátkem, $R_\text{d}$ sklon tečny, počítám ho z bodů $P_1$, $P_2$. Napětí se odečítalo po 10 mV a $\Delta u$ bylo jen 20 mV, takže $R_\text{d}$ diody je jen orientační. Sériové zapojení kontroluji součtem napětí při stejném proudu, paralelní součtem proudů při stejném napětí.“

Zaokrouhlení: $R_\text{s}$ na 3 platné číslice, $R_\text{d}$ na 2 (rozdíly napětí 20, 30 nebo 280 mV mají jen dvě platné číslice).

> [!trap] Na co si dát pozor
> - $R_\text{d}$ se počítá z rozdílů ($P_2 - P_1$), ne jako $u_{P2}/i_{P2}$.
> - Paralelní spojení se nekontroluje vzorcem $1/(1/R_1 + 1/R_2)$, nelineární prvek je v jiném pracovním bodě.
> - Body $P_1$, $P_2$ nemusí být přesně 4,5 a 5,5 mA (strmá charakteristika, kurzor skáče); počítá se se skutečnými souřadnicemi.

## Na co se můžou zeptat

> [!note] Otázky z obhajoby
> - **Rozdíl mezi statickým a dynamickým odporem?** Statický = $u/i$ v $P$ (spojnice s počátkem, stejnosměrný proud), dynamický = $\mathrm{d}u/\mathrm{d}i$ v $P$ (tečna, malé změny).
> - **Kdy se rovnají?** U lineárního prvku vždy.
> - **Proč má dioda $R_\text{d}$ malý a $R_\text{s}$ velký?** $R_\text{s}$ je dané celým napětím asi 0,66 V, $R_\text{d}$ změnou: 20 mV navíc zvedne proud o 2,3 mA.
> - **Co znamená $U_\text{S}$?** Průsečík tečny s osou $u$; pro malé změny se prvek chová jako zdroj $U_\text{S}$ v sérii s $R_\text{d}$.
> - **Kdy model neplatí?** Daleko od $P$, např. při $u$ = 0 by dal (0 − 613)/8,8 ≈ −70 mA.
> - **Jak RC 2000 měří proud?** Z napětí na snímacím rezistoru 100 Ω.
> - **Proč se měří pilovým napětím?** Napětí se plynule mění v celém rozsahu a charakteristika se nakreslí najednou.
