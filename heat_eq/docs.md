## Programming Model
Manual parallelization using thread pool and 
a multiple producer single consumer (mpsc) channel for collecting results.

#### Thread operation:
Can be summarized in the following [!figure](./par_solution.png)

## Partitioning
**Domain decomposition**: each task is a row of the grid to be calculated.

## Communication
Collective communication: scatter and gather operation done by the main thread. 

#### Main thread communication sequence:
- Make an **atomic** reference counter 
(Arc: atomic reference counters are used to safely share pointers between threads) 
to the previous grid
- Send a row of the grid to the thread pool to be executed
- Collect the results and update the new grid
- Add the new grid to the "grid timeline" array for reuse in the next iteration

No need for inter-thread communication as there is no dependency between the separate data
partitions. The previous grid is duplicated and so no need for communication as we broke the dependencies with adjacent cells.

## Synchronization
- **Lock / Semaphore**: internally the thread pool send a Mutex (mutual exclusion lock) of a point to a job (a function
pointer) the first thread to acquire the lock gets to execute the job
- **Synchronous communication operations**: through the mpsc channel discussed earlier to
scatter/reduce the data/results.

## Data Dependencies
- The problem implies data dependencies with adjacent cells. However, we can break this
dependency if we keep a copy of the previous state. So the solution here duplicates the
space to avoid communication and synchronization overhead.

- There is also a hard dependency that we cannot get around, that each "step" of the heat time is dependent on the previous step result.

## Partitioning
**Equally partitioned** individual rows are calculated by threads for a given grid.

## Granularity
**Coarse grained** the communication part is small, as it is only sending the result of the new grid _row_ to the main thread.
While the majority of the computation is done in the thread without any extra need for communication during the computation.

## I/O
Not really a bottleneck in this problem as I/O is only used to display the final output.

## Performance Analysis
Done using **Perf** Linux profiler.
