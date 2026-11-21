+++
title = "Praktikum: bezpečnost a laboratorní řád"
topics = ["Praktikum: bezpečnost a laboratorní řád"]
+++

Co je potřeba vědět před prvním měřením: laboratorní řád, zásady zapojování obvodů, úraz elektrickým proudem a zvláštní pravidla laserové a dozimetrické laboratoře. Shrnuto vlastními slovy z řádů Ústavu fyziky a kapitoly 5 skripta *Úvod do měření*. Závazné je znění řádů vyvěšené v laboratoři a pokyny vyučujícího.

## Než přijdete poprvé {#pred-prvnim}

- Na úvodním cvičení vás seznámí s **laboratorním řádem**, zásadami bezpečné práce, **první pomocí při úrazu elektrickým proudem** a předpisy pro práci s **lasery** a **ionizujícím zářením**.
- **Do konce 3. týdne prvního semestru** musíte mít složenou **zkoušku z bezpečnostních předpisů** a z první pomoci při úrazech elektřinou (podle směrnice děkana). Potvrzení musí být v informačním systému.
- Za bezpečnost a provoz laboratoře během cvičení odpovídá vyučující, který cvičení vede.

## Laboratorní řád v kostce {#rad}

1. **Choďte včas.** Zpoždění o víc než 5 minut se může počítat jako neúčast a pak musíte cvičení nahradit.
2. Měřit smíte, jen když jste **řádně připravení** a odevzdali jste **protokol z minulého cvičení**.
3. Za přístroje během cvičení **ručíte**. Škodu způsobenou nedbalostí, nepřipraveností nebo nedodržením pokynů platíte sami, stejně jako ztráty. Převzetí inventáře úlohy potvrdíte na začátku podpisem, vrácení podepisuje vyučující. **Odejít smíte až potom.**
4. Dokumentaci o měření vede **každý sám**. Opisovat výsledky je zakázáno. Po skončení měření a zápisů zavolejte vyučujícího k **ještě zapojené** úloze. Zkontroluje ji i naměřené hodnoty a potvrdí to podpisem.
5. **Zapojení elektrického obvodu musí schválit vyučující dřív, než ho připojíte k jakémukoli zdroji.** Chrání to vás i přístroje. Kdo to poruší, riskuje, že bude platit škodu.
6. Pracujte na **svém pracovišti**, nechoďte k jiným úlohám a nerušte ostatní. S dotazem se obraťte na vyučujícího.
7. **Každou závadu** hned ohlaste vyučujícímu.
8. Zařízení laboratoře se nesmí poškozovat.
9. Kdo poruší laboratorní řád, tomu se cvičení, ve kterém přestupek udělal, **neuzná**.

Obecné zásady k tomu: řiďte se bezpečnostními a informačními tabulkami, nepoužívejte přístroj, jehož obsluhu neznáte, s horkými předměty manipulujte jen vhodnými pomůckami a **každý úraz hned hlaste** vyučujícímu.

## Zapojování obvodů {#zapojovani}

Základem je, že rozumíte schématu a tomu, co se v obvodu děje. Pro začátek platí:

- **Zdroje** (síť, akumulátor, výstup transformátoru, elektronického zdroje, generátoru) připojte nejdřív **jen jednou svorkou, tou uzemněnou**. Druhou, „živou“ svorku nechte na zdroji odpojenou. Síťovou vidlici nezasouvejte.
- Zapojujte od zdroje nejsilnějšího nebo nejméně citlivého na přetížení. Jděte nejkratší cestou od svorky ke svorce. V uzlu nezačínejte zapojovat všechny větve najednou: nejdřív hlavní obvod, potom postupně boční větve.
- **Měřidla přepněte na nejvyšší rozsah** a na vyšší rozsah je vraťte při každé změně zapojení nebo podmínek. Ověřte, že nepřekročíte dovolené napětí a proud žádného prvku.
- **Regulační prvky** (regulační transformátor, potenciometr, reostat) nastavte před připojením na **minimum** a znovu před každou změnou zapojení.
- U dvojice přívodů rozlišujte **živý** a **zemněný** vodič. Živý vodič nesmí přes zemnicí kolík zásuvky jiného přístroje skončit zkratovaný se zemí.
- Elektronické přístroje potřebují **5 až 10 minut** na ustálení. Časté zapínání a vypínání jim škodí, osciloskop proto nevypínejte, jen stáhněte jas, když neměříte.
- Hotové zapojení zkontrolujte celé a připojte ke zdroji **až se souhlasem vyučujícího**.
- Počítejte s tím, že se kromě prvků ze schématu projeví i odpory a indukčnosti vodičů, přechodové odpory spojů, vnitřní odpory zdrojů a měřidel a parazitní termoelektrická napětí.

## Úraz elektrickým proudem {#uraz}

O účinku na člověka **rozhoduje proud, který projde tělem, ne napětí**. Nebezpečné napětí proto není jedno číslo: podle okolností to může být 200 V, 100 V i jen 50 V. Záleží na:

- **odporu kůže** v místě dotyku (několik kΩ, s vlhkostí kůže klesá),
- **objemovém odporu těla** (kudy proud teče, jak daleko jsou místa dotyku),
- **časovém průběhu proudu**: střídavý proud 50 Hz je mnohem nebezpečnější než stejnosměrný stejné velikosti, proudy vyšších kmitočtů škodí méně.

Za bezpečný se považuje **stejnosměrný proud do 10 mA** a **střídavý proud do 100 Hz do 3,5 mA**.

> [!key] První pomoc: pořadí kroků
> 1. **Vyprostit postiženého** z dosahu proudu tak, aby proud nezasáhl vás: vypnout obvod, odtáhnout postiženého nebo odsunout vodič **suchým izolantem** (v krajním případě rukou obalenou několika vrstvami suché látky).
> 2. **Oživovat**: nedýchá-li, nezdržujte se ošetřováním jiných zranění a začněte s resuscitací (stlačování hrudníku a vdechy v poměru 30 : 2).
> 3. **Přivolat lékaře** (záchranná služba 155, tísňová linka 112).
> 4. **Ohlásit úraz** odpovědné osobě (vyučujícímu).

> [!note] Resuscitace podle současných doporučení
> Úvod popisuje starší postup (hloubka stlačení 4 až 5 cm). Současná doporučení první pomoci uvádějí stlačování uprostřed hrudníku do hloubky **5 až 6 cm** rychlostí **100 až 120 za minutu**, v poměru 30 stlačení na 2 vdechy (bez vdechů stačí nepřetržitá masáž), a pokračovat až do příjezdu záchranky. Pomoc je tím úspěšnější, čím dřív začne.

## Laserová laboratoř {#laser}

Laser vyzařuje intenzivní záření ve viditelné i neviditelné oblasti. Škodí hlavně **tepelnými účinky**: poraní kůži, oči i hlouběji uložené tkáně. Citlivost je individuální.

- Do laboratoře smí jen ten, kdo v ní měří. Ostatní jen na vyzvání vyučujícího.
- **Laser zapíná a vypíná jen vyučující.** Studenti obsluhují jen ty ovládací prvky, které jsou k úloze potřeba.
- Oblast s nebezpečným paprskem je ohraničená zábranou (dráha paprsku je zakrytá). Přístup je jen k ovládacím prvkům.
- Paprsek nesmí dopadnout na sítnici. Oko poraní i **difuzně odražený** paprsek, proto si před měřením **sundejte lesklé předměty** (hodinky, prsteny).
- **Stolní lampa** pro nepřímé osvětlení přístrojů musí svítit po celou dobu.
- Při poruše nebo nejasnosti zavolejte vyučujícího.

## Dozimetrická laboratoř {#dozimetrie}

- Používají se **jen uzavřené zářiče** s platným povolením hygieny, a jen v dozimetrické laboratoři. Jiné pokusy se v ní současně nedělají.
- Se zářiči smí pracovat jen osoby **starší 18 let**, které jsou k tomu způsobilé. Kdo má zdravotní důvod, ohlásí ho vedoucímu cvičení s lékařským potvrzením. **Těhotné ženy se zářiči pracovat nesmějí.**
- Studenti musí být předem prokazatelně seznámeni s postupem a s manipulací se zářiči.
- Zářiče jsou v **uzamčeném trezoru** nebo uzamčených olověných skříňkách. **Vydává a uschovává je vyučující**, každé použití se zapisuje do záznamníku.
- V laboratoři je zakázáno **jíst, pít a kouřit**. Po práci se zářiči si umyjte ruce pod tekoucí vodou.
- Při ztrátě zářiče se po něm okamžitě pátrá (Geigerův–Müllerův počítač) a informuje se vedoucí ústavu.
- Laboratoř opouštíte, až když **všichni doměřili**, vrátili zářiče a vyučující je zamkl do trezoru.

> [!trap] Na co se často zapomíná
> - Zapojení schvaluje vyučující **před** připojením ke zdroji, ne až když něco nefunguje.
> - Měřidla začínají na **nejvyšším** rozsahu, regulátory na **minimu**.
> - Na kontrolu se volá k úloze, která je **ještě zapojená**.
> - U laseru vadí i lesklé hodinky: nebezpečný je i odražený paprsek.
