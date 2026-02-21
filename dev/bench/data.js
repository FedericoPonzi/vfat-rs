window.BENCHMARK_DATA = {
  "lastUpdate": 1771709985556,
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
          "id": "db424083a32fda273c805cf32d282b44faf83aa1",
          "message": "add dependabot, update ci's action versions",
          "timestamp": "2026-02-21T18:04:08Z",
          "tree_id": "786be19fa13836e826bf84d47a6db444ed5d274a",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/db424083a32fda273c805cf32d282b44faf83aa1"
        },
        "date": 1771697557356,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 166592,
            "range": "± 82617",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 117504,
            "range": "± 69975",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 165832,
            "range": "± 78917",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 104085,
            "range": "± 68309",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23205,
            "range": "± 952",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 13675,
            "range": "± 1565",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30713,
            "range": "± 1191",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12661,
            "range": "± 335",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6576,
            "range": "± 95",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3313,
            "range": "± 110",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27550,
            "range": "± 1435",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8348,
            "range": "± 288",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 906824,
            "range": "± 18714",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 659881,
            "range": "± 18203",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1423280,
            "range": "± 84854",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 489605,
            "range": "± 65224",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 327105,
            "range": "± 156114",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 213891,
            "range": "± 130368",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 391410,
            "range": "± 151405",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 207166,
            "range": "± 125029",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5070610,
            "range": "± 67530",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1627420,
            "range": "± 67296",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11793,
            "range": "± 465",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 5505,
            "range": "± 193",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25834,
            "range": "± 1608",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5151,
            "range": "± 205",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1044432,
            "range": "± 28324",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 661233,
            "range": "± 41439",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 257579,
            "range": "± 3328",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 11372,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6417,
            "range": "± 311",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3519,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23810,
            "range": "± 995",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9593,
            "range": "± 987",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "FedericoPonzi@users.noreply.github.com",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f2ec34e58fe148aae3550b13d2f8dd2c06af7465",
          "message": "Merge pull request #15 from FedericoPonzi/dependabot/github_actions/actions-all-76468cb07f\n\nBump actions/checkout from 4 to 6 in the actions-all group",
          "timestamp": "2026-02-21T18:19:03Z",
          "tree_id": "1744009f2b452e9ecea5b22c3b874bfee9f41872",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/f2ec34e58fe148aae3550b13d2f8dd2c06af7465"
        },
        "date": 1771698452988,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 159673,
            "range": "± 79336",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 97614,
            "range": "± 64982",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 164163,
            "range": "± 78820",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 106434,
            "range": "± 71529",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23123,
            "range": "± 1781",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 13946,
            "range": "± 675",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30273,
            "range": "± 989",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12246,
            "range": "± 391",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6668,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3568,
            "range": "± 359",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27402,
            "range": "± 971",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8337,
            "range": "± 650",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 902177,
            "range": "± 37764",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 661072,
            "range": "± 61389",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1410807,
            "range": "± 81677",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 482531,
            "range": "± 56101",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 336518,
            "range": "± 154848",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 217907,
            "range": "± 132004",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 398964,
            "range": "± 154747",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 206939,
            "range": "± 124424",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5132688,
            "range": "± 73522",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1673263,
            "range": "± 81744",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11620,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4976,
            "range": "± 326",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25609,
            "range": "± 1358",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5202,
            "range": "± 473",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1047807,
            "range": "± 25381",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 657986,
            "range": "± 40521",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 259340,
            "range": "± 7162",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 10827,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6467,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3561,
            "range": "± 393",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23911,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9769,
            "range": "± 838",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "FedericoPonzi@users.noreply.github.com",
            "name": "Federico Ponzi",
            "username": "FedericoPonzi"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "87a6ed74f7934ae52417de38e29b218eff08e91f",
          "message": "Merge pull request #16 from FedericoPonzi/dependabot/cargo/cargo-all-daba6f9616\n\nBump the cargo-all group with 5 updates",
          "timestamp": "2026-02-21T18:19:24Z",
          "tree_id": "0c0d9efc31b50bf63688d75d7446d0369a9c4e18",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/87a6ed74f7934ae52417de38e29b218eff08e91f"
        },
        "date": 1771698476309,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 161434,
            "range": "± 80873",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 99854,
            "range": "± 66646",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 166954,
            "range": "± 80318",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 96874,
            "range": "± 67915",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23265,
            "range": "± 1872",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14098,
            "range": "± 1045",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 30471,
            "range": "± 1110",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 12499,
            "range": "± 747",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6658,
            "range": "± 470",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3377,
            "range": "± 234",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 27439,
            "range": "± 2398",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 7964,
            "range": "± 362",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 909113,
            "range": "± 30078",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 647576,
            "range": "± 19982",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1434785,
            "range": "± 81224",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 504426,
            "range": "± 57148",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 325781,
            "range": "± 146238",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 213894,
            "range": "± 129109",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 390867,
            "range": "± 140673",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 207889,
            "range": "± 135155",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5129474,
            "range": "± 107676",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1696285,
            "range": "± 53005",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11432,
            "range": "± 402",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4793,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25382,
            "range": "± 1020",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5170,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1045414,
            "range": "± 35678",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 653191,
            "range": "± 23458",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 258094,
            "range": "± 3449",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 12887,
            "range": "± 500",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6424,
            "range": "± 637",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3690,
            "range": "± 154",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23697,
            "range": "± 1900",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9894,
            "range": "± 195",
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
          "id": "ee2b9eb84b51ab52436824c4b08e1146a2c7d1a4",
          "message": "fix: reject file names exceeding 255-char LFN limit",
          "timestamp": "2026-02-21T20:19:30Z",
          "tree_id": "e23b264816f3d64f9bcd678470dd6fe7b4c06022",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/ee2b9eb84b51ab52436824c4b08e1146a2c7d1a4"
        },
        "date": 1771705685785,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 164191,
            "range": "± 80063",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 82996,
            "range": "± 42035",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 300880,
            "range": "± 151548",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 91697,
            "range": "± 51422",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23299,
            "range": "± 942",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14087,
            "range": "± 279",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 32886,
            "range": "± 554",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 13018,
            "range": "± 1166",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6571,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3310,
            "range": "± 206",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 32260,
            "range": "± 1361",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8646,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 907497,
            "range": "± 33284",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 659816,
            "range": "± 16986",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1457492,
            "range": "± 66533",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 593240,
            "range": "± 202333",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 340422,
            "range": "± 165473",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 204238,
            "range": "± 118713",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 410082,
            "range": "± 145484",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 203964,
            "range": "± 117968",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5097634,
            "range": "± 37517",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1675056,
            "range": "± 68009",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11473,
            "range": "± 572",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4774,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25613,
            "range": "± 514",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5136,
            "range": "± 633",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1040584,
            "range": "± 33730",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 644577,
            "range": "± 31572",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 258066,
            "range": "± 9673",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 10550,
            "range": "± 1052",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6456,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3720,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23873,
            "range": "± 764",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9764,
            "range": "± 621",
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
          "id": "8c6518c7544e70914395a21fc8c41bdeed797db7",
          "message": "fix: reject file names exceeding 255-char LFN limit",
          "timestamp": "2026-02-21T20:21:29Z",
          "tree_id": "1e3070a89d1d8d550be574969e5af794da762520",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/8c6518c7544e70914395a21fc8c41bdeed797db7"
        },
        "date": 1771705800345,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 166601,
            "range": "± 85398",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 84199,
            "range": "± 42960",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 302057,
            "range": "± 151584",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 82016,
            "range": "± 43129",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23135,
            "range": "± 963",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14021,
            "range": "± 620",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 32831,
            "range": "± 1551",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 13122,
            "range": "± 654",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6591,
            "range": "± 306",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3338,
            "range": "± 106",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 32266,
            "range": "± 2238",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8637,
            "range": "± 900",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 920190,
            "range": "± 31779",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 659713,
            "range": "± 12164",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1463852,
            "range": "± 91760",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 591123,
            "range": "± 202023",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 341614,
            "range": "± 150857",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 200058,
            "range": "± 109088",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 407059,
            "range": "± 146214",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 191488,
            "range": "± 95485",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5076119,
            "range": "± 56075",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1678411,
            "range": "± 49651",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11259,
            "range": "± 506",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4976,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25684,
            "range": "± 1460",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5360,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1049283,
            "range": "± 40495",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 654611,
            "range": "± 23527",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 259109,
            "range": "± 9239",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 10679,
            "range": "± 302",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6580,
            "range": "± 204",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3845,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 24308,
            "range": "± 567",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9798,
            "range": "± 262",
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
          "id": "03d2e6193dd245c00fc90f928da5ac0c21cbd1ed",
          "message": "feat: add File::truncate() with cluster chain freeing",
          "timestamp": "2026-02-21T21:12:52Z",
          "tree_id": "a795d5c4ee2fca4bd3999c77820a2a0df8567a27",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/03d2e6193dd245c00fc90f928da5ac0c21cbd1ed"
        },
        "date": 1771708892456,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 162829,
            "range": "± 84083",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 80947,
            "range": "± 39858",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 170923,
            "range": "± 80552",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 86496,
            "range": "± 43034",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 23183,
            "range": "± 1233",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14150,
            "range": "± 787",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 32958,
            "range": "± 2198",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 13278,
            "range": "± 919",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6530,
            "range": "± 398",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3350,
            "range": "± 144",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 32508,
            "range": "± 1968",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 9326,
            "range": "± 460",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 902490,
            "range": "± 10210",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 643106,
            "range": "± 28176",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1441719,
            "range": "± 88535",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 609366,
            "range": "± 204280",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 334889,
            "range": "± 153677",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 210991,
            "range": "± 107552",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 395722,
            "range": "± 151955",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 199143,
            "range": "± 111610",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5088479,
            "range": "± 60604",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1651525,
            "range": "± 71092",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11045,
            "range": "± 653",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 4760,
            "range": "± 421",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 25528,
            "range": "± 951",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5146,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1041821,
            "range": "± 30924",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 657560,
            "range": "± 39742",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 259315,
            "range": "± 8088",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 11418,
            "range": "± 460",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6467,
            "range": "± 375",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 3795,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 23844,
            "range": "± 1237",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 9523,
            "range": "± 808",
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
          "id": "074e82ccddb05211d0a71e05cba40bfb2550bde5",
          "message": "feat: add File::truncate() with cluster chain freeing",
          "timestamp": "2026-02-21T21:31:10Z",
          "tree_id": "5310756094bcb48b2f7420c725d7196a643a736a",
          "url": "https://github.com/FedericoPonzi/vfat-rs/commit/074e82ccddb05211d0a71e05cba40bfb2550bde5"
        },
        "date": 1771709985090,
        "tool": "cargo",
        "benches": [
          {
            "name": "dir_create_file/uncached",
            "value": 161833,
            "range": "± 81661",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_file/cached",
            "value": 82609,
            "range": "± 43479",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/uncached",
            "value": 173265,
            "range": "± 80753",
            "unit": "ns/iter"
          },
          {
            "name": "dir_create_directory/cached",
            "value": 82768,
            "range": "± 46020",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/uncached",
            "value": 22866,
            "range": "± 1575",
            "unit": "ns/iter"
          },
          {
            "name": "dir_list_contents/cached",
            "value": 14005,
            "range": "± 1158",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/uncached",
            "value": 32683,
            "range": "± 657",
            "unit": "ns/iter"
          },
          {
            "name": "dir_delete_file/cached",
            "value": 13111,
            "range": "± 1120",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/uncached",
            "value": 6530,
            "range": "± 439",
            "unit": "ns/iter"
          },
          {
            "name": "dir_contains/cached",
            "value": 3290,
            "range": "± 174",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/uncached",
            "value": 32021,
            "range": "± 1048",
            "unit": "ns/iter"
          },
          {
            "name": "dir_rename/cached",
            "value": 8642,
            "range": "± 757",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/uncached",
            "value": 908214,
            "range": "± 41466",
            "unit": "ns/iter"
          },
          {
            "name": "fat_chain_traversal/cached",
            "value": 648711,
            "range": "± 21953",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/uncached",
            "value": 1467539,
            "range": "± 96411",
            "unit": "ns/iter"
          },
          {
            "name": "cluster_allocation/cached",
            "value": 590248,
            "range": "± 199011",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/small_16B",
            "value": 347191,
            "range": "± 163492",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/small_16B",
            "value": 214885,
            "range": "± 115915",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/medium_4KB",
            "value": 400912,
            "range": "± 158474",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/medium_4KB",
            "value": 212749,
            "range": "± 105552",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/uncached/large_256KB",
            "value": 5158670,
            "range": "± 1510845",
            "unit": "ns/iter"
          },
          {
            "name": "file_write/cached/large_256KB",
            "value": 1696111,
            "range": "± 72842",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/small_16B",
            "value": 11694,
            "range": "± 610",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/small_16B",
            "value": 5301,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/medium_4KB",
            "value": 26243,
            "range": "± 1114",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/medium_4KB",
            "value": 5640,
            "range": "± 377",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/uncached/large_256KB",
            "value": 1053626,
            "range": "± 28966",
            "unit": "ns/iter"
          },
          {
            "name": "file_read/cached/large_256KB",
            "value": 649270,
            "range": "± 18278",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/uncached",
            "value": 260819,
            "range": "± 8689",
            "unit": "ns/iter"
          },
          {
            "name": "file_seek/cached",
            "value": 11382,
            "range": "± 548",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/uncached",
            "value": 6986,
            "range": "± 407",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_shallow/cached",
            "value": 4340,
            "range": "± 142",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/uncached",
            "value": 25471,
            "range": "± 2525",
            "unit": "ns/iter"
          },
          {
            "name": "path_traversal_deep/cached",
            "value": 11444,
            "range": "± 473",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}