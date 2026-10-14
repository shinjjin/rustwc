# rustwc — TODO: vom Lernprojekt zum echten `wc`-Ersatz

Ziel: `rustwc` soll für dieselben Aufrufe **dieselbe Ausgabe** liefern wie das
System-`wc` — inklusive Binärdateien, mehreren Dateien und Exit-Codes.

Test-Orakel ist immer das echte Tool:
```
wc -l -w -c -m DATEI
```

---

## 0. Stand jetzt (erledigt)
- [x] Argumente auslesen (`std::env::args`)
- [x] Datei öffnen (`File::open`) und lesen (`read_to_string`)
- [x] Zeilen zählen (`s.lines().count()`)
- [x] Zeichen zählen (`s.chars().count()`)
- [x] Bytes zählen (Rückgabe von `read_to_string`)
- [x] Fehlerbehandlung mit `match` (kein Panic bei fehlender Datei)

---

## 1. Grundfunktionen vervollständigen
- [ ] Wörter zählen: `s.split_whitespace().count()`
- [ ] Dateiname aus der Kommandozeile nehmen: `&args[1]` statt fest verdrahtet
- [ ] Prüfen, ob überhaupt ein Argument da ist (sonst sauberer Hinweis)
- [ ] Ausgabe in der richtigen Reihenfolge: **Zeilen, Wörter, Zeichen, Bytes**

## 2. Flags wie beim Original (`-l -w -c -m`)
- [ ] `-l` nur Zeilen
- [ ] `-w` nur Wörter
- [ ] `-c` nur Bytes
- [ ] `-m` nur Zeichen
- [ ] Kein Flag ⇒ Standard = `-l -w -c`
- [ ] Flags kombinierbar und in beliebiger Reihenfolge (`-lw`, `-w -l`)
- [ ] `-h` / `--help` mit kurzer Usage-Ausgabe
- [ ] `--version`

## 3. Mehrere Dateien + Summenzeile
- [ ] Über alle Pfad-Argumente iterieren
- [ ] Bei mehr als einer Datei eine `total`-Zeile anhängen
- [ ] Läuft eine Datei schief: Fehler melden, aber mit den anderen weitermachen

## 4. Standardeingabe (stdin)
- [ ] Kein Dateiname ⇒ von stdin lesen
- [ ] `-` als Dateiname ⇒ stdin
- [ ] Beispiel: `cat datei | rustwc -l`

## 5. Ausgabe wie das Original
- [ ] Zahlen rechtsbündig ausrichten (feste Spaltenbreite), wie `write_counts` im C-Code
      → in Rust z. B. `format!("{:>width$}", n, width = w)`
- [ ] Trennzeichen: ein Leerzeichen zwischen den Zahlen, dann der Dateiname
- [ ] Dateiname nur anhängen, wenn es einen gibt (bei stdin weglassen)

## 6. Robustheit — das macht es erst zum „echten Ersatz"
- [ ] **Binärdateien:** nicht mehr `read_to_string` benutzen!
      Das schlägt bei ungültigem UTF-8 fehl. Bytes müssen auch bei Binärdateien
      zählbar sein. ⇒ auf `Read::read` mit `Vec<u8>` umstellen.
- [ ] **Große Dateien:** nicht die ganze Datei in den RAM laden,
      sondern in Blöcken lesen (`BufReader` mit fester Puffergröße).
- [ ] `-m` korrekt für UTF-8 (mehrbyte-Zeichen = 1 Zeichen, aber mehrere Bytes)
- [ ] Exit-Code: `0` bei Erfolg, `!= 0` sobald eine Datei nicht lesbar war
- [ ] Verzeichnis als Argument ⇒ saubere Fehlermeldung (kein Panic)

## 7. Performance (optional, Bonus)
- [ ] `BufReader` mit z. B. 64 KiB Puffer
- [ ] Mit `time rustwc große_datei` gegen `time wc große_datei` vergleichen

## 8. Bonus / Streckziele
- [ ] `-L` längste Zeile (max line length)
- [ ] `--files0-from=DATEI` (NUL-getrennte Dateinamen)
- [ ] `cargo test`: Unit-Tests für die Zählfunktionen (Zeilen/Wörter/Bytes)
- [ ] Eine kleine Test-Datei mit Umlauten anlegen und `-m` vs `-c` prüfen

---

## Test-Kommandos (Parity-Check)
```sh
# Sollte jeweils identisch sein:
wc -l DATEI ; rustwc -l DATEI
wc -w DATEI ; rustwc -w DATEI
wc -c DATEI ; rustwc -c DATEI
wc -m DATEI ; rustwc -m DATEI

# Direkter Vergleich (leere Ausgabe = perfekt):
diff <(wc DATEI) <(rustwc DATEI)

# Sonderfälle:
rustwc /tmp/opencode/umlaut.txt      # chars != bytes
rustwc /bin/ls                       # Binärdatei, darf nicht crashen
rustwc fehlte_doch.txt ; echo $?     # Exit-Code != 0
cat DATEI | rustwc -l                # stdin
```

## Definition of Done
- [ ] `diff <(wc DATEI) <(rustwc DATEI)` ist **leer** für Text- und Binärdateien
- [ ] Funktioniert auf Binärdateien ohne Fehler
- [ ] Kein `panic!` / `unwrap()` in `main` bei kaputten Eingaben
- [ ] Exit-Codes stimmen mit `wc` überein
