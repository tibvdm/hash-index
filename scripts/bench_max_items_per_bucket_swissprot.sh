#!/bin/sh

OUT_DIR="results/hash/conflicts"
TMP_FILE="${OUT_DIR}/tmp2.out"
RESULT_FILE="${OUT_DIR}/result2.out"

AMOUNT_OF_BUCKETS=$(seq 100000 1000000 109791940)

echo -e "#kmers\t#buckets\tmin\tmax\tmean" > $RESULT_FILE

for i in $AMOUNT_OF_BUCKETS
do
    FILENAME=${OUT_DIR}/109791940_${i}.out
    
    echo "creating $FILENAME"

    hash_index bench_max_items_per_bucket_swissprot -b $((i))          < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 100000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 200000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 300000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 400000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 500000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 600000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 700000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 800000)) < $1 >> $TMP_FILE &
    hash_index bench_max_items_per_bucket_swissprot -b $((i + 900000)) < $1 >> $TMP_FILE &
    
    wait
done

sort -k2 -n $TMP_FILE >> $RESULT_FILE

rm $TMP_FILE
