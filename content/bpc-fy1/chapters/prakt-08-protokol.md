+++
title = "Praktikum: příprava, záznam a protokol"
topics = ["Protokol o měření"]
+++

Každá úloha má tři části: **přípravu** doma, **záznam o měření** v laboratoři a **protokol**, který odevzdáte na dalším cvičení. Kapitola shrnuje kapitoly 5 a 6 skripta *Úvod do měření* a hlavičku protokolu. Když návod k úloze nebo vyučující určí jinak, řiďte se jimi.

## Příprava na měření {#priprava}

Bez přípravy se úloha dělá mechanicky podle návodu, bez pochopení, a roste riziko, že poškodíte přístroje. U elektrických úloh hrozí i úraz. Příprava se kontroluje: kdo není připravený, neměří v řádném termínu.

- **Ujasněte si cíl**: co přesně má měření zjistit. To je nejdůležitější část přípravy.
- Nastudujte teorii v návodu (a v Úvodu do měření metody a výpočet chyb) tak, abyste věděli, **jaký výsledek čekat**.
- Ze vztahů vypište, které veličiny se budou **měřit přímo**.
- Proměřujete-li závislost, odhadněte předem **rozsah hodnot**, **počet a rozložení bodů** a kde je potřeba měřit hustěji.
- Splňte **vstupní test** v e-learningu.
- Nejasnosti konzultujte s vyučujícím **předem**.

## V laboratoři: záznam o měření {#zaznam}

- Nejdřív zkontrolujte, že jsou připravené všechny přístroje, seznamte se s nimi a zapište, které **skutečně** používáte (případně upravte tabulky). Úloha se oproti návodu mohla mezitím změnit. Lepší postup měření můžete s vyučujícím prodiskutovat.
- Obvod zapojte podle schématu a **nechte zapojení zkontrolovat** dřív, než připojíte zdroj.
- Z předběžného rozboru přesnosti plyne, jak přesně je potřeba měřit jednotlivé veličiny. Běžná laboratorní měření se žádají s přesností **do 1 %**, někdy do 5 %. Orientační měření do 10 %.
- **Záznam pište rovnou v konečné podobě**, současně s měřením, do sešitu nebo na předepsaný list. Přepisování „načisto“ je zbytečné. Záznam podepisuje vyučující.
- V záhlaví: **název a číslo úlohy, datum**. Dál všechno, co je potřeba ke zpracování: naměřené hodnoty s jednotkami, **čísla vzorků, seznam přístrojů s výrobními nebo evidenčními čísly**. Záznam musí být srozumitelný i po delší době a i pro někoho jiného.
- Odhadujte **desetiny nejmenšího dílku**, dejte pozor na paralaxu, u nulových metod kontrolujte nulu. Zapište všechny okolnosti, které mohly ovlivnit výsledek.
- I u jednorázových měření se určuje chyba, obvykle jako mezní odhad. Proto si u elektrických přístrojů **zapište třídu přesnosti a rozsah**.
- Na konci zavolejte vyučujícího k **ještě zapojené** úloze na kontrolu.

## Hlavička protokolu {#hlavicka}

První list protokolu má předepsanou hlavičku (razítko v laboratoři nebo šablona ke stažení). Vyplňují se všechny kolonky:

- pracoviště: Fyzikální praktikum, FEKT VUT v Brně, semestr,
- **jméno a příjmení**, **ID**, ročník, předmět, program a obor, studijní skupina (kroužek), laboratorní skupina,
- **měřeno dne**, **odevzdáno dne**, **spolupracoval** (spolužák ze skupiny),
- kolonky pro vyučujícího: příprava, opravy, učitel, hodnocení,
- **název úlohy** a **číslo úlohy**.

Pod hlavičkou následuje úkol měření.

## Co protokol obsahuje {#obsah}

1. **Vyplněnou hlavičku.**
2. **Název úlohy.**
3. **Úkol měření**: stručně, co se má měřením zjistit (podle návodu nebo zadání u úlohy).
4. **Popis metody**: teoretický základ **vlastními slovy** (žádné doslovné opisování celých pasáží návodu), vztahy potřebné ke zpracování a vysvětlení všech symbolů. Rovnice je vhodné číslovat.
5. **Popis experimentu**: schéma zapojení nebo aparatury **nakreslené podle pravítka**, seznam přístrojů a pomůcek s výrobními nebo evidenčními čísly.
6. **Naměřené hodnoty a zpracování**: tabulky naměřených i vypočtených hodnot. Každá tabulka má **číslo a název**, v záhlaví veličiny **s jednotkami**, rastr podle pravítka. U každé veličiny výpočet nebo odhad chyby.
7. **Příklad výpočtu**: každý použitý vztah s **dosazenými čísly** a hned výsledek, bez dalších mezivýpočtů. Opakuje-li se výpočet, stačí jeden příklad.
8. **Grafy**: naměřené závislosti zpracované do grafu podle zásad (popsané osy s jednotkami, body jako značky, hladká křivka, číslo a název).
9. **Zhodnocení výsledků**: výsledky s chybou přehledně pohromadě, porovnání metod z hlediska přesnosti, porovnání s tabulkovou hodnotou, možné příčiny rozdílů.

> [!key] Dosazování do vzorců
> Do rovnice se dosazují hodnoty i s jednotkami a číselné hodnoty konstant. Počítáte-li jen v SI a víte, v jaké jednotce vyjde výsledek, smíte dosadit jen čísla a jednotku připsat za výsledek. Rovnice musí být rozměrově homogenní.

## Zhodnocení: kdy výsledek souhlasí {#zhodnoceni}

Výsledek se porovná s tabulkovou (očekávanou) hodnotou. **Liší-li se o víc než trojnásobek chyby**, je měření pravděpodobně zatížené **soustavnou chybou** (nevhodná metoda, zanedbaný podstatný vliv) nebo je někde **hrubá chyba** v měření či ve výpočtu.

Nesoulad s očekávanou hodnotou není neúspěch. Je to příležitost hledat příčinu a v závěru ji rozumně zdůvodnit (jako domněnku) a navrhnout, jak měření zlepšit.

> [!example] Závěr s porovnáním
> Naměřeno $g = (9{,}79 \pm 0{,}03)\ \mathrm{m\,s^{-2}}$, $n = 5$, $P = 95\ \%$, $\delta_\mathrm{r} \doteq 0{,}3\ \%$. Tabulková hodnota pro Brno je $9{,}81\ \mathrm{m\,s^{-2}}$.
> $$\lvert 9{,}81 - 9{,}79\rvert = 0{,}02\ \mathrm{m\,s^{-2}} < 3\delta = 0{,}09\ \mathrm{m\,s^{-2}}$$
> Výsledek se s tabulkovou hodnotou shoduje. Kdyby vyšlo $g = (9{,}72 \pm 0{,}02)\ \mathrm{m\,s^{-2}}$, byl by rozdíl $0{,}09\ \mathrm{m\,s^{-2}}$ větší než $3\delta = 0{,}06\ \mathrm{m\,s^{-2}}$ a v závěru by se hledala soustavná chyba (třeba nepřesně změřená délka nebo vliv rozkmitu).
>
> **Jak to říct, když se zeptají:** „Rozdíl od tabulkové hodnoty porovnávám s trojnásobkem chyby. Ten je $0{,}09$, rozdíl jen $0{,}02$, takže v rámci přesnosti měření výsledek souhlasí.“

## Časté důvody vrácení protokolu {#chyby}

> [!trap] Na co si dát pozor
> - Chybí jednotky v záhlaví tabulek nebo na osách grafu.
> - Výsledek bez chyby, bez $n$ a $P$, nebo s chybou na tři číslice a výsledkem na víc míst, než dovolí chyba.
> - Graf jako lomená čára nebo „spojnicový“ graf místo bodového, bez čísla a názvu.
> - Teorie opsaná z návodu místo vlastních slov.
> - Chybí příklad výpočtu s dosazenými čísly.
> - V závěru chybí porovnání s tabulkovou hodnotou nebo zdůvodnění rozdílu.
> - Opisování výsledků od spolužáka je zakázané laboratorním řádem.

Protokol se odevzdává na **nejbližším následujícím cvičení**. Vrácený protokol opravte a odevzdejte znovu na dalším cvičení. Pravidla hodnocení jsou v kapitole **Praktikum: jak ho projít**.
