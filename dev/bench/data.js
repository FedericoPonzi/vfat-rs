window.BENCHMARK_DATA = {
  "lastUpdate": 1771578424873,
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
      }
    ]
  }
}