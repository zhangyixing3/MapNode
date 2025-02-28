### MapNode: matching the pangenome nodes tool

#### Overview
MapNode matches node IDs between modified GFA files in two scenarios:
1. **Sorted GFA for Visualization**  
   When using `odgi` to draw 2D figures from minigraph-cactus GFA files, sorting changes node IDs. MapNode maps original ↔ sorted node IDs.
2. **Chopped GFA Comparison**  
   Maps nodes between an original GFA and one processed with `vg mod -X`.

---

#### Installation
```bash
git clone git@github.com:zhangyixing3/MapNode.git
cd MapNode && cargo build -j 2 --release
```
#### Run Mapping Command
```
$ convert
Usage: convert <raw.gfa> <sub.gfa> > output.txt
$ head output.txt
1770	4305
1771	4307
1772	4310
1773	4311
1774	4312
1775	4316
1776	4317
1777	4319
1778	4320
1779	4323
```