window.BENCHMARK_DATA = {
  "lastUpdate": 1771582058530,
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
      }
    ]
  }
}