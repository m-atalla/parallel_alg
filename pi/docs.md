## Programming Model
Manual parallelization using thread pool (_not really needed for this problem_) and a multiple producer singler consumer (mpsc) channel for collecting results.

#### Thread operation:
- Generate **N** random points drawn from a uniform distributions
- For each point determine if the point is inside the circle or not
- Filter all points that are outside
- Count the remaining points inside
- Send the result to the main thread through the channel

## Partitioning
**Domain decomposition**: the data points are divided evenly for each thread.

## Communication

Collective communication: scatter and gather operation done by the main thread. 

#### Main thread communication sequence:
- Send 

No need for inter-thread communication as there is no dependancy between the seperate data
partitions

## Synchronization

- **Lock / Semaphore**: internally the thread pool send a mutex of a point to a job (a function
pointer) the first thread to acquire the lock gets to execute the job
- **Synchronous communication operations**: through the mpsc channel discussed earlier to
scatter/reduce the data/results.


## Data Dependancies
No data depedancies as each point does not require any other point to be generated.

## Partitioning
**Equally partitioned** work for each task sent.

## Granularity
**Coarse grained** the communication part is small, as it is only sending the integer result of
the total points inside the circle through the channel. 
While the majority of the computation is done in the thread without any extra need for communication during the computation.

## I/O
Not really a bottleneck in this problem as I/O is only used to display the final output.

## Performance Analysis
Performance analysis was done using **Perf** linux profiler.


