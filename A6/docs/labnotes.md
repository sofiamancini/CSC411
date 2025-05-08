
| Benchmark                                | Time        | Instructions | Rel to start | Rel to prev | Improvement |
|------------------------------------------|------------:|-------------:|-------------:|------------:|------------:|
| **Original (no LTO)**                    |             |              |             |            |            |
| • small (midmark.um)                     | 2.656 s     | 2,146        | 1.000        | —           | —          |
| • big   (sandmark.umz)                   | 64.195 s    | —            | 1.000        | —           | —          |
| **+ LTO**                                |             |              |             |            |            |
| • small                                 | 2.107 s     | 2,146        | 0.794        | 0.794       | 20.6%      |
| • big                                   | 55.591 s    | —            | 0.866        | 0.866       | 13.4%      |
| **Vec segments (no HashMap)**            |             |              |             |            |            |
| • small                                 | 0.487 s     | 329          | 0.183        | 0.231       | 76.9%      |
| • big                                   | 9.221 s      | —            | —            | —           | —          |
| **Segment-pool reuse**                   |             |              |             |            |            |
| • small                                 | 0.449 s     | 329          | 0.169        | 0.922       | 7.8%       |
| • big                                   |7.981 s      | —            | —            | —           | —          |
| **`#[inline(never)]` on handlers**       |             |              |             |            |            |
| • small                                 | 0.312 s     | 322          | 0.118        | 0.695       | 30.5%      |
| • big                                   | 7.889 s     | —            | 0.123        | 0.142       | 85.8%      |


1. Original Code Benchmarks

1a. Without lto:

Benchmark 1: target/profiling/rum umbin/midmark.um
  Time (mean ± σ):      2.656 s ±  0.071 s    [User: 2.103 s, System: 0.123 s]
  Range (min … max):    2.548 s …  2.772 s    10 runs
 
Benchmark 1: target/profiling/rum umbin/sandmark.umz
  Time (mean ± σ):     64.195 s ±  3.450 s    [User: 51.303 s, System: 3.123 s]
  Range (min … max):   60.781 s … 69.313 s    10 runs
 

1b. With lto:

Benchmark 1: target/profiling/rum umbin/midmark.um
  Time (mean ± σ):      2.107 s ±  0.039 s    [User: 1.710 s, System: 0.113 s]
  Range (min … max):    2.042 s …  2.173 s    10 runs
 
Benchmark 1: target/profiling/rum umbin/sandmark.umz
  Time (mean ± σ):     55.591 s ±  2.191 s    [User: 43.105 s, System: 2.699 s]
  Range (min … max):   53.361 s … 60.732 s    10 runs
 
Original Code Profile (midmark.um)

Samply report:
    
    rum::vm::VirtualMachine::run
    rum

    Call node details
    Running samples
    100%
    2,364
    Self samples
    91%
    2,146
    Categories
    Running sample count
    User
    100%
    2,364
    Categories
    Self sample count
    User
    100%
    2,146


3. The second-most expansive operation (after run) the hash table insertions:

hashbrown::map::HashMap::insert
rum

Call node details
Running samples
3%
70
Self samples
3%
69
Categories
Running sample count
User
100%
70
Categories
Self sample count
User
100%
69

In the original implementation, each 'Map Segment' instruction performed a HashMap::insert. That hashing overhead alone showed up as ~3% of our total samples and had a significant impact on performance.

I replaced the `HashMap<SegmentID, Vec<Word>>` with a `Vec<Option<Vec<Word>>>`, using direct indexing on simple integer IDs and a free_ids stack for reusing slots. This makes both allocation and lookup pure O(1) array accesses, with no hashes or rehashing. It required only a small VM-struct refactor (storing segments in a Vec and bumping an index for new IDs).

Benchmark 1: target/profiling/rum umbin/midmark.um
  Time (mean ± σ):     486.7 ms ±  72.2 ms    [User: 350.1 ms, System: 17.6 ms]
  Range (min … max):   408.7 ms … 665.6 ms    10 runs

Benchmark 1: target/profiling/rum umbin/sandmark.umz
  Time (mean ± σ):      9.221 s ±  0.232 s    [User: 9.033 s, System: 0.045 s]
  Range (min … max):    9.002 s …  9.757 s    10 runs

Results:

midmark.um went from
Time (mean): 2.107 s

to

Time (mean): 0.487 ms

- over 4x speed improvement.

New Samply report:

rum::vm::VirtualMachine::run
rum

Call node details
Running samples
100%
439
Self samples
75%
329
Categories
Running sample count
User
100%
439
Categories
Self sample count
User
100%
329

4. After making those optimizations the next most expensive call according to Samply is a call to malloc:

tiny_free_no_lock
libsystem_malloc.dylib

Call node details
Running samples
7%
29
Self samples
5%
22
Categories
Running sample count
User
100%
29
Categories
Self sample count
User
100%
22


This is showing that there are frequent accesses to memory allocation and deallocation, likely heap allocations, which are computationally costly. After looking at the flamegraph and examing my code, I believe most of these calls are coming from the 'load' opcode, specifically this line 'let new0 = seg.clone();'. The repeated cloning of segments for each instance of load is accessing heap memory repeatedly. 

Original load program:
                12 => { // Load Program
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    if self.registers[b] != 0 {
                        if let Some(seg) = &self.segments[self.registers[b] as usize] {
                            let new0 = seg.clone();
                            self.segments[0] = Some(new0);
                        } else {
                            panic!("Invalid segment");
                        }
                    }
                    self.pc = self.registers[c] as usize;
                }

After optimization: 

                12 => { // Load Program
                    let b = ((instruction >> 3) & 0x7) as usize;
                    let c = (instruction & 0x7) as usize;
                    if self.registers[b] != 0 {
                        if let Some(seg) = &self.segments[self.registers[b] as usize] {
                            let mut reused = self.segment_pool.pop().unwrap_or_else(|| Vec::with_capacity(seg.len()));
                            reused.clear(); // ensure it's empty
                            reused.extend_from_slice(seg); // copy data
                            self.segments[0] = Some(reused);
                            
                        } else {
                            panic!("Invalid segment");
                        }
                    }
                    self.pc = self.registers[c] as usize;
                }

To avoid repeated heap allocations I added a segment pool to the Virtual Machine struct. This pool is a Vec of previously use memory chunks that can now be reused, where before they would have been thrown away. Now the code will reuse a memory segment if available and copies data into it, avoiding any system calls to malloc.

Benchmark 1: target/profiling/rum umbin/midmark.um
  Time (mean ± σ):     449.4 ms ±  33.0 ms    [User: 339.7 ms, System: 23.8 ms]
  Range (min … max):   405.9 ms … 495.9 ms    10 runs

  
Benchmark 1: target/profiling/rum umbin/sandmark.umz
  Time (mean ± σ):      7.981 s ±  0.227 s    [User: 7.818 s, System: 0.040 s]
  Range (min … max):    7.774 s …  8.421 s    10 runs

 
midmark.um went from
Time (mean): 0.487 ms

to

Time (mean): 0.449 ms


5. A happy accident

Since the main body of my code is in vm.rs, I added `#[inline(never)]` in front of each of the functions with the hopes of getting a more detailed report from Samply. After making this change I chose to run the benchmark again with hyperfine to make sure nothing was broken and discovered a significant improvment in the run times.
After some Googling I realized the reason for this improvement is due to having fewer cache misses by separating the main body loop. Since the map, unmap, load, and run functions no longer exist inside the main loop, they are only paged when needed, improving the CPU prefetching and reducing cache overflows.

Benchmark 1: target/profiling/rum umbin/midmark.um
  Time (mean ± σ):     311.9 ms ±   5.2 ms    [User: 306.2 ms, System: 3.1 ms]
  Range (min … max):   305.3 ms … 322.4 ms    10 runs

Benchmark 1: target/profiling/rum umbin/sandmark.umz
  Time (mean ± σ):      7.889 s ±  0.123 s    [User: 7.792 s, System: 0.038 s]
  Range (min … max):    7.766 s …  8.186 s    10 runs


midmark.um went from
Time (mean): 0.449 ms

to

Time (mean): 0.312 ms


to

Time (mean): 0.321 ms

or about a 30% improvement in speed.