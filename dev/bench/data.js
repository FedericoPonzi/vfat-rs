window.BENCHMARK_DATA = {
  "lastUpdate": 1771697430185,
  "repoUrl": "https://github.com/FedericoPonzi/vfat-rs",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "ad27edb3ffb1e9f35ffff24c3196d62b3b526d6d",
          "message": "Add benchmarks, fixes #12",
          "timestamp": "2026-02-20T08:38:51Z",
          "tree_id": "c3e786729e7b7a5f27e262b7a3bf53c455216538",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/ad27edb3ffb1e9f35ffff24c3196d62b3b526d6d"
        },
        "date": 1771578423928,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 40514,
            "range": "± 28562",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 18953,
            "range": "± 6667",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 48646,
            "range": "± 15800",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 19916,
            "range": "± 7846",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 22955,
            "range": "± 1298",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 13697,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30093,
            "range": "± 1572",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "15702715eb631d3ef719fa0007b6932ea4f9b71a",
          "message": "Add benchmarks, fixes #12",
          "timestamp": "2026-02-20T08:38:51Z",
          "tree_id": "e14b3d6de7304f60153253dff5f788b447866232",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/15702715eb631d3ef719fa0007b6932ea4f9b71a"
        },
        "date": 1771582058131,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 156139,
            "range": "± 79992",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 118305,
            "range": "± 76460",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 158609,
            "range": "± 79165",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 117516,
            "range": "± 74503",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 19224,
            "range": "± 681",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14490,
            "range": "± 224",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 20926,
            "range": "± 1170",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12885,
            "range": "± 559",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 5242,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3651,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 18119,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8218,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 483080,
            "range": "± 19121",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 380532,
            "range": "± 11619",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 2884278,
            "range": "± 1247315",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 1425072,
            "range": "± 1077581",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 320043,
            "range": "± 158592",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 253906,
            "range": "± 159552",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 385964,
            "range": "± 155210",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 245803,
            "range": "± 150735",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 12419927,
            "range": "± 5485669",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 5270665,
            "range": "± 4120036",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 8414,
            "range": "± 429",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4940,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 16004,
            "range": "± 1613",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5339,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 555647,
            "range": "± 11405",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 387085,
            "range": "± 23204",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 135524,
            "range": "± 1679",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 12975,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 4967,
            "range": "± 273",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3849,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 17435,
            "range": "± 921",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9780,
            "range": "± 631",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "cca13ac2e5a24824cff63a2acad8835239e592dc",
          "message": "Add support for FSI_Nxt_Free\n\nThis helps speeding up allocations. From fat docuementation: https://www.cs.fsu.edu/~cop4610t/assignments/project3/spec/fatspec.pdf\n\nThis is a hint for the FAT driver. It indicates the cluster number at\nwhich the driver should start looking for free clusters. Because a\nFAT32 FAT is large, it can be rather time consuming if there are a\nlot of allocated clusters at the start of the FAT and the driver starts\nlooking for a free cluster starting at cluster 2. Typically this value is\nset to the last cluster number that the driver allocated. If the value is\n0xFFFFFFFF, then there is no hint and the driver should start\nlooking at cluster 2. Any other value can be used, but should be\nchecked first to make sure it is a valid cluster number for the\nvolume.",
          "timestamp": "2026-02-20T08:55:00Z",
          "tree_id": "06eae10afa3c262f1d4752753c1f9c91d74cc37d",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/cca13ac2e5a24824cff63a2acad8835239e592dc"
        },
        "date": 1771586205938,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 161066,
            "range": "± 78190",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 94793,
            "range": "± 63315",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 169825,
            "range": "± 78772",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 94363,
            "range": "± 63291",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 25758,
            "range": "± 699",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14412,
            "range": "± 298",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 35731,
            "range": "± 1387",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12404,
            "range": "± 697",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 7676,
            "range": "± 574",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3533,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 31809,
            "range": "± 1204",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 7738,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 1127296,
            "range": "± 41921",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 755908,
            "range": "± 49330",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1678552,
            "range": "± 80923",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 637393,
            "range": "± 180501",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 338685,
            "range": "± 158800",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 211544,
            "range": "± 126804",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 431686,
            "range": "± 156928",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 207578,
            "range": "± 119577",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 6205613,
            "range": "± 68765",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "4df77314fc739819637c75eff65dbf071536cf7f",
          "message": "Parametetrized size of test fs",
          "timestamp": "2026-02-20T08:57:00Z",
          "tree_id": "811697af15b3b681ef1041b00affd674c00fad0f",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/4df77314fc739819637c75eff65dbf071536cf7f"
        },
        "date": 1771591845560,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 161438,
            "range": "± 79865",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 100270,
            "range": "± 66190",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 162307,
            "range": "± 78125",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 109090,
            "range": "± 72302",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23472,
            "range": "± 1255",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14199,
            "range": "± 955",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30388,
            "range": "± 483",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12391,
            "range": "± 1408",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6648,
            "range": "± 361",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3328,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27605,
            "range": "± 1023",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8197,
            "range": "± 307",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 897874,
            "range": "± 38499",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 639496,
            "range": "± 28035",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1435790,
            "range": "± 93587",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 485829,
            "range": "± 65805",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 326875,
            "range": "± 154951",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 216229,
            "range": "± 140910",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 390237,
            "range": "± 152362",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 209271,
            "range": "± 127540",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5070704,
            "range": "± 63854",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1661172,
            "range": "± 48586",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11491,
            "range": "± 684",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4862,
            "range": "± 165",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25325,
            "range": "± 1364",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5166,
            "range": "± 460",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1050009,
            "range": "± 21369",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 648739,
            "range": "± 34647",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 253861,
            "range": "± 6008",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 11123,
            "range": "± 413",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6424,
            "range": "± 428",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3646,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23999,
            "range": "± 623",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9614,
            "range": "± 907",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "13ec033a778aee41a719c40d06c0683f650f12d1",
          "message": "docs: add docs to public api, enable doc required directive",
          "timestamp": "2026-02-21T15:51:30Z",
          "tree_id": "ad654cb5dcd5ee663aa11a6b73de8a9d446ded01",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/13ec033a778aee41a719c40d06c0683f650f12d1"
        },
        "date": 1771689603079,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 163827,
            "range": "± 80172",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 93679,
            "range": "± 62865",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 169583,
            "range": "± 79263",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 100277,
            "range": "± 66348",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 25362,
            "range": "± 1155",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14107,
            "range": "± 951",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 35252,
            "range": "± 1516",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12424,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 7338,
            "range": "± 393",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3577,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 31847,
            "range": "± 775",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 7992,
            "range": "± 727",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 1103329,
            "range": "± 27716",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 721997,
            "range": "± 19169",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1714389,
            "range": "± 75507",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 639156,
            "range": "± 181242",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 340378,
            "range": "± 159194",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 218113,
            "range": "± 133273",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 403111,
            "range": "± 148299",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 203768,
            "range": "± 118296",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 6144702,
            "range": "± 57551",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1940776,
            "range": "± 69686",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 13045,
            "range": "± 478",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4948,
            "range": "± 379",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 29913,
            "range": "± 1615",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5110,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1253684,
            "range": "± 20923",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 749246,
            "range": "± 24410",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 313870,
            "range": "± 5946",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 10907,
            "range": "± 513",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 7594,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3802,
            "range": "± 330",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 27136,
            "range": "± 819",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9653,
            "range": "± 381",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "2cefd8bafef4081292901b7684a9ff306ad604f9",
          "message": "Add cargo-clippy as part of ci step",
          "timestamp": "2026-02-21T17:38:50Z",
          "tree_id": "2e94807a90a2be7bac9f016577703b0ac13998a0",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/2cefd8bafef4081292901b7684a9ff306ad604f9"
        },
        "date": 1771696041783,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 161310,
            "range": "± 80146",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 105379,
            "range": "± 71102",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 164567,
            "range": "± 78728",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 98625,
            "range": "± 70537",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23436,
            "range": "± 1614",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 13845,
            "range": "± 1547",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30218,
            "range": "± 1790",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12126,
            "range": "± 581",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6824,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3287,
            "range": "± 114",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27342,
            "range": "± 1596",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 7942,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 908798,
            "range": "± 36467",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 644116,
            "range": "± 16713",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1420600,
            "range": "± 83959",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 494506,
            "range": "± 63889",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 331659,
            "range": "± 155548",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 218590,
            "range": "± 136383",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 381849,
            "range": "± 151248",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 215716,
            "range": "± 127332",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5102621,
            "range": "± 62428",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1689480,
            "range": "± 65441",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11431,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4760,
            "range": "± 104",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25652,
            "range": "± 403",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5204,
            "range": "± 519",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1045799,
            "range": "± 18785",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 660965,
            "range": "± 19041",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 257720,
            "range": "± 9474",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 13024,
            "range": "± 816",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6453,
            "range": "± 449",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3649,
            "range": "± 97",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 24068,
            "range": "± 1230",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9803,
            "range": "± 742",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "8aa246b45eb06b60736fa4e030f48b5b551c8e86",
          "message": "Add cargo-clippy as part of ci step",
          "timestamp": "2026-02-21T17:50:49Z",
          "tree_id": "802d6c2ed7084b2d10dfa8d543d9bc9f2eea0284",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/8aa246b45eb06b60736fa4e030f48b5b551c8e86"
        },
        "date": 1771696761268,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 160752,
            "range": "± 80224",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 100849,
            "range": "± 69139",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 167702,
            "range": "± 80432",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 98615,
            "range": "± 72657",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23343,
            "range": "± 902",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 13719,
            "range": "± 614",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30158,
            "range": "± 1005",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12493,
            "range": "± 976",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6698,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3270,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27468,
            "range": "± 1458",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8267,
            "range": "± 292",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 907917,
            "range": "± 29607",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 651110,
            "range": "± 13212",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1423902,
            "range": "± 76365",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 482227,
            "range": "± 60497",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 327118,
            "range": "± 153604",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 219839,
            "range": "± 133241",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 389322,
            "range": "± 158052",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 205511,
            "range": "± 118168",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5048067,
            "range": "± 98128",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1680756,
            "range": "± 54592",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11632,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4737,
            "range": "± 244",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25514,
            "range": "± 1794",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5156,
            "range": "± 535",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1041887,
            "range": "± 26048",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 661096,
            "range": "± 39515",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 256309,
            "range": "± 10434",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 12932,
            "range": "± 1782",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6381,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3657,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23625,
            "range": "± 1181",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9511,
            "range": "± 364",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "me@fponzi.me",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "distinct": true,
          "id": "a7d8a748704384e5e167c868cb48ec9de9eb893f",
          "message": "Add cargo-clippy as part of ci step",
          "timestamp": "2026-02-21T18:01:54Z",
          "tree_id": "cbf721cbaed06a757c9f581f6248ccf9c478388a",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/a7d8a748704384e5e167c868cb48ec9de9eb893f"
        },
        "date": 1771697429676,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 172962,
            "range": "± 83205",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 98722,
            "range": "± 68402",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 164469,
            "range": "± 78601",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 104488,
            "range": "± 71121",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23346,
            "range": "± 1595",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14366,
            "range": "± 572",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30501,
            "range": "± 649",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12205,
            "range": "± 397",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6554,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3310,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27428,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 7863,
            "range": "± 647",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 902295,
            "range": "± 39251",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 647852,
            "range": "± 27262",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1429635,
            "range": "± 87247",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 487317,
            "range": "± 62092",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 326081,
            "range": "± 159003",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 222719,
            "range": "± 139035",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 390295,
            "range": "± 155509",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 216284,
            "range": "± 134528",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5096472,
            "range": "± 35690",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1650598,
            "range": "± 61270",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11617,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 5070,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25977,
            "range": "± 1227",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5218,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1048233,
            "range": "± 33150",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 655610,
            "range": "± 26016",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 258159,
            "range": "± 5203",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 10974,
            "range": "± 428",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6419,
            "range": "± 378",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3650,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23856,
            "range": "± 1464",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 10040,
            "range": "± 468",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}