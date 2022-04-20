#!/bin/sh

OUT_DIR="results/hash/conflicts"
RESULT_FILE="${OUT_DIR}/result.out"

AMOUNT_OF_ITERATIONS=1000
AMOUNT_OF_KMERS=100000
#AMOUNT_OF_BUCKETS=$(seq 100 100 100)
AMOUNT_OF_BUCKETS=$(seq 100 100 100000)
DEPTH=1200

echo -e "amount of iterations\tamount of kmers\tamount of buckets\tmax depth" > $RESULT_FILE

for i in $AMOUNT_OF_BUCKETS
do
    FILENAME=${OUT_DIR}/${AMOUNT_OF_KMERS}_${i}_${AMOUNT_OF_ITERATIONS}.out
    
    echo "creating $FILENAME"

    hash_index bench_hash_conflicts -i $AMOUNT_OF_ITERATIONS -n $AMOUNT_OF_KMERS -b $i -d $DEPTH > $FILENAME
        
    DEPTH=$(
        cat $FILENAME                               |
            grep ".* 0/${AMOUNT_OF_KMERS}"          | 
            head -1                                 | 
            sed -e 's/conflicts (i: \(.*\)).*/\1/'
    )

    echo -e "${AMOUNT_OF_ITERATIONS}\t${AMOUNT_OF_KMERS}\t${i}\t${DEPTH}" >> $RESULT_FILE

    DEPTH=$((DEPTH + 10))

    rm $FILENAME
done