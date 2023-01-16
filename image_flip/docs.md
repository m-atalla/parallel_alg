# Image to Grayscale

## Programming Model
Manual parallelization using thread pool implementation.

#### Thread operation:
- Convert pixel rgba to luma alpha (grayscale with alpha channel)
- Push the converted pixel to an array
- Send the resulting array to channel where the main threads collects it and rebuild the image.

## Partitioning
**Domain decomposition**: the image pixels are divided evenly for each thread.

## Communication
Collective communication: scatter and gather operation done by the main thread. 

#### Main thread communication sequence:
- Scatter the subset (chunk) of pixels as jobs sent to the thread
pool to be executed.
- Gather the resulting grayscale pixel arrays from the threads

No need for inter-thread communication as there is no dependency between the separate data
partitions

## Synchronization

- **Lock / Semaphore**: internally the thread pool send a mutex of a point to a job (a function
pointer) the first thread to acquire the lock gets to execute the job
- **Synchronous communication operations**: through the mpsc channel discussed earlier to
scatter/reduce the data/results.


## Data Dependencies
No data dependencies as each pixel does not require any other pixel to be converted to gray
scale.

## Partitioning
**Equally partitioned** work for each task sent.

## Granularity
Coarse grained: there are no dependencies between pixels and so the communication time is
minimized to sending the final results.

## I/O
I/O is bottleneck here for the main thread as we need to read the image from disk and finally
save the resulting grayscale image to disk.

## Performance Analysis
Done using **Perf** Linux profiler.
