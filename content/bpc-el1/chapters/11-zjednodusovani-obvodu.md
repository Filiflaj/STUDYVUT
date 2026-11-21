+++
title = "Cvičení: postupné zjednodušování obvodu"
topics = ["Zjednodušování obvodu"]
+++

Obvod s jedním zdrojem, ve kterém jdou všechny rezistory postupně sloučit jako sériové a paralelní, se řeší bez soustavy rovnic: složí se na jediný odpor, spočítá se proud ze zdroje a postupem zpět se dopočítají napětí a proudy. Je to první příklad testu v PC7.

## Spojování rezistorů a děliče

$$R_\text{s} = R_1 + R_2 + \dots, \qquad \frac{1}{R_\text{p}} = \frac{1}{R_1} + \frac{1}{R_2} + \dots, \qquad R_1 \parallel R_2 = \frac{R_1 R_2}{R_1 + R_2}$$

- **Sériově** jsou prvky, kterými teče **tentýž proud** (mezi nimi není žádná odbočka).
- **Paralelně** jsou prvky připojené **ke stejné dvojici uzlů** (mají stejné napětí).

**Dělič napětí** (nezatížený, dva rezistory v sérii na napětí $U$) a **dělič proudu** (dva rezistory paralelně, celkový proud $I$):

$$U_2 = U\,\frac{R_2}{R_1 + R_2}, \qquad I_2 = I\,\frac{R_1}{R_1 + R_2} = I\,\frac{G_2}{G_1 + G_2}$$

> [!trap] Sériově, nebo paralelně?
> Rozhoduje zapojení do uzlů, ne to, jak je obrázek nakreslený. Dva rezistory „pod sebou“ můžou být v sérii a dva „vedle sebe“ paralelně. Pomůže obarvit uzly: prvky mezi stejnými dvěma barvami jsou paralelně. U proudového děliče je v čitateli **ten druhý** odpor.

## Postup zjednodušování

1. Najít dvojici prvků v sérii nebo paralelně, nahradit ji jedním odporem, nakreslit nové schéma.
2. Opakovat, dokud nezbude zdroj a jediný odpor $R$.
3. Proud ze zdroje $I = U/R$.
4. Vracet se ke starším schématům a z Ohmova zákona, 2. KZ, 1. KZ nebo děličů dopočítat ostatní veličiny.

![Příčkový obvod](images/cv-zjednodusovani.svg "Zdroj U, podélné rezistory R1 a R3, příčné R2 a R4")

> [!example] Sbírka, př. 2.1: napětí $U_4$
> $U = 10\ \text{V}$, $R_1 = 20\ \Omega$, $R_2 = 100\ \Omega$, $R_3 = R_4 = 50\ \Omega$.
>
> Zjednodušování odzadu:
> $$R_{34} = R_3 + R_4 = 50 + 50 = 100\ \Omega$$
> $$R_{234} = \frac{R_2 R_{34}}{R_2 + R_{34}} = \frac{100\cdot100}{100 + 100} = 50\ \Omega, \qquad R = R_1 + R_{234} = 20 + 50 = 70\ \Omega$$
> Zpět:
> $$I_1 = \frac{U}{R} = \frac{10}{70} = 0{,}142\,86\ \text{A}, \qquad U_2 = U - R_1 I_1 = 10 - 20\cdot0{,}142\,86 = 7{,}143\ \text{V}$$
> $$U_4 = U_2\,\frac{R_4}{R_3 + R_4} = 7{,}143\cdot\frac{50}{100} = 3{,}571\ \text{V}$$
> (Ve cvičení PC4 a v COZ2 je tentýž obvod s $R_1 = 30\ \Omega$; tam vyjde $U_4 = 3{,}125\ \text{V}$.)
>
> **Jak to říct, když se zeptají:** „$R_3$ a $R_4$ jsou v sérii, k nim paralelně $R_2$ a k tomu v sérii $R_1$. Z celkového odporu mám proud ze zdroje, odečtu úbytek na $R_1$ a napětí $U_2$ rozdělí dělič $R_3$, $R_4$.“

## Metoda úměrných veličin

V lineárním obvodu s jedním zdrojem jsou všechny proudy a napětí úměrné napětí zdroje. Zvolí se pohodlná hodnota na konci obvodu (třeba proud nejvzdálenějším rezistorem), postupem **k zdroji** se dopočítá, jaké napětí zdroje $U'$ by jí odpovídalo, a všechno se vynásobí koeficientem $k = U/U'$. Odpadá skládání odporů. S měřením je metoda v kapitole **Úloha 2: Kirchhoffovy zákony**.

> [!example] Tentýž obvod metodou úměrných veličin
> Zvolím $U_4' = 50\ \text{V}$. Pak $I_4' = U_4'/R_4 = 1\ \text{A}$, $U_2' = (R_3 + R_4) I_4' = 100\ \text{V}$, $I_2' = U_2'/R_2 = 1\ \text{A}$, $I_1' = I_2' + I_4' = 2\ \text{A}$ a
> $$U' = U_2' + R_1 I_1' = 100 + 20\cdot2 = 140\ \text{V}, \qquad k = \frac{U}{U'} = \frac{10}{140} = 0{,}071\,43$$
> $$U_4 = k\,U_4' = 0{,}071\,43\cdot50 = 3{,}571\ \text{V}$$
>
> **Jak to říct, když se zeptají:** „Obvod je lineární, takže když zdroj zvětším $k$krát, zvětší se $k$krát všechno. Počítal jsem pozpátku s vymyšlenou hodnotou a nakonec přepočítal skutečným napětím zdroje.“

## Transfigurace hvězda – trojúhelník

Tři rezistory zapojené do **trojúhelníku** mezi uzly 1, 2, 3 jde nahradit třemi rezistory do **hvězdy** se středem 0 (a naopak), aniž by se cokoli změnilo zvenku. Pomůže to tam, kde nic není ani v sérii, ani paralelně (můstek).

![Trojúhelník a hvězda](images/cv-hvezda-trojuhelnik.svg "Trojúhelník R12, R23, R31 a ekvivalentní hvězda R10, R20, R30")

$$\Delta \to \text{Y}: \quad R_{10} = \frac{R_{12} R_{31}}{\Sigma R}, \quad R_{20} = \frac{R_{23} R_{12}}{\Sigma R}, \quad R_{30} = \frac{R_{31} R_{23}}{\Sigma R}, \qquad \Sigma R = R_{12} + R_{23} + R_{31}$$

$$\text{Y} \to \Delta: \quad R_{12} = R_{10} + R_{20} + \frac{R_{10} R_{20}}{R_{30}} \quad\text{(a cyklicky)}$$

Rezistor hvězdy u uzlu 1 je součin dvou stran trojúhelníku, které z uzlu 1 vycházejí, lomeno součtem všech tří. Pro tři stejné odpory platí $R_\text{Y} = R_\Delta/3$.

> [!example] Převod trojúhelníku
> $R_{12} = 10\ \Omega$, $R_{23} = 20\ \Omega$, $R_{31} = 30\ \Omega$, $\Sigma R = 60\ \Omega$:
> $$R_{10} = \frac{10\cdot30}{60} = 5\ \Omega, \quad R_{20} = \frac{20\cdot10}{60} = 3{,}333\ \Omega, \quad R_{30} = \frac{30\cdot20}{60} = 10\ \Omega$$

## Můstek

Ve **vyváženém** můstku (s rozložením podle obrázku v kapitole **Cvičení: věty o náhradním zdroji**) platí $R_1 R_4 = R_2 R_3$: oba děliče dávají stejné napětí a diagonálou neteče proud, takže ji jde vynechat. Nevyvážený můstek se zjednodušovat nedá bez transfigurace. Řeší se metodou smyčkových proudů, uzlových napětí nebo náhradním zdrojem.
