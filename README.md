### Harry's Solution

#### Downloader
```
cargo run --bin downloader -- --start 1 --end 1000 --out ./files/raw --threads 50
```
join files
```
./join_files.sh 1 50 50

./join_all_files.sh

```

#### Task 1
Runs on all joined files

```
cargo run --bin task1
# prints hashmap of words 
# done. elapsed = 42.037937125s
```

#### Task 2
Runs on all joined files
```
cargo run --bin task2
# prints stats
# n_most_frequent_words = [("the", 4089183), ("of", 2463910), ("and", 2266544), ("to", 1858030), ("", 1575462), ("a", 1521706), ("in", 1152746), ("I", 760546), ("that", 747620), ("was", 741024)]
# n_unique_words = 2307682
# n_sentence_= 3058112
# done. elapsed = 48.462803583s
```
#### Task 3
Runs on seperate files
```
cargo run --bin task3
n_most_frequent_words = [("the", 4089015), ("of", 2463799), ("and", 2266480), ("to", 1857958), ("", 1575462), ("a", 1519845), ("in", 1151617), ("I", 760541), ("that", 747608), ("was", 741022)]
n_unique_words = 2307681
n_sentences = 3054745
done. elapsed = 53.779108041s
```
#### Task 4
```
cargo run --bin task4 -- --dir ./files/raw --files 1000 --threads 50
n_most_frequent_words = [("the", 4089015), ("of", 2463799), ("and", 2266480), ("to", 1857958), ("", 1575462), ("a", 1519845), ("in", 1151617), ("I", 760541), ("that", 747608), ("was", 741022)]
n_unique_words = 2307681
n_sentences = 3054745
done. elapsed = 54.204670625s
```
#### Task 5
```
cargo run --bin task5 -- --dir ./files/raw --files 1000 --threads 1000 --word harry
word = "harry" found = 60
paths = {
    "./files/raw/318.txt",
    "./files/raw/715.txt",
    ...
}
done. elapsed = 7.888408291s
```
