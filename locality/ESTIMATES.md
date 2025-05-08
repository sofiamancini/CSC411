# Cache Hit Rate and Performance Predictions

## Cache Hit Rankings

|                  | Row-Major Access | Column-Major Access  |
| ---------------- |:---------------: | --------------------:|
| 90-deg Rotation  | Rank 3           | Rank 4               |
| 180-deg Rotation | Rank 1           | Rank 2               |

## Justification for Cache Hit Rate Rankings

- ### 180-degree Rotation (row-major access):
    - in a 180-degree rotation, each pixel is mapped to a new position symmetrically across the center of the image. When using row-major access, the source and destination pixels are likely in the same cache line, leading to good spatial locality, since the assumption is that the images are too large to fit in the cache, spatial locality is extremely important for a high hit rate because temporal locality cannot be used. This should result in a high cache hit rate.

- ### 180-degree Rotation (column-major access):
    - Since the access pattern is now column-major, the spatial locality will be slightly worse than the row-major iterator since my device uses cache lines stored in row-major order. Since it is still a 180-degree rotation, there will still be symmetry across the center line so there should still be a relatively high cache hit rate.

- ### 90-degree Rotation (row-major access):
    - The 90-degree rotation requires mapping each pixel from (x, y) to (y, width - x -1). This additional math does not align with the row-major access pattern and will likely result in more misses. Since the destination pixels are less likely to be in the same cache line as the source pixels, the spatial locality will be worse than the 180-degree rotations. Due to this I expect the hit rate to be poor.

- ### 90-degree Rotation (column-major access):
    - I expect this rotation to have the worst cache hit rate because it has misses that arise from both the poor spatial locality of colum-major access and the non-symmetric rotation of 90-degrees.

## Performance Predictions

|                  | Row-Major Access | Column-Major Access  |
| ---------------- |:---------------: | --------------------:|
| 90-deg Rotation  | Rank 3           | Rank 4               |
| 180-deg Rotation | Rank 1           | Rank 2               |

## Justifications for Performace Rankings

- ### 180-degree Rotation (row-major access):
    - Highest cache use efficiency
    - Low computational overhead (fewest arthimetic operations)

- ### 180-degree Rotation (column-major acess):
    - Second-best cache hit rate
    - Low computational overhead (fewest arthimetic operations)

- ### 90-degree Rotation (row-major access):
    - Relatively poor spatial locality from mapping coordinates outside of row-major order
    - The mapping requires more arthimetic operations (compared to 180-degree rotations)

- ### 90-degree Rotation (column-major access):
    - Worst cache hit rate
    - Similar amount of arthimetic operations as the row-major access

## Cost Estimates per Pixel

|  op  |  + - | muls | divs/mods | comps | loads | hit rate (l) | stores | hit rate (s) |
| ---- |:----:|:----:|:---------:|:-----:|:-----:|:------------:|:------:|:------------:|
| 180R |   2  |  0   |     0     |   2   |   1   |     ~80%     |    1   |     ~80%     |
| 180C |   2  |  0   |     0     |   2   |   1   |     ~70%     |    1   |     ~70%     |
| 90R  |   3  |  1   |     0     |   2   |   1   |     ~50%     |    1   |     ~50%     |
| 90C  |   3  |  1   |     0     |   2   |   1   |     ~40%     |    1   |     ~40%     |

## Speed Rankings

|                  | Row-Major Access | Column-Major Access  |
| ---------------- |:---------------: | --------------------:|
| 90-deg Rotation  | Rank 3           | Rank 4               |
| 180-deg Rotation | Rank 1           | Rank 2               |

## Justifications for Speed Rankings

- ### 180-degree Rotation (row-major access):
    - Cache Performance: High cache hit rate. Consecutive memory accesses are to adjacent elements.
    - Computational Cost: Few arthimetic operations.
    - Loads/Stores: Each pixel has one load and one store, with a high percentage of cache hits for both.
    - Overall Speed: Fastest due to efficient cache utilization and low computational overhead.

- ### 180-degree Rotation (column-major acess):
    - Cache Performance: Fairly high cache hit rate. Some spatial locality is lost from column-major access.
    - Computational Cost: Few arthimetic operations.
    - Loads/Stores: Each pixel has one load and one store, with a slighlty worse percentage of cache hits for both.
    - Overall Speed: Slightly slower than row-major due to inefficient cache utilization and low computational overhead.

- ### 90-degree Rotation (row-major access):
    - Cache Performance: Low cache hit rate. Some spatial locality is lost due to location of source and destination pixels.
    - Computational Cost: More complex coordinate mapping.
    - Loads/Stores: Each pixel has one load and one store, with a significantly worse percentage of cache hits for both.
    - Overall Speed: Slower than 180-degree rotations due to inefficient cache utilization and high computational overhead.

- ### 90-degree Rotation (column-major access):
    - Cache Performance: Very low cache hit rate. Spatial locality is lost due to location of source and destination pixels as well as access method inefficiency. 
    - Computational Cost: More complex coordinate mapping.
    - Loads/Stores: Each pixel has one load and one store, with a significantly worse percentage of cache hits for both.
    - Overall Speed: Slowest due to inefficient cache utilization and high computational overhead.

## Summary Speed Rankings

| Operation | Cache Hit Rate | Computational Cost  |   Loads/Stores  | Speed Rank |
| --------- |:-------------: | :------------------:|:---------------:|-----------:|
|    180R   |      High      |         Low         |     Efficient   |      1     |
|    180C   |     Moderate   |         Low         |  Less Efficient |      2     |
|    90R    |       Low      |       Moderate      |  Less Efficient |      3     |
|    90C    |     Very Low   |       Moderate      | Least Efficient |      4     |