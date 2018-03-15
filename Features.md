# Quality Features for Parity with State of the Art Encoders

| Feature       |  rav1e        | libaom | x264 | x265 |
| ------------- |:-------------:|:-----:|:-----:|:---:|
| Exact RDO | Yes | No | Yes | Yes |
| Motion-adaptive quantizer (CRF) | Yes | No | Yes | Yes |
| Temporal RDO | Yes | No | Yes (mbtree) | Yes |
| Smart distortion | Yes | Broken (cdef-dist, daala-dist) | Yes (psyrd) | Yes |
| Automatic QM | Yes | No | No | Yes |
| Rational luma/chroma weight | Yes | No | Yes | Yes |
| B-pyramid | Yes | No | Yes | Yes |
|  Good rate control | Yes | No | Yes | Yes |

# Unique Quality Features beyond beyond State of the Art Encoders

| Feature       |  rav1e        | libaom | x264 | x265 |
| ------------- |:-------------:|:-----:|:-----:|:---:|
| Joint loop filter search | Yes | No | No | No |
| Chunk-compatible first pass | Yes | No | No | No (in UHDKit) |
| Dynamic programming mvs | Yes | No | No | No |
| Auto film grain | Yes | No | No | No |

# Speed Features

| Feature       |  rav1e        | libaom | x264 | x265 |
| ------------- |:-------------:|:-----:|:-----:|:---:|
| Pruning using approximate RDO | Yes | No | Yes | Yes |
| Frame-parallel encoding | Yes | No | Yes | Yes |
| ML trained pruning | Yes | No | No | No |
| Approximate transforms | Yes | Yes (by accident) | No | No |
| Model-based RDO cost | Yes | No (broken) | Yes | ? |


