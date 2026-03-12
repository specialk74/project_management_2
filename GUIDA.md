# Project Management Effort Tracker App — Guida

## Avvio da riga di comando

```
project_app [OPZIONI] [FILE]
```

### Argomenti

| Argomento | Forma breve | Descrizione | Default |
|-----------|-------------|-------------|---------|
| `--file <percorso>` | `-f <percorso>` | Percorso del file JSON da caricare | `efforts.json` |
| `--start-date <data>` | `-d <data>` | Data di inizio visualizzazione (formato `YYYY-MM-DD`) | Calcolata dai progetti |
| `[FILE]` | — | Argomento posizionale: percorso del file JSON | `efforts.json` |

### Esempi

```bash
# Apre il file di default (efforts.json nella directory corrente)
project_app

# File specifico come argomento posizionale
project_app mio_progetto.json

# File specifico con flag
project_app --file /percorso/mio_progetto.json
project_app -f /percorso/mio_progetto.json

# Data di inizio personalizzata (mostra settimane da quella data)
project_app --start-date 2025-01-06
project_app -d 2025-01-06

# Combinazione file + data di inizio
project_app -f mio_progetto.json -d 2025-01-06
```

> **Nota:** `--start-date` sovrascrive la data di inizio calcolata automaticamente
> a partire dai dati dei progetti nel file JSON.

---

## Scorciatoie da tastiera

### Finestra principale

| Scorciatoia | Azione |
|-------------|--------|
| `Ctrl+S` | Salva il file corrente |
| `Ctrl+O` | Apre il dialogo di selezione file (apri un file JSON) |
| `Ctrl+N` | Crea un nuovo progetto |
| `Ctrl+f` | Apre la finestra di ricerca worker |
| `Ctrl+Shift+F` | Cancella il filtro di ricerca (mostra tutto) |

### Celle modificabili (Cell-RW)

| Scorciatoia | Azione |
|-------------|--------|
| `Invio` | Entra in modalità modifica della cella |
| `Invio` *(in modifica)* | Conferma il valore e esce dalla modifica |
| `Esc` | Esce dalla modifica senza applicare cambiamenti |
| `Canc` / `Backspace` | Svuota il contenuto della cella |
| `Tab` | Sposta il focus alla cella successiva |
| `Ctrl+C` | Copia il contenuto della cella negli appunti interni |
| `Ctrl+V` | Incolla il contenuto dagli appunti interni nella cella |
| `Ctrl+S` | Salva il file (funziona anche con una cella selezionata) |
| `Ctrl+f` | Apre la ricerca worker (funziona anche con una cella selezionata) |
| `Ctrl+Shift+F` | Cancella il filtro (funziona anche con una cella selezionata) |

### Finestra di ricerca worker

| Scorciatoia / Azione | Comportamento |
|----------------------|---------------|
| `Invio` | Applica il filtro e chiude la finestra |
| `Esc` | Chiude la finestra ripristinando il filtro precedente |
| Pulsante **Search** | Applica il filtro e chiude la finestra |
| Pulsante **Abort** | Chiude la finestra e rimuove il filtro |

#### Sintassi del filtro di ricerca

- Inserire il nome (o parte del nome) del worker da cercare.
- Usare `|` per cercare più worker contemporaneamente.

```
Mario               → mostra solo i progetti con "Mario"
Mario|Lucia         → mostra i progetti con "Mario" oppure "Lucia"
```

---

## Menu contestuale (tasto destro)

Il menu contestuale si apre facendo **clic destro** sulla barra colorata di separazione
di una riga sviluppatore nella colonna di sinistra.

| Voce | Azione |
|------|--------|
| **Add Row** | Aggiunge una riga persona alla sezione sviluppatore |
| **Del Row** | Rimuove l'ultima riga persona dalla sezione sviluppatore |
| **Hide Dev** | Nasconde la sezione sviluppatore |
| **Show Dev** | Mostra la sezione sviluppatore (se nascosta) |

> **Doppio clic** sull'etichetta colorata dello sviluppatore (es. *Mcsw*, *Hw*, ecc.)
> aggiunge direttamente una riga persona, equivalente a **Add Row**.

---

## Barra del titolo

```
Project Management Effort Tracker App [nomefile.json] (*)
```

| Parte | Significato |
|-------|-------------|
| `[nomefile.json]` | Nome del file JSON attualmente aperto |
| `(*)` | Indica modifiche non ancora salvate |

---

## Colori effort

Le celle della griglia settimanale mostrano i giorni/persona assegnati,
colorati in base alla percentuale di effort consumata rispetto al totale:

| Colore | Percentuale consumata |
|--------|-----------------------|
| Bianco / Nero | Nessun effort assegnato |
| Verde scuro | < 10% |
| Verde | 10% – 29% |
| Giallo-verde | 30% – 49% |
| Arancione | 50% – 79% |
| Rosso | ≥ 80% |

> **Hover** su una cella mostra in giallo i giorni/persona **rimanenti** (remains).

---

## Formato file JSON

Il file di dati è un JSON con la seguente struttura:

```json
{
  "sovra": [...],
  "week_off": [],
  "worker_names": ["Mario Rossi", "Lucia Bianchi"],
  "projects": [...]
}
```

Il file viene salvato nella stessa posizione da cui è stato aperto.
