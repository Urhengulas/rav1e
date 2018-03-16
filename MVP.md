Tracked in https://github.com/xiph/rav1e/projects/5

# Notes below will be moved into cards in the project tracking page.

### Minimum Viable Product (6 months)
  * Focus on offline streaming (Usable by Wikipedia)
  * Similar to libvpx VP9 visually
  * Similar to libvpx compute @ parameters wikipedia uses
  * 8 and 10 bit video 4:2:0 only (profile 0)
  * What type of encoding does Wikipedia use (CBR, VBR....)?
  * Must output compatible bitstreams
  * Conformance test suite checked against libaom (Jenkins?)
  * No disabled experiments in README
  * Must support padding
  * Speed levels (integers) 0 to 10 (low to high complexity)
  * Must support all partitions (use speed levels for this) Do we want 128x128 from the start? Do this include 4:1 and 1:4 partitions?
  * Rate-distortion debugging and dumping
  * Must have fast rate distortion estimation (with Luma/Chroma weighting ?)
  * Coefficient coding (lv_map)
  * Must support inter frames
  * Must support CDEF
  * Must support Motion Vector Prediction
  * Must have a fast motion search (basic hexagon search, SAD first, half-pel a must)
  * Must support deblocking
  * Must support all intra predictors
  * GOP-level rate control, below GOP plain boosts (Open or Close GOP?)
  * Reasonable software engineering
  * Code coverage (any % target)?
  Automation?
    * DailyPer-commit metrics (tracking regressions)? Nightly Test (System tests)?
  * Documentation, e.g. contributor guide (not API users)
  * In-tree docs with table of contents
  * Logo, name, website (Find someone to do this)
  * Must support pyramid B
  * Must support compound prediction (2 reference frames)
  * Entropy Coding (Is the EC in Rav1e good enough for MVP, do we need to add anything?)
  * Do we support all transform types in MVP? VAR_TX? (Wrappers to libaom transforms)
  * Quantization (Is the quantization in Rav1e good enough for MVP, do we need to add anything?)
  * Does the MPV require OBU support?
  * Keep dependencies to a minimum (evaluate after MVP)
    * Up and to the left after every change.