+++
title = "Jak na laborky a pracovní sešit"
topics = ["Pracovní sešit"]
+++

Laboratorní cvičení z EL1 se nezpracovávají jako protokol na počítači, ale **ručně do pracovního sešitu** (PS). Návody jsou ve skriptech LC. Sešit má pro každou úlohu předtištěné zadání, tabulky a místo na teorii, schéma, výpočty, graf a závěr.

## Tři fáze každé úlohy

| Kdy | Co se dělá |
|---|---|
| **doma, před cvičením** | číslo a název úlohy, zadání (v PS předtištěné), stručný výtah z teorie (vztahy a principiální schémata, ne opis celého úvodu), skutečné schéma zapojení tužkou a pravítkem, u některých úloh předem spočítané teoretické hodnoty |
| **v laboratoři** | naměřené hodnoty rovnou do tabulek, seznam přístrojů, kontrola několika hodnot, podpis učitele |
| **po cvičení** | pod každou tabulku obecný vztah a příklad výpočtu jednoho řádku, grafy, závěr |

> [!key] Bez přípravy se neměří
> Domácí příprava je podmínka vstupu do laboratoře. U úloh 3, 4 a 7 je její hlavní částí výpočet (Théveninovy parametry, matice MUN, indukce B a B̃).

V tabulkách platí: **bílé buňky se měří, šedé se počítají.**

## Zápis naměřených hodnot

- Hodnoty se píšou **rovnou do sešitu** a nepřepisují se. Chybný zápis se nechá, škrtne se jen jasný překlep ve vypočtené (šedé) buňce.
- **Digitální přístroj:** zapisují se všechny zobrazené číslice, nezaokrouhluje se, ani koncové nuly se nevynechávají.
- **Analogový přístroj:** zapisuje se výchylka $\alpha$ (zaokrouhlená na půl dílku) a konstanta $k$ jako nezkrácený zlomek *rozsah / počet dílků*, např. $12/120$. Hodnota se pak dopočítá:

$$X = \alpha \cdot k, \qquad k = \frac{X_\text{R}}{\alpha_\text{max}}$$

- **Rozsah** se volí nejbližší vyšší, než je měřená hodnota, aby byla ručka co nejvíc vpravo (v poslední třetině stupnice). Při přepínání se začíná na nejvyšším rozsahu.
- **Seznam přístrojů:** typ a výrobní číslo, u analogových přístrojů navíc soustava, rozsah a třída přesnosti.

> [!note] Jak to říct, když se zeptají
> „$k$ píšu jako zlomek 12/120, ne 0,1, aby ze zápisu šlo zpětně poznat použitý rozsah (12 V).“ „Ručku držím v poslední třetině stupnice, protože chyba analogového přístroje je daná z rozsahu (třída přesnosti), takže relativní chyba je nejmenší při velké výchylce.“

## Výpočty a zaokrouhlování

Relativní odchylka měřené hodnoty od teoretické (vztah A.1.1 ve skriptech):

$$\delta = \frac{X_\text{měř} - X_\text{teor}}{X_\text{teor}} \cdot 100\ \%$$

- Vypočtené hodnoty mají **stejný počet míst jako měřené**.
- Odchylka se zaokrouhluje **nahoru** na 1 až 2 platné číslice a nechává se jí znaménko.
- Ke každé tabulce stačí **obecný vztah a jeden dosazený příklad**, ostatní řádky se jen zapíšou.
- Počítá se standardním postupem z přednášek a s čísly tak, jak jsou zapsaná v sešitu (včetně jejich zaokrouhlení). Postup musí jít obhájit u učitele.

> [!trap] Pozor na jednotky
> V/mA = kΩ, mA/V = mS, mV/mA = Ω, V·mA = mW. Kdo dělí volty miliampéry a výsledek píše v ohmech, je o tři řády vedle.

## Grafy

Tužkou, s popsanými osami (veličina a jednotka), názvem a zřetelně odlišenými křivkami (kroužky, křížky, legenda). Měřítko se volí tak, aby se vešly všechny body a graf zabral co největší plochu. Mřížky v sešitu mají 15 × 11 (nebo 15 × 10) velkých dílků po 5 × 5 čtverečcích. Body se prokládají přímkou nebo plynulou křivkou, nespojují se lomenou čarou.

## Závěr

Krátký, asi tři věty:

1. jestli se metoda nebo zákon potvrdily a k čemu se hodí,
2. největší odchylka (číslo),
3. příčiny odchylek (tolerance rezistorů, vnitřní odpor ampérmetru, přesnost přístrojů, odečítání stupnice).

> [!trap] Časté chyby
> - Přepsaná nebo „učesaná“ naměřená hodnota. Měřené se nemění, chyba se vysvětlí v závěru.
> - Zaokrouhlené údaje z multimetru (12,0 místo 12,03).
> - Chybějící obecný vztah nad dosazeným výpočtem.
> - Bod (0; 0) nebo jiný chybný zápis zakreslený do grafu bez komentáře.
> - Rozpojený obvod dřív, než učitel podepíše hodnoty.
