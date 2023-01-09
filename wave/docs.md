## Programming Model
Manual parallelization using thread pool and a multiple producer single consumer (mpsc) channel for collecting results.

#### Thread operation:
- Calculate the given point using the provided formula in the problem statement
- Send the result to the main thread through the mpsc channel

## Partitioning
**Domain decomposition**: the data points are divided evenly for each thread.

## Communication
Collective communication: scatter and gather operation done by the main thread. 

#### Main thread communication sequence:
- Send points to be calculated to the thread pool
- Collect the result
- Add the result to the current wave array
- Add the new wave to the "waves timeline" array for next waves calculations

No need for inter-thread communication as there is no dependency between the separate data
partitions.

## Synchronization
- **Lock / Semaphore**: internally the thread pool send a Mutex (mutual exclusion lock) of a point to a job (a function
pointer) the first thread to acquire the lock gets to execute the job
- **Synchronous communication operations**: through the mpsc channel discussed earlier to
scatter/reduce the data/results.

## Data Dependencies
There are dependencies between waves as calculated a point in the current wave (t) requires the
equivalent point in the previous two waves (t - 1 and t - 2) and so we cannot parallelize
waves. However, we can parallelize a single wave points calculation.

## Partitioning
**Equally partitioned** individual points are calculated by threads for a given wave.

## Granularity
**Coarse grained** the communication part is small, as it is only sending the integer result of
the total points inside the circle through the channel. 
While the majority of the computation is done in the thread without any extra need for communication during the computation.

## I/O
Not really a bottleneck in this problem as I/O is only used to display the final output.

## Performance Analysis
Done using **Perf** Linux profiler.

