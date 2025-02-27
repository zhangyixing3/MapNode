#### You can use MapNode to match the two graph nodes based on their coordinates.
1. if you have minigraph-cactus gfa, and you want to draw 2D-figure by odgi, you have to sort the graph, unfortunatelly, the gfa file will change node id after sorting, so this tool will help you to match the two graph nodes based on their coordinates.
2. if you have two gfa files, the first one is the original gfa file, and the second one is chopped by 'vg mod -X', you can also use this tool to match the two graph nodes based on their coordinates.

```
git clone git@github.com:zhangyixing3/MapNode.git
cd MapNode && cargo build -j 2 --release
```
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