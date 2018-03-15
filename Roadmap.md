# AV1 Encoder Roadmap

Things we need to do to make a good encoder:

Write syntax elements & reconstruct
- Write partitions
- Write ZeroMV / skipmv
- Write fullpel MV
- Write subpel MV
- Write variable sized transforms

RDO
- Breadth-first partition split search
  + Transform size search
    * Transform type search
- Simple hex motion search
  + SAD
  + SATD
  + Model-based R-D
- Luma/Chroma weighting
- Alternate (8x8) distortion functions
- Quant matrices
- Activity masking (spatial_segmentation)
- Quantization/tokenization
  + Trellis optimization
  + Greedy optimization
  + Faster rounding/thresholding strategies
- Loop filters
  + Deblocking
    * Approximate during partition search
  + CDEF
  + Joint parameter optimization
  + Loop Restoration
- Inter modes
  + Compound mode
  + Inter-inter compound segments
  + Inter-intra
  + Warped motion
  + OBMC
  + Wedge modes
  + Global motion
+ Non-causal (iterated/dynamic programming) MV/mode search

Add assembly support
- Port libaom stuffs

RDO speed optimizations
Rate estimation for coefficients (in decreasing order of complexity)
- Estimate rate of coefficients after quantization
- Estimate distortion from residual + quantizer
- Estimate rate of coefficients before quantization
- Estimate rate of coefficients pre inverse transform
- Estimate rate of coefficients from residual

Overall search
- Least accurate search -> list of best possibilities -> slower -> shorter list -> slowest -> pick 1
- Decide size of list item (superblock, block, etc)

Rate control
- Constant quantizer
- Constant bitrate (over a window)
- Real-time/interactive (sub-frame control loop)
- Two-pass
  + Fast first pass
- Look-ahead
- Temporal RDO (MB-Tree)
- Scene change detection
- Frame type decision
  + Pyramid reference structure

=================================================

What do we need to have done to start development:
 - catch up on AV1 bitstream
 - run rav1e on AWCY with normal video sets
 - have a repository URL (gitlab, github?) - Make decision by end of the month, GitHub or GitLab, TD will figure out the exact date.
   - how do we accept anonymous patches
   - look at how GNOME is using gitlab and do that
   - fix github logins
 - decide on auto formatting, style guide, naming conventions, etc.

 ---------------------------------------------------------------------------------------------------------------------------

Minimum Viable Product (6 months)
  - Focus on offline streaming (Usable by Wikipedia)
  - Similar to libvpx VP9 visually
  - Similar to libvpx compute @ parameters wikipedia uses
  - 8 and 10 bit video 4:2:0 only (profile 0)
  - What type of encoding does Wikipedia use (CBR, VBR....)?


  - Must output compatible bitstreams
  - Conformance test suite checked against libaom (Jenkins?)
  - No disabled experiments in README
  - Must support padding
  - Speed levels (integers) 0 to 10 (low to high complexity)
  - Must support all partitions (use speed levels for this) Do we want 128x128 from the start? Do this include 4:1 and 1:4 partitions?
  - Rate-distortion debugging and dumping
  - Must have fast rate distortion estimation (with Luma/Chroma weighting ?)
  - Coefficient coding (lv_map)
  - Must support inter frames
  - Must support CDEF
  - Must support Motion Vector Prediction
  - Must have a fast motion search (basic hexagon search, SAD first, half-pel a must)
  - Must support deblocking
  - Must support all intra predictors
  - GOP-level rate control, below GOP plain boosts (Open or Close GOP?)
  - Reasonable software engineering
  - Code coverage (any % target)?
  Automation?
    - DailyPer-commit metrics (tracking regressions)? Nightly Test (System tests)?
  - Documentation, e.g. contributor guide (not API users)
  - In-tree docs with table of contents
  - Logo, name, website (Find someone to do this)
  - Must support pyramid B
  - Must support compound prediction (2 reference frames)
  - Entropy Coding (Is the EC in Rav1e good enough for MVP, do we need to add anything?)
  - Do we support all transform types in MVP? VAR_TX? (Wrappers to libaom transforms)
  - Quantization (Is the quantization in Rav1e good enough for MVP, do we need to add anything?)
  - Does the MPV require OBU support?




  - Keep dependencies to a minimum (evaluate after MVP)
    - Up and to the left after every change.

TD-Linux 
Update rav1e to work with unmodified latest libaom
set up gitlab (maybe)
 - set up gitlab ci

xiphmont
lv_map

codeview
partitions

lu_zero
speed

ltrudeau
Add IntraOnly support in Uncompressed Header
Support inter frames

tmatth
partitions

unlord
???

derf
???

barrbrain
cfl

=== RDO Checkpoint Fix ===

Currently, RDO checkpoints make a copy of most context. When the checkpoint is restored, all that context is copied back. This is slow. One idea is for all Writer functions to log what context they touched, and only restore that. However, this still means the copy into the checkpoint has to be everything. That can be made smaller by knowing in advance what context will be touched when creating the checkpoint. However there's another fix:
    
1. Make sure all context is only written by Writer. E.g. partition context must be fixed.
2. Remove checkpoints, and instead add a ContextWriterTransaction (?)
3. cwt = cw.create_transaction();
4. cwt contains a mutable reference to cw so cw can't be used until cwt is dropped
5. all Writer functions are implemented by ContextWriterTransaction. It copies state into itself only when actually mutated.
6. cwt.commit() copies all state back into the cw. Only call this once you want to commit to a mode.
7. cwt can itself create a transaction, e.g. cwt.create_transaction(). This can go arbitrarily deep.