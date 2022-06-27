#!/bin/sh

tmp="$(mktemp -d)"

# ======================================================================
# Argument parsing
# ======================================================================

tryptic_used="false"
ninemer_used="false"
type="high-precision"

while getopts I:T:z1:2:t:o: option; do
    case "$option" in
        I) index="$OPTARG"          ;;
        T) taxons="$OPTARG"         ;;
        1) inputfile1="$OPTARG"     ;;
        2) inputfile2="$OPTARG"     ;;
        t) type="$OPTARG"           ;;
        o) outputfile="$OPTARG"     ;;
        \?) echo "TODO"
            exit                    ;;
    esac
done

if [ -z "$index" ]; then
    echo "Missing index."
    exit
fi

if [ -z "$taxons" ]; then
    echo "Missing taxons index."
    exit
fi

if [ -z "$inputfile1" -o -z "$inputfile2" ]; then
    echo "Two files needed for now."
    exit
fi

case "$type" in
    "max-sensitivity") ninemer_used="true"      ;;
    "high-sensitivity") ninemer_used="true"     ;;
    "tryptic-sensitivity") tryptic_used="true"  ;;
    "tryptic-precision") tryptic_used="true"    ;;
    "high-precision") ninemer_used="true"       ;;
    "max-precision") ninemer_used="true"        ;;
    *) echo "Invalid type."; 
       exit                                     ;;
esac

if [ -z "$outputfile" ]; then
    echo "No output file specified."
    exit
fi

# ======================================================================
# Pipelines
# ======================================================================

if [ "$ninemer_used" = "true" ]; then
    umgap prot2kmer2lca -m -o -s "$tmp/socket" "$index" &
    index_pid="$!"
    trap "rm -rf '$tmp' && kill '$index_pid'" KILL EXIT
    while [ ! -S "$tmp/socket" ] && sleep 1; do true; done
fi

mkfifo "$tmp/1" "$tmp/2" "$tmp/out"
zcat $inputfile1 > "$tmp/1" &
zcat $inputfile2 > "$tmp/2" &

case "$type" in
    "max-sensitivity")
        mkdir -p max-sensitivity

        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > max-sensitivity/fastq2fasta.out         
        echo "Executing translate"     
        umgap translate -n -a -t1                       < max-sensitivity/fastq2fasta.out   > max-sensitivity/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2kmer2lca -m -o "$index"              < max-sensitivity/translate.out     > max-sensitivity/prot2kmer2lca.out
        echo "Executing bestof"
        umgap bestof                                    < max-sensitivity/prot2kmer2lca.out > max-sensitivity/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < max-sensitivity/bestof.out        > max-sensitivity/uniq.out
        echo "Executing taxa2agg"
        umgap taxa2agg -l1 -m rmq -a mrtl "$taxons"     < max-sensitivity/uniq.out          > max-sensitivity/taxa2agg.out                  ;;
    "high-sensitivity")
        mkdir -p high-sensitivity
        
        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > high-sensitivity/fastq2fasta.out 
        echo "Executing translate"
        umgap translate -n -a -t1                       < high-sensitivity/fastq2fasta.out   > high-sensitivity/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2kmer2lca -m -o "$index"              < high-sensitivity/translate.out     > high-sensitivity/prot2kmer2lca.out
        echo "Executing bestof"
        umgap bestof                                    < high-sensitivity/prot2kmer2lca.out > high-sensitivity/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < high-sensitivity/bestof.out        > high-sensitivity/uniq.out
        echo "Executing taxa2agg"
        umgap taxa2agg -l1 -a hybrid -f 0.25 "$taxons"  < high-sensitivity/uniq.out          > high-sensitivity/taxa2agg.out                ;;
    "tryptic-sensitivity") 
        mkdir -p tryptic-sensitivity

        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > tryptic-sensitivity/fastq2fasta.out 
        echo "Executing translate"   
        umgap translate -n -a -t11                      < tryptic-sensitivity/fastq2fasta.out   > tryptic-sensitivity/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2tryp2lca -m -o -l9 -L45 "$index"           < tryptic-sensitivity/translate.out     > tryptic-sensitivity/prot2tryp2lca.out
        echo "Executing bestof"
        umgap bestof                                    < tryptic-sensitivity/prot2tryp2lca.out > tryptic-sensitivity/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < tryptic-sensitivity/bestof.out        > tryptic-sensitivity/uniq.out      
        echo "Executing taxa2agg"                   
        umgap taxa2agg -l1 -m rmq -a mrtl "$taxons"     < tryptic-sensitivity/uniq.out          > tryptic-sensitivity/taxa2agg.out          ;;
    "tryptic-precision")
        mkdir -p tryptic-precision

        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > tryptic-precision/fastq2fasta.out 
        echo "Executing translate"   
        umgap translate -n -a -t11                      < tryptic-precision/fastq2fasta.out   > tryptic-precision/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2tryp2lca -m -o -l9 -L45 "$index"           < tryptic-precision/translate.out     > tryptic-precision/prot2tryp2lca.out
        echo "Executing bestof"
        umgap bestof                                    < tryptic-precision/prot2tryp2lca.out > tryptic-precision/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < tryptic-precision/bestof.out        > tryptic-precision/uniq.out     
        echo "Executing taxa2agg"
        umgap taxa2agg -l5 -m rmq -a mrtl "$taxons"     < tryptic-precision/uniq.out          > tryptic-precision/taxa2agg.out              ;;
    "high-precision")
        mkdir -p high-precision

        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > high-precision/fastq2fasta.out  
        echo "Executing translate"   
        umgap translate -a                              < high-precision/fastq2fasta.out   > high-precision/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2kmer2lca -m -o "$index"              < high-precision/translate.out     > high-precision/prot2kmer2lca.out
        echo "Executing bestof"
        umgap bestof                                    < high-precision/prot2kmer2lca.out > high-precision/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < high-precision/bestof.out        > high-precision/uniq.out
        echo "Executing taxa2agg"
        umgap taxa2agg -l2 -a lca\* "$taxons"           < high-precision/uniq.out          > high-precision/taxa2agg.out                    ;;
    "max-precision")
        mkdir -p max-precision

        echo "Executing fastq2fasta"
        umgap fastq2fasta "$tmp/1" "$tmp/2"             > max-precision/fastq2fasta.out  
        echo "Executing translate"   
        umgap translate -a                              < max-precision/fastq2fasta.out   > max-precision/translate.out
        echo "Executing prot2kmer2lca"
        umgap prot2kmer2lca -m -o "$index"              < max-precision/translate.out     > max-precision/prot2kmer2lca.out   
        echo "Executing bestof"                       
        umgap bestof                                    < max-precision/prot2kmer2lca.out > max-precision/bestof.out
        echo "Executing uniq"
        umgap uniq -d ' '                               < max-precision/bestof.out        > max-precision/uniq.out
        echo "Executing taxa2agg"
        umgap taxa2agg -l5 -a lca\* "$taxons"           < max-precision/uniq.out          > max-precision/taxa2agg.out                      ;;
esac
