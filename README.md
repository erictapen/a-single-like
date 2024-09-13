# A single like

Ein Poster, dass den Datenfluss eines einzelnen Likes darstellt.

![](preview.jpeg)

In hoher Auflösung kann das Plakat in der Datei `plakat_final.pdf` betrachtet werden.


## Building

Benötigt werden `cargo`, `rustc`, `wireshark` und `inkscape`. [Nix](https://nixos.org/nix)-User können mit `nix-shell` bequem alle Dependencies temporär installieren.

Zum generieren des Plakates dann

```
cargo build
./target/debug/plakat
```

ausführen. Das Ergebnis liegt dann in template.svg (in diesem Fall natürlich ohne die meisten Diagramme, weil die Rohdaten nicht im Repo liegen).
