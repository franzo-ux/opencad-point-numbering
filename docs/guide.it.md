# Guida rapida — Numerazione punti

> **In prova:** prima di operare su disegni importanti, salva una copia del DWG/DXF e verifica le etichette create.

## Cosa fa

Il plugin inserisce etichette di testo numeriche nel disegno. Dopo l’avvio, fai clic su un vertice o in un punto vuoto: viene inserita un’etichetta leggermente in alto a destra del clic. Il valore successivo viene mantenuto finché OpenCADStudio resta aperto.

## Prima installazione su Windows

1. Apri la [release più recente](https://github.com/franzo-ux/opencad-point-numbering/releases/latest).
2. Scarica **entrambi** i file:
   - `opencad.point_numbering-windows-x86_64.dll`
   - `plugin.toml`
3. In Esplora file, incolla questo percorso nella barra degli indirizzi:

   ```text
   %APPDATA%\OpenCADStudio\plugins\opencad.point_numbering\
   ```

4. Se la cartella non esiste, creala.
5. Copia entrambi i file al suo interno.
6. Riavvia OpenCADStudio.

Dopo il riavvio compare la scheda **xfTools** nel ribbon, con il pulsante **Number points** e l’icona `1,2…`.

## Installazione su macOS (Apple Silicon)

1. Apri la [release più recente](https://github.com/franzo-ux/opencad-point-numbering/releases/latest).
2. Scarica `opencad.point_numbering-macos-aarch64.dylib` e `plugin.toml`.
3. Copia entrambi in:

   ```text
   ~/Library/Application Support/OpenCADStudio/plugins/opencad.point_numbering/
   ```

4. Riavvia OpenCADStudio.

## Numerazione standard

1. Apri un disegno.
2. Seleziona **xfTools → Number points** oppure digita `PNUM` nella riga di comando.
3. Fai clic sui punti nell’ordine desiderato.
4. Premi **Invio** o **Esc** per terminare.

La prima esecuzione parte da `1`, aumenta di `1` e non ha prefisso.

## Configurare o riavviare la numerazione

Prima di selezionare i punti, digita:

```text
PNUM:numero-iniziale,incremento,prefisso
```

| Obiettivo | Comando | Risultato |
| --- | --- | --- |
| Continuare | `PNUM` | Mantiene il prossimo valore della sessione |
| Ricominciare da 1 | `PNUM:1` | Riparte da 1 e conserva incremento/prefisso correnti |
| Prefisso `P-` | `PNUM:1,1,P-` | `P-1`, `P-2`, `P-3`… |
| Salti di 10 | `PNUM:100,10,PT-` | `PT-100`, `PT-110`, `PT-120`… |
| Conto alla rovescia | `PNUM:10,-1,N-` | `N-10`, `N-9`, `N-8`… |
| Rimuovere il prefisso | `PNUM:1,1,` | `1`, `2`, `3`… |

Non inserire spazi dopo `PNUM`: il comando parte immediatamente. Il prefisso non può contenere spazi.

## Aspetto delle etichette

- Le etichette sono entità `TEXT`, quindi puoi selezionarle, spostarle, modificarle o eliminarle come normale testo CAD.
- Altezza testo: `2.5` unità di disegno.
- Offset: `1.25` unità verso alto-destra dal punto cliccato.
- Stile testo: quello standard del disegno.

## Risoluzione problemi

| Problema | Cosa verificare |
| --- | --- |
| La scheda **xfTools** non compare | Controlla che DLL e `plugin.toml` siano nella stessa cartella e riavvia OpenCADStudio. |
| Il plugin non viene caricato | Verifica di usare OpenCADStudio `v2026.37`; il plugin dipende dalla sua API. |
| Il testo è troppo piccolo/grande | La prima versione usa altezza fissa `2.5`; modifica la singola etichetta o richiedi una versione configurabile. |
| Il numero non riparte | Avvia `PNUM` con il numero iniziale desiderato, ad esempio `PNUM:1,1,P-`. |
