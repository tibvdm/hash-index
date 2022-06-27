#!/usr/bin/env python3

from tqdm import tqdm

import csv
import gzip
import json

TAXON_FILE = '../data/taxons.tsv'

EC_FILE = '../data/ec_cross_references.tsv.gz'
EC_INFO_FILE = '../data/ec_numbers.tsv.gz'

GO_FILE = '../data/go_cross_references.tsv.gz'
GO_INFO_FILE = '../data/go_terms.tsv.gz'

INTERPRO_FILE = '../data/interpro_cross_references.tsv.gz'
INTERPRO_INFO_FILE = '../data/interpro_entries.tsv.gz'

def construct_taxon_mapping(filename):
    print('constructing the taxon mapping!')

    taxon_mapping = dict()

    with open(filename) as fh:
        for row in tqdm(fh.readlines()):
            tid, name, rank = row.rstrip().split('\t')[:3]

            taxon_mapping[int(tid)] = {
                'name': name,
                'rank': rank
            }

    print()

    return taxon_mapping

def construct_ec_mapping(filename):
    print('constructing the EC mapping!')

    ec_mapping = dict()

    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, uniprot_id, ec_number = row.rstrip().split('\t')[:3]
            
            ec_mapping[uniprot_id] = ec_number

    print()

    return ec_mapping

def construct_go_mapping(filename):
    print('constructing the GO mapping!')

    go_mapping = dict()

    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, uniprot_id, go_term = row.rstrip().split('\t')[:3]
            
            go_mapping[uniprot_id] = go_term

    print()

    return go_mapping

def construct_interpro_mapping(filename):
    print('constructing the InterPro mapping!')

    interpro_mapping = dict()

    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, uniprot_id, interpro_number = row.rstrip().split('\t')[:3]
            
            interpro_mapping[uniprot_id] = interpro_number

    print()

    return interpro_mapping

def construct_ec_info_mapping(filename):
    print('constructing the EC INFO mapping!')

    ec_info_mapping = dict()

    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, ec_number, name = row.rstrip().split('\t')[:3]
            
            ec_info_mapping[ec_number] = {
                'ec_number': ec_number,
                'name': name
            }

    print()

    return ec_info_mapping

def construct_go_info_mapping(filename):
    print('constructing the GO INFO mapping!')

    go_info_mapping = dict()
    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, go_term, namespace, name = row.rstrip().split('\t')[:4]
            
            go_info_mapping[go_term] = {
                'go_number': go_term,
                'namespace': namespace,
                'name': name
            }

    print()

    return go_info_mapping

def construct_interpro_info_mapping(filename):
    print('constructing the GO INFO mapping!')

    interpro_info_mapping = dict()

    with gzip.open(filename, 'rt') as fh:
        for row in tqdm(fh.readlines()):
            _, interpro_number, namespace, name = row.rstrip().split('\t')[:4]
            
            interpro_info_mapping[interpro_number] = {
                'interpro_number': interpro_number,
                'namespace': namespace,
                'name': name
            }

    print()

    return interpro_info_mapping

def process_agg_file(filename, output_file):
    print('Processing the aggregation file!')

    json_dict = dict()

    ec_mapping = construct_ec_mapping(EC_FILE)
    go_mapping = construct_go_mapping(GO_FILE)
    interpro_mapping = construct_interpro_mapping(INTERPRO_FILE)

    ec_info_mapping = construct_ec_info_mapping(EC_INFO_FILE)
    go_info_mapping = construct_go_info_mapping(GO_INFO_FILE)
    interpro_info_mapping = construct_interpro_info_mapping(INTERPRO_INFO_FILE)

    with open(filename) as fh:
        for line in tqdm(fh.readlines()):
            # Skip headers
            if line.startswith('>'): 
                continue

            splitted = line.rstrip().split('\t')

            lca = int(splitted[0])

            if lca not in json_dict:
                json_dict[lca] = {
                    'lca': lca,
                    'ec': [],
                    'go': [],
                    'interpro': []
                }

            if len(splitted) > 1:
                functions = splitted[1].split(';')

                for uniprot_id in functions:
                    if uniprot_id in ec_mapping:
                        ec_number = ec_mapping[uniprot_id]
                        if ec_number not in ec_info_mapping:
                            json_dict[lca]['ec'].append({'ec_number': ec_number, 'name': None})
                        else:
                            json_dict[lca]['ec'].append(ec_info_mapping[ec_number])

                    if uniprot_id in go_mapping:
                        go_number = go_mapping[uniprot_id]
                        if go_number not in go_info_mapping:
                            json_dict[lca]['go'].append({'go_number': go_number, 'namespace': None, 'name': None})
                        else:
                            json_dict[lca]['go'].append(go_info_mapping[go_number])

                    if uniprot_id in interpro_mapping:
                        interpro_number = interpro_mapping[uniprot_id]
                        if interpro_number not in interpro_info_mapping:
                            json_dict[lca]['interpro'].append({'interpro_number': interpro_number, 'namespace': None, 'name': None})
                        else:
                            json_dict[lca]['interpro'].append(interpro_info_mapping[interpro_number])
            
    print()

    with open(output_file, 'w') as fh:
        print('Encoding json!')
        encoded = list(json.JSONEncoder(sort_keys=True, indent=4).iterencode(json_dict))

        print()
        
        print(f'Dumping contents to {output_file}!')
        for chunk in tqdm(encoded):
            fh.write(chunk)

# taxon_mapping = construct_taxon_mapping(TAXON_FILE)
# 
# ec_mapping = construct_ec_mapping(EC_FILE)
# go_mapping = construct_go_mapping(GO_FILE)
# interpro_mapping = construct_interpro_mapping(INTERPRO_FILE)
# 
# ec_info_mapping = construct_ec_info_mapping(EC_INFO_FILE)
# go_info_mapping = construct_go_info_mapping(GO_INFO_FILE)
# interpro_info_mapping = construct_interpro_info_mapping(INTERPRO_INFO_FILE)
# 
# print(ec_info_mapping[ec_mapping[list(ec_mapping.keys())[0]]])
# print(go_info_mapping[go_mapping[list(go_mapping.keys())[0]]])
# print(interpro_info_mapping[interpro_mapping[list(interpro_mapping.keys())[0]]])

process_agg_file('../data/taxa2agg.minimized.out', 'test.out')
