#!/bin/sh

OUT_DIR="results/hash/conflicts"
TMP_FILE="${OUT_DIR}/tmp2.out"
RESULT_FILE="${OUT_DIR}/result2.out"

AMOUNT_OF_ITERATIONS=100
AMOUNT_OF_KMERS=10000000
AMOUNT_OF_BUCKETS=$(seq 10000 100000 10000000)
DEPTH=1200

echo -e "#iterations\t#kmers\t#buckets\tmin\tmax\tmean\tsd" > $RESULT_FILE

for i in $AMOUNT_OF_BUCKETS
do
    FILENAME=${OUT_DIR}/${AMOUNT_OF_KMERS}_${i}_${AMOUNT_OF_ITERATIONS}.out
    
    echo "creating $FILENAME"

    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i))         >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 10000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 20000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 30000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 40000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 50000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 60000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 70000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 80000)) >> $TMP_FILE &
    hash_index bench_max_items_per_bucket -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $((i + 90000)) >> $TMP_FILE &
    
    wait
done

sort -k3 -n $TMP_FILE >> $RESULT_FILE

rm $TMP_FILE
