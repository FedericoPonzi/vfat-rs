window.BENCHMARK_DATA = {
  "lastUpdate": 1771591845860,
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
      }
    ]
  }
}