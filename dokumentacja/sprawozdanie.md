---
documentclass: article
papersize: a4
lang: pl
toc: t

fontsize: 8pt

geometry: margin=2cm
---

# Opis algorytmu

## Symulowane wyżarzanie

Symulowane wyżarzanie to rodzaj algorytmu heurystycznego przeszukującego przestrzeń
alternatywnych rozwiązań problemu w celu wyszukania najlepszego. Nazwa algorytmu
bierze się z metalurgii, gdzie metal jest podgrzewany i chłodzony w celu osiągnięcia
struktury krystalicznej o najmniejszej energii.

Przyjmując, że dowolny problem to funkcja matematyczna pewnego stanu $S$, szuka się
stanu, który daje najlepszy wynik (najmniejszą wartość $f(S)$). Algorytm można
przedstawić jako:

1. Wylosuj stan sąsiedni $S'$ do obecnego $S$.
2. Oblicz $f(S')$.
3. Zdecyduj, czy przyjąć stan $S'$. Jeśli nie, przejdź do kroku 1.
4. Ustaw stan $S'$ jako obecny stan. Przejdź do kroku 1.

Decyzja o przyjęciu stanu zależy od $f(S')$ oraz od temperatury. Symulowane
wyżarzanie różni się tym od algorytmu zachłannego, że przy wysokiej temperaturze
akceptuje zmianę stanu, która pogarsza wynik. Dzięki temu, algorytm nie zatrzymuje
się w minimum lokalnym.

Temperatura maleje przy każdej zmianie stanu. Przy niskiej temperaturze, algorytm
zaczyna działać jak algorytm zachłanny. Nasza implementacja algorytmu przerywa pracę,
gdy odrzucone zostanie 1.000.000 zmian stanu z rzędu.

## Mutacje

Problemem typowej implementacji algorytmu symulowanego wyżarzania do rozwiązania
problemu szukania planu lekcji jest rozmiar stanu. Typowa implementacja algorytmu
wykonuje kopię całego stanu.

Stwierdziliśmy, że kopiowanie stanu planu lekcji byłoby zbyt kosztowne. Z tego
powodu, zaimplementowaliśmy coś co nazwaliśmy "mutacjami".  Mutacja to struktura
przechowująca rodzaj zmiany stanu i pozwalająca na wygenerowanie mutacji odwrotnej,
której wykonanie przywróci stan przed oryginalną mutacją.

Losowanie stanu sąsiedniego w naszym programie polega na losowaniu mutacji. Mutacja
jest następnie wykonywana na stanie programu. Oceniana jest energia stanu po mutacji
i podejmowna jest decyzja o przyjęciu nowego stanu. Przy odrzuceniu nowego stanu,
wykonywana jest mutacja odwrotna.

## Przechowywanie stanu programu

Stan planu przechowywany jest w naszym programie za pomocą czterech struktur danych.
Są to: tablica i trzy tablice mieszające. Taka kombinacja znacznie zwiększa
skomplikowanie programu, ale przyspiesza wykonywanie mutacji. Zwykła tablica
przechowuje struktury zawierające dane o pojedynczej lekcji. Są to grupa studencka,
nauczyciel, sala lekcyjna i czas. Tablice mieszające mapują pary czasu i innych
charakterystyk do lekcji, która posiada taką kombinację czasu i charakterystyki. W
pseudokodzie można to przedstawić jako.

```
struct PlanLekcji {
    lekcje: Array<{czas: int, grupa: int, nauczyciel: int, sala: int}>,

    czas_sala: HashMap<{czas: int, sala: int}, int>,
    czas_nauczyciel: HashMap<{czas: int, nauczyciel: int}, int>,
    czas_grupa: HashMap<{czas: int, grupa: int}, int>,
}
```

## Obliczanie energii

# Bibliografia

- <http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.66.5018&rep=rep1&type=pdf> (dostęp: 2021-05-22)
- <http://arantxa.ii.uam.es/~die/[Lectura%20EDA]%20Annealing%20-%20Rutenbar.pdf>
  (dostęp: 2021-05-22)
